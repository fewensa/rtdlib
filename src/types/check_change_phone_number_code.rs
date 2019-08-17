
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the authentication code sent to confirm a new phone number of the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckChangePhoneNumberCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkChangePhoneNumberCode
  /// Verification code received by SMS, phone call or flash call.
  code: Option<String>,
  
}



impl Object for CheckChangePhoneNumberCode {}
impl RObject for CheckChangePhoneNumberCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkChangePhoneNumberCode" }
  fn td_type(&self) -> RTDType { RTDType::CheckChangePhoneNumberCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckChangePhoneNumberCode {}


impl CheckChangePhoneNumberCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkChangePhoneNumberCode".to_string(),
      code: None,
      
    }
  }
  
  pub fn code(&self) -> Option<String> { self.code.clone() }
  #[doc(hidden)] pub fn _set_code(&mut self, code: String) -> &mut Self { self.code = Some(code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



