
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Provides information about the method by which an authentication code is delivered to the user. 
#[typetag::serde(tag = "@struct")]
pub trait AuthenticationCodeType: Object + RObject + Debug {}






impl AuthenticationCodeType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<AuthenticationCodeType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDAuthenticationCodeTypeType {
  AuthenticationCodeTypeCall,
  AuthenticationCodeTypeFlashCall,
  AuthenticationCodeTypeSms,
  AuthenticationCodeTypeTelegramMessage,
  
}
impl RTDAuthenticationCodeTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDAuthenticationCodeTypeType)(text.as_ref()) }
}



