#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;

use crate::downloader::Downloader;
use crate::generator::Generator;

mod apipe;
mod downloader;
mod log;
mod types;
mod generator;
mod filter;
mod parser;

fn main() {
  log::clear();

  let rtdlib_path = toolkit::path::root_dir().join("../").canonicalize().expect("Can not get rtdlib path.");

  let download_path = rtdlib_path.join("_tdapihtml");
  let downloader = Downloader::new(download_path);

  let classes = downloader.download();

  let generator = Generator::new(rtdlib_path);
  generator.generate(classes);

//  println!("{:?}", rtdlib_path);
  log::info("GENERATED DONE")
}

