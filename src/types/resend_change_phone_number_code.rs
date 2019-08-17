
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Re-sends the authentication code sent to confirm a new phone number for the user. Works only if the previously received 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendChangePhoneNumberCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // resendChangePhoneNumberCode
  
}



impl Object for ResendChangePhoneNumberCode {}
impl RObject for ResendChangePhoneNumberCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendChangePhoneNumberCode" }
  fn td_type(&self) -> RTDType { RTDType::ResendChangePhoneNumberCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ResendChangePhoneNumberCode {}


impl ResendChangePhoneNumberCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "resendChangePhoneNumberCode".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



