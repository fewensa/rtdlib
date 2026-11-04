#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typed_builder;

use std::path::{Path, PathBuf};

use tera::Tera;

use cycle::*;
use rtd::RTD;
use tgclient::TGClient;
use tl_parser::parser::parser::TLParser;
use tokenwrap::TokenWrap;

mod cycle;
mod rtd;
mod tdfill;
mod terafill;
mod tgclient;
mod tokenwrap;
mod types;

fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Info);

  let generate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
  let rtd_out = std::env::var("RTD_OUT")
    .map(PathBuf::from)
    .unwrap_or_else(|_| generate_root.join("../target/rtd-out"));

  let tdtypefill =
    tdfill::TDTypeFill::new(generate_root.join("schema/td_type_fill.toml")).unwrap();

  let config: Config = Config::builder()
    .path_rtd(rtd_out)
    .path_telegram_client(generate_root.join("../target/telegram-client-out"))
    .path_template(generate_root.join("template"))
    .file_tl(generate_root.join("schema/td_api.tl"))
    .build();

  let mut tera = Tera::new(&format!("{}/**/*", config.path_template().display()))
    .expect("Can not create Tera template engine.");

  let tokens = TLParser::new(config.file_tl()).parse().unwrap();
  let tknwrap = TokenWrap::new(tokens, tdtypefill.clone());

  terafill::fill(&mut tera, tknwrap.clone());

  let renderer = Renderer::builder().tera(tera).build();

  let cycle: Cycle = Cycle::builder()
    .config(config)
    .tknwrap(tknwrap)
    .renderer(renderer)
    .build();

  RTD::new(&cycle).generate().unwrap();
  // telegram-client wrappers are generated separately.
  let _ = TGClient::new(&cycle);
}
