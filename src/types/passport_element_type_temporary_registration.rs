
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's temporary registration. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeTemporaryRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeTemporaryRegistration
  
}



impl Object for PassportElementTypeTemporaryRegistration {}
impl RObject for PassportElementTypeTemporaryRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeTemporaryRegistration" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeTemporaryRegistration }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeTemporaryRegistration {}


impl PassportElementTypeTemporaryRegistration {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeTemporaryRegistration".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



