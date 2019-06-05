#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::path::Path;

mod bog;
mod ctgo;


fn main() {
  match env::var("BUILD_TDAPI") {
    Ok(val) => {
      let val = val.to_lowercase();
      if &val == "true" || &val == "yes" {
        build_tdapi();
      }
    }
    Err(_) => {}
  }
}

fn build_tdapi() {
  bog::clear();

  let dpath = toolkit::path::out_dir().join("tg_api");
//  let dpath = toolkit::path::root_dir().join("_tmp");
  ctgo::tdapibuilder::build(dpath);
}

