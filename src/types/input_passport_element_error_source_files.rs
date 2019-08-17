
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The list of attached files contains an error. The error is considered resolved when the file list changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementErrorSourceFiles
  /// Current hashes of all attached files.
  file_hashes: Option<Vec<String>>,
  
}



impl Object for InputPassportElementErrorSourceFiles {}
impl RObject for InputPassportElementErrorSourceFiles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceFiles" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementErrorSourceFiles }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElementErrorSource for InputPassportElementErrorSourceFiles {}


impl InputPassportElementErrorSourceFiles {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementErrorSourceFiles".to_string(),
      file_hashes: None,
      
    }
  }
  
  pub fn file_hashes(&self) -> Option<Vec<String>> { self.file_hashes.clone() }
  #[doc(hidden)] pub fn _set_file_hashes(&mut self, file_hashes: Vec<String>) -> &mut Self { self.file_hashes = Some(file_hashes); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



