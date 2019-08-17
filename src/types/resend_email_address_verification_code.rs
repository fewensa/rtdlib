
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Re-sends the code to verify an email address to be added to a user's Telegram Passport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendEmailAddressVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // resendEmailAddressVerificationCode
  
}



impl Object for ResendEmailAddressVerificationCode {}
impl RObject for ResendEmailAddressVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendEmailAddressVerificationCode" }
  fn td_type(&self) -> RTDType { RTDType::ResendEmailAddressVerificationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ResendEmailAddressVerificationCode {}


impl ResendEmailAddressVerificationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "resendEmailAddressVerificationCode".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



