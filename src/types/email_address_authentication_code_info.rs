
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Information about the email address authentication code that was sent. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationCodeInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // emailAddressAuthenticationCodeInfo
  /// Pattern of the email address to which an authentication code was sent.
  email_address_pattern: Option<String>,
  /// Length of the code; 0 if unknown.
  length: Option<i32>,
  
}



impl Object for EmailAddressAuthenticationCodeInfo {}
impl RObject for EmailAddressAuthenticationCodeInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "emailAddressAuthenticationCodeInfo" }
  fn td_type(&self) -> RTDType { RTDType::EmailAddressAuthenticationCodeInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl EmailAddressAuthenticationCodeInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "emailAddressAuthenticationCodeInfo".to_string(),
      email_address_pattern: None,
      length: None,
      
    }
  }
  
  pub fn email_address_pattern(&self) -> Option<String> { self.email_address_pattern.clone() }
  #[doc(hidden)] pub fn _set_email_address_pattern(&mut self, email_address_pattern: String) -> &mut Self { self.email_address_pattern = Some(email_address_pattern); self }
  
  pub fn length(&self) -> Option<i32> { self.length.clone() }
  #[doc(hidden)] pub fn _set_length(&mut self, length: i32) -> &mut Self { self.length = Some(length); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



