
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementErrorSourceSelfie {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementErrorSourceSelfie
  
}



impl Object for PassportElementErrorSourceSelfie {}
impl RObject for PassportElementErrorSourceSelfie {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceSelfie" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementErrorSourceSelfie }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementErrorSource for PassportElementErrorSourceSelfie {}


impl PassportElementErrorSourceSelfie {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementErrorSourceSelfie".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



