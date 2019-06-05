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
  rm_rawtd("types.rs");
  rm_rawtd("tdsupplement.rs");
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
    if is_skip(cname.to_lowercase()) {
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
  let write_to_types = "types.rs";
  let write_to_supplement = "tdsupplement.rs";

  let tpl_path = "build/tpl/**/*";
  bog::debug(format!("Template path: {}", tpl_path));
  let mut tera = Tera::new(tpl_path).expect("Can not create Tera template engine.");
  tera.register_filter("field_type", filter::filter_field_type);

  let apipe = Apipe::new(czs);



  let td_common = parser::td_common(&apipe);
  let mut context_common = Context::new();
  context_common.insert("common", &td_common);
  render(&tera, "tdcm.tpl.txt", &context_common, write_to_types);



  let td_all_types = parser::td_all_types(&apipe);
  td_all_types.iter().for_each(|td| {
    let mut context = Context::new();
    context.insert("td", &td);
    render(&tera, "tdfn.tpl.txt", &context, write_to_types);
  });


  let td_supplement = parser::td_supplement(&apipe);
  let mut context_supplement = Context::new();
  context_supplement.insert("sp", &td_supplement);
  render(&tera, "tdsupplement.tpl.txt", &context_supplement, write_to_supplement);
}


fn is_skip(cname: String) -> bool {
  // skip jsonObjectMember JsonValue jsonValueNull jsonValueBoolean jsonValueNumber jsonValueString jsonValueArray jsonValueObject
  if cname.starts_with("json") {
    return true;
  }
  // skip GetJsonString
  if cname.contains("getjsonstring") ||
    cname.contains("saveapplicationlogevent") ||
    cname.contains("getjsonvalue") {
    return true;
  }
  false
}

fn write_rawtd<S: AsRef<str>>(content: S, write_to: S) {
  let write_to = write_to.as_ref();
  let path = rawtd_path(write_to);
  if !path.exists() {
    File::create(&path).expect(&format!("Can not create {}", write_to)[..]);
    bog::debug(format!("Create {:?}", path));
  }
  bog::info(format!("Append code to {} ```rust\\n{}\\n```", write_to, content.as_ref().trim().replace("\n", "\\n")));

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();


  if let Err(e) = writeln!(file, "{}", content.as_ref()) {
    panic!("Couldn't write to file: {:?}", e);
  }
}

fn rm_rawtd<S: AsRef<str>>(write_to: S) {
  let write_to = write_to.as_ref();
  let path = rawtd_path(write_to);
  if path.exists() {
    fs::remove_file(&path).expect(&format!("Can not remove {} file", write_to)[..]);
  }
}

fn rawtd_path<S: AsRef<str>>(write_to: S) -> PathBuf {
  Path::new("src")
    .canonicalize()
    .expect("Can not get rawtd path")
    .join(write_to.as_ref())
}


fn render(tera: &Tera, tpl: &str, context: &Context, write_to: &str) {
  match tera.render(tpl, &context) {
    Ok(s) => write_rawtd(s, write_to.to_string()),
    Err(e) => {
      println!("Error: {}", e);
      for e in e.iter().skip(1) {
        println!("Reason: {}", e);
      }
      panic!("Can not gen rawfn => {:?}", e);
    }
  }
}
