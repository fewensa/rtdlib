use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use curl::easy::{Easy, ProxyType};
use scraper::{Html, Selector};
use tera::{Context, Tera};

use crate::bog;
use crate::ctgo::{filter, parser};
use crate::ctgo::apipe::Apipe;

const TG_API_DOC_BASE_URL: &'static str = "https://core.telegram.org/tdlib/docs/";


pub fn build<P: AsRef<Path>>(save_path: P) {
  handle_main("td__api_8h.html", save_path);
  bog::info("Generate complete.");
}

fn handle_main<P: AsRef<Path>>(main_page: &'static str, save_path: P) {
  let save_path = save_path.as_ref();
  self::download_page(main_page, save_path, "_index.html");
  let main_body = fs::read_to_string(save_path.join("_index.html")).expect("Not found telegram api index.html");

  let doc = Html::parse_fragment(&main_body[..]);
  let selector_all_class = Selector::parse("tr.memitem\\:").unwrap();
  let selector_name = Selector::parse(".el").unwrap();

  let classes = doc.select(&selector_all_class);

  let mut czs = Vec::new();
  let mut count = 0;
  classes.for_each(|clz| {
    let cl_names = clz.select(&selector_name).next().expect(&format!("Not found class name")[..]);
    let ele_cname = cl_names.value();

    let cname = match cl_names.text().next() {
      Some(cname) => cname.trim(),
      None => return
    };

    // skip some class
    if is_skip(cname) {
      return;
    }

    let href = ele_cname.attr("href");
    if let None = href {
      return;
    }
    let href = href.unwrap();
    let path = self::download_page(href, save_path, &format!("{}.html", cname)[..]);
    czs.push((cname.trim().to_string(), path));
    count += 1;
  });

  bog::info(format!("CLASS COUNT: {}", count));

  self::gen_rs2(czs);
}

fn download_page<S: AsRef<str>, P: AsRef<Path>>(url: S, save_path: P, save_name: S) -> PathBuf {
  let path = save_path.as_ref();
  let save_name = save_name.as_ref();
  let url = &format!("{}{}", TG_API_DOC_BASE_URL, url.as_ref())[..];

  if !path.exists() {
    fs::create_dir_all(path).expect("Can not create save path.");
  }
  let fpath = path.join(save_name);
  if fpath.exists() {
    bog::debug(format!("exists {:?} file, use cache.", fpath.canonicalize().expect("Can not get tdapi save path")));
    return fpath;
  }

  bog::debug(format!("Download {} ==> {:?}", url, fpath.canonicalize().expect("Can not get tdapi save path")));

  let mut easy = Easy::new();
  let hostname: Option<String> = hostname::get_hostname();
  if let Some(name) = hostname {
    if "amip" == &name[..] {
      easy.proxy("127.0.0.1").expect("Proxy host fail.");
      easy.proxy_port(1080).expect("Proxy port fail.");
      easy.proxy_type(ProxyType::Socks5).expect("Proxy type fail.");
    }
  }
  easy.url(url).expect("Url fail.");

  let mut all = Vec::<u8>::new();
  {
    let mut easy = easy.transfer();
    easy.write_function(|data| {
      all.extend(data);
      Ok(data.len())
    }).unwrap();
    easy.perform().expect("Send request fail.");
  }


  let content = String::from_utf8(all).expect("Can not get response body");
  fs::write(&fpath, content).expect(&format!("Can not write conten to => {:?}", fpath.canonicalize().expect("Not found save path"))[..]);
  return fpath;

//  println!("===> {}", easy.response_code().unwrap());
}


fn gen_rs2(czs: Vec<(String, PathBuf)>) {

  let write_to_types = Path::new("src")
    .canonicalize()
    .expect("Can not get rawtd path")
    .join("types.rs");
  let write_to_supplement = Path::new("src")
    .canonicalize()
    .expect("Can not get rawtd path")
    .join("tdsupplement.rs");
  let write_to_schema = Path::new("schema")
    .canonicalize()
    .expect("Can not get rawtd path")
    .join("schema.toml");
  if write_to_types.exists() {
    fs::remove_file(&write_to_types).expect(&format!("Can not remove {:?} file", write_to_types)[..]);
  }
  if write_to_supplement.exists() {
    fs::remove_file(&write_to_supplement).expect(&format!("Can not remove {:?} file", write_to_supplement)[..]);
  }
  if write_to_schema.exists() {
    fs::remove_file(&write_to_schema).expect(&format!("Can not remove {:?} file", write_to_schema)[..]);
  }


  let tpl_path = "build/tpl/**/*";
  bog::debug(format!("Template path: {}", tpl_path));
  let mut tera = Tera::new(tpl_path).expect("Can not create Tera template engine.");
  tera.register_filter("field_type", filter::filter_field_type);

  let apipe = Apipe::new(czs);


  // common
  let td_common = parser::td_common(&apipe);
  let mut context_common = Context::new();
  context_common.insert("common", &td_common);
  render(&tera, "tdcm.tpl.txt", &context_common, &write_to_types, "rust");
  // schema
  render(&tera, "schema.common.tpl.toml", &context_common, &write_to_schema, "toml");

  // rtd types
  let td_all_types = parser::td_all_types(&apipe);
  td_all_types.iter().for_each(|td| {
    let mut context = Context::new();
    context.insert("td", &td);
    render(&tera, "tdfn.tpl.txt", &context, &write_to_types, "rust");
    // schema
    render(&tera, "schema.rtd.tpl.toml", &context, &write_to_schema, "toml");
  });

  // supplement
  let td_supplement = parser::td_supplement(&apipe);
  let mut context_supplement = Context::new();
  context_supplement.insert("sp", &td_supplement);
  render(&tera, "tdsupplement.tpl.txt", &context_supplement, &write_to_supplement, "rust");
}


fn is_skip<S: AsRef<str>>(cname: S) -> bool {
  let low_cname = cname.as_ref().to_lowercase();
  // skip jsonObjectMember JsonValue jsonValueNull jsonValueBoolean jsonValueNumber jsonValueString jsonValueArray jsonValueObject
  if low_cname.starts_with("json") {
    return true;
  }
  // skip GetJsonString
  if low_cname.contains("getjsonstring") ||
    low_cname.contains("saveapplicationlogevent") ||
    low_cname.contains("getjsonvalue") {
    bog::info(format!("Skip => {}", cname.as_ref()));
    return true;
  }
  false
}




fn render<P: AsRef<Path>>(tera: &Tera, tpl: &str, context: &Context, write_to: P, code_type: &str) {
  let body = tera.render(tpl, &context).unwrap();
  bog::info(format!("Append code to {} ```{}\\n{}\\n```",
                    write_to.as_ref().to_str().unwrap(),
                    code_type,
                    body.trim().replace("\n", "\\n"))
  );
  toolkit::fs::append(write_to, &body[..]).unwrap();
}
