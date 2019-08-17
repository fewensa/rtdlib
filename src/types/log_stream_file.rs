
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The log is written to a file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStreamFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // logStreamFile
  /// Path to the file to where the internal TDLib log will be written.
  path: Option<String>,
  /// Maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated.
  max_file_size: Option<i64>,
  
}



impl Object for LogStreamFile {}
impl RObject for LogStreamFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logStreamFile" }
  fn td_type(&self) -> RTDType { RTDType::LogStreamFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl LogStream for LogStreamFile {}


impl LogStreamFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "logStreamFile".to_string(),
      path: None,
      max_file_size: None,
      
    }
  }
  
  pub fn path(&self) -> Option<String> { self.path.clone() }
  #[doc(hidden)] pub fn _set_path(&mut self, path: String) -> &mut Self { self.path = Some(path); self }
  
  pub fn max_file_size(&self) -> Option<i64> { self.max_file_size.clone() }
  #[doc(hidden)] pub fn _set_max_file_size(&mut self, max_file_size: i64) -> &mut Self { self.max_file_size = Some(max_file_size); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



