
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceReverseSide {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceReverseSide
  
}



impl Object for PassportElementErrorSourceReverseSide {}
impl RObject for PassportElementErrorSourceReverseSide {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceReverseSide" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceReverseSide }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceReverseSide {}


impl PassportElementErrorSourceReverseSide {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceReverseSide".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



