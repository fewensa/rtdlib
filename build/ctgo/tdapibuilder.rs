use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use curl::easy::{Easy, ProxyType};
use tera::{Context, Tera};

use crate::{bakit, bog};
use crate::ctgo::apipe::{Apipe, FieldINF};
use scraper::{Html, Selector};

const TG_API_DOC_BASE_URL: &'static str = "https://core.telegram.org/tdlib/docs/";

fn download_page<S: AsRef<str>, P: AsRef<Path>>(url: S, save_path: P, save_name: S) -> &'static str {
  let path = save_path.as_ref();
  let save_name = save_name.as_ref();
  let url = &format!("{}{}", TG_API_DOC_BASE_URL, url.as_ref())[..];

  if !path.exists() {
    fs::create_dir_all(path).expect("Can not create save path.");
  }
  let fpath = path.join(save_name);
  if fpath.exists() {
    bog::debug(format!("exists {}/{} file, use cache.", bakit::canonicalize_path(path), save_name));
    return bakit::canonicalize_path(path.join(save_name));
  }

  bog::debug(format!("Download {} ==> {}/{}", url, bakit::canonicalize_path(path), save_name));

  let mut easy = Easy::new();
  let hostname: Option<String> = hostname::get_hostname();
  if let Some(name) = hostname {
    if let "amip" = &name[..] {
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
  fs::write(&fpath, content).expect(&format!("Can not write conten to => {:?}", bakit::canonicalize_path(&fpath))[..]);
  return bakit::canonicalize_path(fpath);

//  println!("===> {}", easy.response_code().unwrap());
}

pub fn build<P: AsRef<Path>, S: AsRef<str>>(save_path: P, write_to: S) {
  let write_to = write_to.as_ref();
  rm_rawtd(write_to);
  handle_main("td__api_8h.html", save_path, write_to);
  bog::info("Generate complete.");
}


fn handle_main<P: AsRef<Path>, S: AsRef<str>>(main_page: &'static str, save_path: P, write_to: S) {
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
    czs.push((cname.trim().to_string(), path.to_string()));
    count += 1;
  });

  bog::info(format!("CLASS COUNT: {}", count));

  self::gen_rs(czs, write_to);
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
  let path = Path::new(rawtd_path(write_to));
  if !path.exists() {
    File::create(path.clone()).expect(&format!("Can not create {}", write_to)[..]);
    bog::debug(format!("Create {}", bakit::canonicalize_path(path.clone())));
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
  let path = Path::new(rawtd_path(write_to));
  if path.exists() {
    fs::remove_file(path).expect(&format!("Can not remove {} file", write_to)[..]);
  }
}

fn rawtd_path<S: AsRef<str>>(write_to: S) -> &'static str {
  let path = format!("{}/src/{}", bakit::root_dir(), write_to.as_ref());
  Box::leak(path.into_boxed_str())
}

fn gen_rs<S: AsRef<str>>(czs: Vec<(String, String)>, write_to: S) {
  let write_to = write_to.as_ref();
  let tpl_path = &format!("{}/build/tpl/**/*", bakit::canonicalize_path(Path::new(bakit::root_dir())))[..];
  bog::debug(format!("Template path: {}", tpl_path));
  let tera = Tera::new(tpl_path).expect("Can not create Tera template engine.");

  let apipe = Apipe::new(czs.clone());

  self::gen_common(write_to);

  apipe.names().iter().for_each(|name| {
    let mut context = Context::new();
    context.insert("origin_name", name);
    context.insert("clz_name", &toolkit::text::uppercase_first_char(name)[..]);
    context.insert("is_private", &false);

    self::gen_trait2(&apipe, &mut context, name);
    self::gen_father2(&apipe, &mut context, name);
    self::gen_fields2(&apipe, &mut context, name);
    self::gen_fill(&apipe, &mut context);


    match tera.render("tdfn.rs.txt", &context) {
      Ok(s) => write_rawtd(s, write_to.to_string()),
      Err(e) => {
        println!("Error: {}", e);
        for e in e.iter().skip(1) {
          println!("Reason: {}", e);
        }
        panic!("Can not gen rawfn => {:?}", e);
      }
    }
  });
}

fn gen_fill(apipe: &Apipe, context: &mut Context) {
  let result = context.as_json().expect("Tera engine fail.");

  // set trait typetag
  let cit = result.get("clz_is_trait").expect("Not set clz_is_trait");
  let ist = if cit.is_boolean() { cit.as_bool().unwrap().to_string() } else { cit.as_str().unwrap().to_string() };
  let mut set_trait_typetag = true;
  if ist == "true" {
    let clz_name = result.get("clz_name").expect("Not set clz_name");
    let clz_name = clz_name.as_str().unwrap().to_lowercase();
    if clz_name == "object" || clz_name == "function" {
      set_trait_typetag = false;
    }
  }
  context.insert("set_trait_typetag", &set_trait_typetag);

  // impl trait typetag
  let mut impl_trait_typetag = true;
  if let Some(clz_super) = result.get("clz_super") {
    let clz_super = &clz_super.as_str().unwrap().to_lowercase();
    if clz_super == "object" ||
      clz_super == "function" ||
      clz_super == "tlobject" {
      impl_trait_typetag = false;
    }
  }
  context.insert("impl_trait_typetag", &impl_trait_typetag);

  // has builder struct
  let mut has_builder_struct = true;
  if let Some(v) = result.get("is_private") {
    if v.as_bool().map_or(false, |a| a) {
      has_builder_struct = false;
    }
  }
  if let Some(v) = result.get("clz_is_trait") {
    if v.as_bool().map_or(false, |a| a) {
      has_builder_struct = false;
    }
  }
  if let Some(clz_super) = result.get("clz_super") {
    let clz_super = &clz_super.as_str().unwrap().to_lowercase();
    if clz_super == "update" ||
      clz_super == "authorizationstate" {
      has_builder_struct = false;
    }
  }

  context.insert("has_builder_struct", &has_builder_struct);
}

fn gen_trait2(apipe: &Apipe, context: &mut Context, name: &String) {
  let is_trait = apipe.is_trait(name);
  let description = apipe.description(name);
  context.insert("clz_is_trait", &is_trait);
  context.insert("clz_description", &description);
}

fn gen_father2(apipe: &Apipe, context: &mut Context, name: &String) {
  apipe.father_class(name).map(|father| {
    let text = toolkit::text::uppercase_first_char(father);
    context.insert("clz_super", &text[..]);
  });
}

fn gen_fields2(apipe: &Apipe, context: &mut Context, name: &String) {
  let mut has_trait_field = false;
  let fields = apipe.fields(name)
    .iter()
    .map(|field| {
      field.iter().map(|(inf, val)| {
        if !has_trait_field && inf == &FieldINF::IsTrait {
          has_trait_field = val == "1";
        }
        (&inf.string()[..], val.clone())
      })
        .collect::<HashMap<&str, String>>()
    })
    .collect::<Vec<HashMap<&str, String>>>();

  context.insert("fields", &fields);
  context.insert("fields_size", &fields.len());
  context.insert("has_trait_field", &has_trait_field);
}

fn gen_common<S: AsRef<str>>(write_to: S) {
  let cm_path = Path::new(bakit::root_dir()).join("build/tpl/tdcm.rs.txt");
  match fs::read_to_string(cm_path) {
    Ok(text) => write_rawtd(text, write_to.as_ref().to_string()),
    Err(_) => return
  }
}
