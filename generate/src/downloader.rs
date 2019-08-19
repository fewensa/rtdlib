use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use reqwest::Proxy;
use scraper::{Html, Selector};

use crate::apipe::Apipe;
use crate::log;

const TG_API_DOC_BASE_URL: &'static str = "https://core.telegram.org/tdlib/docs";


pub struct Downloader<P: AsRef<Path>> {
  save_path: P
}

impl<P: AsRef<Path>> Downloader<P> {
  pub fn new(save_path: P) -> Self {
    let path = save_path.as_ref();
    if !path.exists() {
      fs::create_dir_all(path).expect("Can not create td api html download path.");
    }
    Downloader { save_path }
  }

  fn client(&self) -> reqwest::Client {
    let builder = reqwest::ClientBuilder::new();

    let hostname: Option<String> = hostname::get_hostname();
    if let Some(name) = hostname {
      if "amip" == &name[..] {
        return builder
          .proxy(Proxy::https("http://127.0.0.1:1081")
            .expect("Proxy url error"))
          .build()
          .expect("Can not create request client");
      }
    }
    builder.build()
      .expect("Can not create request client")
  }

  pub fn download(&self) -> Vec<(String, PathBuf)> {
    let base_path = self.save_path.as_ref();
    self.download_page("td__api_8h.html", "_index.html");

    let main_body = fs::read_to_string(base_path.join("_index.html")).expect("Not found telegram api index.html");

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
      if Apipe::is_skip(cname) {
        log::info(format!("Skip => {}", cname));
        return;
      }

      let href = ele_cname.attr("href");
      if let None = href {
        return;
      }
      let href = href.unwrap();

      let path = self.download_page(href, &format!("{}.html", cname)[..]);
      czs.push((cname.trim().to_string(), path));
      count += 1;
    });
    log::info(format!("DOWNLOAD OVER, CLASS COUNT: {}", count));
    czs
  }

  fn download_page<U: AsRef<str>, S: AsRef<str>>(&self, uri: U, save_name: S) -> PathBuf {
    let uri = uri.as_ref();
    let save_name = save_name.as_ref();
    let base_path = self.save_path.as_ref();

    let save_file = base_path.join(save_name);
    if save_file.exists() {
      log::debug(format!("exists {:?} file, use cache.", save_file.canonicalize().expect("Can not get tdapi save path")));
      return save_file;
    }

    File::create(&save_file).expect(&format!("Can not create {} file", save_name)[..]);
    let save_file = save_file.canonicalize().expect("Can not get tdapi save path");

    let url = &format!("{}/{}", TG_API_DOC_BASE_URL, uri)[..];
    log::debug(format!("Download {} ==> {:?}", url, save_file));

    let client = self.client();
    let mut response = client.get(url)
      .send()
      .expect("Can not request");
    let content = response.text().expect(&format!("Can not get body from {}", url)[..]);
    fs::write(&save_file, content).expect(&format!("Can not write html to {:?}", save_file)[..]);
    save_file
  }
}

