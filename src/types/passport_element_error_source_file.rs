
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file contains an error. The error will be considered resolved when the file changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceFile
  /// Index of a file with the error.
  file_index: Option<i32>,
  
}



impl Object for PassportElementErrorSourceFile {}
impl RObject for PassportElementErrorSourceFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceFile" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceFile {}


impl PassportElementErrorSourceFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceFile".to_string(),
      file_index: None,
      
    }
  }
  
  pub fn file_index(&self) -> Option<i32> { self.file_index.clone() }
  #[doc(hidden)] pub fn _set_file_index(&mut self, file_index: i32) -> &mut Self { self.file_index = Some(file_index); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



