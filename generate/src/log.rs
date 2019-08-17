use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

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

fn logfile() -> PathBuf {
  let rtdlib_path = toolkit::path::root_dir().join("../").canonicalize().expect("Can not get rtdlib path.");
  rtdlib_path.join("generate.log")
}

pub fn clear() {
  let log_file = self::logfile();
  if log_file.exists() {
    fs::remove_file(log_file).expect("Can not remove build log file.");
  }
}

fn println<S: AsRef<str>>(level: Level, log: S) {
  let log_file = self::logfile();
  let log_text = format!("[{}] -> {}", level.string(), log.as_ref());
  println!("{}", log_text);
  toolkit::fs::append(log_file, log_text).unwrap();
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

