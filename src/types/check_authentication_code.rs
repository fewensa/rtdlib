
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the authentication code. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAuthenticationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkAuthenticationCode
  /// The verification code received via SMS, Telegram message, phone call, or flash call.
  code: Option<String>,
  /// If the user is not yet registered, the first name of the user; 1-64 characters. You can also pass an empty string for unregistered user there to check verification code validness. In the latter case PHONE_NUMBER_UNOCCUPIED error will be returned for a valid code.
  first_name: Option<String>,
  /// If the user is not yet registered; the last name of the user; optional; 0-64 characters.
  last_name: Option<String>,
  
}



impl Object for CheckAuthenticationCode {}
impl RObject for CheckAuthenticationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkAuthenticationCode" }
  fn td_type(&self) -> RTDType { RTDType::CheckAuthenticationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckAuthenticationCode {}


impl CheckAuthenticationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkAuthenticationCode".to_string(),
      code: None,
      first_name: None,
      last_name: None,
      
    }
  }
  
  pub fn code(&self) -> Option<String> { self.code.clone() }
  #[doc(hidden)] pub fn _set_code(&mut self, code: String) -> &mut Self { self.code = Some(code); self }
  
  pub fn first_name(&self) -> Option<String> { self.first_name.clone() }
  #[doc(hidden)] pub fn _set_first_name(&mut self, first_name: String) -> &mut Self { self.first_name = Some(first_name); self }
  
  pub fn last_name(&self) -> Option<String> { self.last_name.clone() }
  #[doc(hidden)] pub fn _set_last_name(&mut self, last_name: String) -> &mut Self { self.last_name = Some(last_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



