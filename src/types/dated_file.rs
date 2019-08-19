
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// File with the date it was uploaded. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatedFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // datedFile
  /// The file.
  file: Option<File>,
  /// Point in time (Unix timestamp) when the file was uploaded.
  date: Option<i32>,
  
}



impl Object for DatedFile {}
impl RObject for DatedFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "datedFile" }
  fn td_type(&self) -> RTDType { RTDType::DatedFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl DatedFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "datedFile".to_string(),
      file: None,
      date: None,
      
    }
  }
  
  pub fn file(&self) -> Option<File> { self.file.clone() }
  #[doc(hidden)] pub fn _set_file(&mut self, file: File) -> &mut Self { self.file = Some(file); self }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



