
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceTranslationFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceTranslationFile
  /// Index of a file with the error.
  file_index: Option<i32>,
  
}



impl Object for PassportElementErrorSourceTranslationFile {}
impl RObject for PassportElementErrorSourceTranslationFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceTranslationFile" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceTranslationFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceTranslationFile {}


impl PassportElementErrorSourceTranslationFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceTranslationFile".to_string(),
      file_index: None,
      
    }
  }
  
  pub fn file_index(&self) -> Option<i32> { self.file_index.clone() }
  #[doc(hidden)] pub fn _set_file_index(&mut self, file_index: i32) -> &mut Self { self.file_index = Some(file_index); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



