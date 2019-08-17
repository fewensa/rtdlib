
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Re-sends the code to verify a phone number to be added to a user's Telegram Passport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendPhoneNumberVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // resendPhoneNumberVerificationCode
  
}



impl Object for ResendPhoneNumberVerificationCode {}
impl RObject for ResendPhoneNumberVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendPhoneNumberVerificationCode" }
  fn td_type(&self) -> RTDType { RTDType::ResendPhoneNumberVerificationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ResendPhoneNumberVerificationCode {}


impl ResendPhoneNumberVerificationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "resendPhoneNumberVerificationCode".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



