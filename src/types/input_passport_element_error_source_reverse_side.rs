
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceReverseSide {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementErrorSourceReverseSide
  /// Current hash of the file containing the reverse side.
  file_hash: Option<String>,
  
}



impl Object for InputPassportElementErrorSourceReverseSide {}
impl RObject for InputPassportElementErrorSourceReverseSide {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceReverseSide" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementErrorSourceReverseSide }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElementErrorSource for InputPassportElementErrorSourceReverseSide {}


impl InputPassportElementErrorSourceReverseSide {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementErrorSourceReverseSide".to_string(),
      file_hash: None,
      
    }
  }
  
  pub fn file_hash(&self) -> Option<String> { self.file_hash.clone() }
  #[doc(hidden)] pub fn _set_file_hash(&mut self, file_hash: String) -> &mut Self { self.file_hash = Some(file_hash); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



