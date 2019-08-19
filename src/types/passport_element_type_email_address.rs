
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's email address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeEmailAddress
  
}



impl Object for PassportElementTypeEmailAddress {}
impl RObject for PassportElementTypeEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeEmailAddress" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeEmailAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeEmailAddress {}


impl PassportElementTypeEmailAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeEmailAddress".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



