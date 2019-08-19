
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceUnspecified {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceUnspecified
  
}



impl Object for PassportElementErrorSourceUnspecified {}
impl RObject for PassportElementErrorSourceUnspecified {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceUnspecified" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceUnspecified }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceUnspecified {}


impl PassportElementErrorSourceUnspecified {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceUnspecified".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



