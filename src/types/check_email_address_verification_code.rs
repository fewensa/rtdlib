
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the email address verification code for Telegram Passport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckEmailAddressVerificationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkEmailAddressVerificationCode
  /// Verification code.
  code: Option<String>,
  
}



impl Object for CheckEmailAddressVerificationCode {}
impl RObject for CheckEmailAddressVerificationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkEmailAddressVerificationCode" }
  fn td_type(&self) -> RTDType { RTDType::CheckEmailAddressVerificationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckEmailAddressVerificationCode {}


impl CheckEmailAddressVerificationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkEmailAddressVerificationCode".to_string(),
      code: None,
      
    }
  }
  
  pub fn code(&self) -> Option<String> { self.code.clone() }
  #[doc(hidden)] pub fn _set_code(&mut self, code: String) -> &mut Self { self.code = Some(code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



