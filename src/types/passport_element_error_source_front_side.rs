
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The front side of the document contains an error. The error will be considered resolved when the file with the front side changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFrontSide {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceFrontSide
  
}



impl Object for PassportElementErrorSourceFrontSide {}
impl RObject for PassportElementErrorSourceFrontSide {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceFrontSide" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceFrontSide }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceFrontSide {}


impl PassportElementErrorSourceFrontSide {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceFrontSide".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



