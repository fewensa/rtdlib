
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Information about the authentication code that was sent. 
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationCodeInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authenticationCodeInfo
  /// A phone number that is being authenticated.
  phone_number: Option<String>,
  /// Describes the way the code was sent to the user.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<AuthenticationCodeType>>,
  /// Describes the way the next code will be sent to the user; may be null.
  next_type: Option<Box<AuthenticationCodeType>>,
  /// Timeout before the code should be re-sent, in seconds.
  timeout: Option<i32>,
  
}


impl Clone for AuthenticationCodeInfo {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for AuthenticationCodeInfo {}
impl RObject for AuthenticationCodeInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authenticationCodeInfo" }
  fn td_type(&self) -> RTDType { RTDType::AuthenticationCodeInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl AuthenticationCodeInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authenticationCodeInfo".to_string(),
      phone_number: None,
      type_: None,
      next_type: None,
      timeout: None,
      
    }
  }
  
  pub fn phone_number(&self) -> Option<String> { self.phone_number.clone() }
  #[doc(hidden)] pub fn _set_phone_number(&mut self, phone_number: String) -> &mut Self { self.phone_number = Some(phone_number); self }
  
  pub fn type_(&self) -> Option<Box<AuthenticationCodeType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<AuthenticationCodeType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn next_type(&self) -> Option<Box<AuthenticationCodeType>> { self.next_type.clone() }
  #[doc(hidden)] pub fn _set_next_type(&mut self, next_type: Box<AuthenticationCodeType>) -> &mut Self { self.next_type = Some(next_type); self }
  
  pub fn timeout(&self) -> Option<i32> { self.timeout.clone() }
  #[doc(hidden)] pub fn _set_timeout(&mut self, timeout: i32) -> &mut Self { self.timeout = Some(timeout); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



