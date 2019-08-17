
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's phone number. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypePhoneNumber
  
}



impl Object for PassportElementTypePhoneNumber {}
impl RObject for PassportElementTypePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypePhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypePhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypePhoneNumber {}


impl PassportElementTypePhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypePhoneNumber".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



