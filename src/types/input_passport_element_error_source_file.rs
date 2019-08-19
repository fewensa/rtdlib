
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file contains an error. The error is considered resolved when the file changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementErrorSourceFile
  /// Current hash of the file which has the error.
  file_hash: Option<String>,
  
}



impl Object for InputPassportElementErrorSourceFile {}
impl RObject for InputPassportElementErrorSourceFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceFile" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementErrorSourceFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElementErrorSource for InputPassportElementErrorSourceFile {}


impl InputPassportElementErrorSourceFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementErrorSourceFile".to_string(),
      file_hash: None,
      
    }
  }
  
  pub fn file_hash(&self) -> Option<String> { self.file_hash.clone() }
  #[doc(hidden)] pub fn _set_file_hash(&mut self, file_hash: String) -> &mut Self { self.file_hash = Some(file_hash); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



