
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends a code to verify an email address to be added to a user's Telegram Passport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailAddressVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendEmailAddressVerificationCode
  /// Email address.
  email_address: Option<String>,
  
}



impl Object for SendEmailAddressVerificationCode {}
impl RObject for SendEmailAddressVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendEmailAddressVerificationCode" }
  fn td_type(&self) -> RTDType { RTDType::SendEmailAddressVerificationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendEmailAddressVerificationCode {}


impl SendEmailAddressVerificationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendEmailAddressVerificationCode".to_string(),
      email_address: None,
      
    }
  }
  
  pub fn email_address(&self) -> Option<String> { self.email_address.clone() }
  #[doc(hidden)] pub fn _set_email_address(&mut self, email_address: String) -> &mut Self { self.email_address = Some(email_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



