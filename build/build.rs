#[macro_use]
extern crate tera;

use std::env;
use std::path::Path;

mod bakit;
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

//  let dpath = Path::new(bakit::out_dir()).join("tg_api");
  let dpath = Path::new(bakit::root_dir()).join("_tmp");
  ctgo::tdapibuilder::build(dpath, "types.rs");
}

