use std::fs::File;

use failure::Error;

use tl_parser::parser::*;
use tl_parser::parser::parser::TLParser;
use std::path::Path;

#[macro_use]
extern crate log;

fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Debug);

  let path = std::env::current_dir().unwrap().join("schema/master/td_api.tl");
  let parser = TLParser::new(path);
  match parser.parse() {
    Ok(tokens) => {
      debug!("tokens: {:#?}", tokens);
      debug!("finish");
    },
    Err(e) => panic!("{:?}", e)
  }
}
