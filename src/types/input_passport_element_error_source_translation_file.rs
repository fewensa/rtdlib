
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceTranslationFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementErrorSourceTranslationFile
  /// Current hash of the file containing the translation.
  file_hash: Option<String>,
  
}



impl Object for InputPassportElementErrorSourceTranslationFile {}
impl RObject for InputPassportElementErrorSourceTranslationFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceTranslationFile" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementErrorSourceTranslationFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElementErrorSource for InputPassportElementErrorSourceTranslationFile {}


impl InputPassportElementErrorSourceTranslationFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementErrorSourceTranslationFile".to_string(),
      file_hash: None,
      
    }
  }
  
  pub fn file_hash(&self) -> Option<String> { self.file_hash.clone() }
  #[doc(hidden)] pub fn _set_file_hash(&mut self, file_hash: String) -> &mut Self { self.file_hash = Some(file_hash); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



