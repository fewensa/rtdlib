use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::bakit;

enum Level {
  Debug,
  Info,
  Warning,
  Error
}

impl Level {
  fn string(&self) -> &'static str {
    match *self {
      Level::Debug => "DEBU",
      Level::Info => "INFO",
      Level::Warning => "WARN",
      Level::Error => "ERRO"
    }
  }
}

pub fn clear() {
  let log_file = Path::new(bakit::root_dir()).join("build/build.log");
  fs::remove_file(log_file).expect("Can not remove build log file.");
}

fn println<S: AsRef<str>>(level: Level, log: S) {
  let log_file = Path::new(bakit::root_dir()).join("build/build.log");
//  fs::write(log_path, log.as_ref()).expect("Can not write log file.");
  if !log_file.exists() {
    File::create(log_file.clone()).expect("Can not create log file.");
  }

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(log_file)
    .unwrap();

  if let Err(e) = writeln!(file, "[{}] -> {}", level.string(), log.as_ref()) {
    panic!("Couldn't write to file: {:?}", e);
  }
}


pub fn debug<S: AsRef<str>>(log: S) {
  self::println(Level::Debug, log)
}

pub fn info<S: AsRef<str>>(log: S) {
  self::println(Level::Info, log)
}

pub fn warning<S: AsRef<str>>(log: S) {
  self::println(Level::Warning, log)
}

pub fn error<S: AsRef<str>>(log: S) {
  self::println(Level::Error, log)
}

