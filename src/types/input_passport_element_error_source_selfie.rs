
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The selfie contains an error. The error is considered resolved when the file with the selfie changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceSelfie {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementErrorSourceSelfie
  /// Current hash of the file containing the selfie.
  file_hash: Option<String>,
  
}



impl Object for InputPassportElementErrorSourceSelfie {}
impl RObject for InputPassportElementErrorSourceSelfie {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceSelfie" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementErrorSourceSelfie }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElementErrorSource for InputPassportElementErrorSourceSelfie {}


impl InputPassportElementErrorSourceSelfie {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementErrorSourceSelfie".to_string(),
      file_hash: None,
      
    }
  }
  
  pub fn file_hash(&self) -> Option<String> { self.file_hash.clone() }
  #[doc(hidden)] pub fn _set_file_hash(&mut self, file_hash: String) -> &mut Self { self.file_hash = Some(file_hash); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



