use std::env;
use std::fs;
use std::path::Path;

pub fn out_dir() -> &'static str {
  let out_dir_env = env::var("OUT_DIR").unwrap();
  Box::leak(out_dir_env.into_boxed_str())
}

pub fn canonicalize_path<S: AsRef<Path>>(path: S) -> &'static str {
  let buf = fs::canonicalize(path).unwrap();
  Box::leak(buf.to_str().unwrap().to_string().into_boxed_str())
}

pub fn root_dir() -> &'static str {
  canonicalize_path("./")
}

pub fn flatten_option<T>(origin: Option<Option<T>>) -> Option<T> {
  match origin {
    Some(v) => v,
    None => None,
  }
}
