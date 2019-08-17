
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The translation of the document contains an error. The error will be considered resolved when the list of translation files changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceTranslationFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceTranslationFiles
  
}



impl Object for PassportElementErrorSourceTranslationFiles {}
impl RObject for PassportElementErrorSourceTranslationFiles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceTranslationFiles" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceTranslationFiles }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceTranslationFiles {}


impl PassportElementErrorSourceTranslationFiles {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceTranslationFiles".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



