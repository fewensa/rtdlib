
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the phone number verification code for Telegram Passport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPhoneNumberVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkPhoneNumberVerificationCode
  /// Verification code.
  code: Option<String>,
  
}



impl Object for CheckPhoneNumberVerificationCode {}
impl RObject for CheckPhoneNumberVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkPhoneNumberVerificationCode" }
  fn td_type(&self) -> RTDType { RTDType::CheckPhoneNumberVerificationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckPhoneNumberVerificationCode {}


impl CheckPhoneNumberVerificationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkPhoneNumberVerificationCode".to_string(),
      code: None,
      
    }
  }
  
  pub fn code(&self) -> Option<String> { self.code.clone() }
  #[doc(hidden)] pub fn _set_code(&mut self, code: String) -> &mut Self { self.code = Some(code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



