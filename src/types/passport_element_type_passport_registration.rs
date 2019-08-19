
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the registration page of the user's passport. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypePassportRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypePassportRegistration
  
}



impl Object for PassportElementTypePassportRegistration {}
impl RObject for PassportElementTypePassportRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypePassportRegistration" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypePassportRegistration }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypePassportRegistration {}


impl PassportElementTypePassportRegistration {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypePassportRegistration".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



