
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeAddress
  
}



impl Object for PassportElementTypeAddress {}
impl RObject for PassportElementTypeAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeAddress" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeAddress {}


impl PassportElementTypeAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeAddress".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



