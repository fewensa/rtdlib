
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The translation of the document contains an error. The error is considered resolved when the list of files changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceTranslationFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementErrorSourceTranslationFiles
  /// Current hashes of all files with the translation.
  file_hashes: Option<Vec<String>>,
  
}



impl Object for InputPassportElementErrorSourceTranslationFiles {}
impl RObject for InputPassportElementErrorSourceTranslationFiles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceTranslationFiles" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementErrorSourceTranslationFiles }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElementErrorSource for InputPassportElementErrorSourceTranslationFiles {}


impl InputPassportElementErrorSourceTranslationFiles {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementErrorSourceTranslationFiles".to_string(),
      file_hashes: None,
      
    }
  }
  
  pub fn file_hashes(&self) -> Option<Vec<String>> { self.file_hashes.clone() }
  #[doc(hidden)] pub fn _set_file_hashes(&mut self, file_hashes: Vec<String>) -> &mut Self { self.file_hashes = Some(file_hashes); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



