
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's identity card. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeIdentityCard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeIdentityCard
  
}



impl Object for PassportElementTypeIdentityCard {}
impl RObject for PassportElementTypeIdentityCard {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeIdentityCard" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeIdentityCard }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeIdentityCard {}


impl PassportElementTypeIdentityCard {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeIdentityCard".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



