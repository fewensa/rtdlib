
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents the current authorization state of the client. 
#[typetag::serde(tag = "@struct")]
pub trait AuthorizationState: Object + RObject + Debug {}






impl AuthorizationState {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<AuthorizationState> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDAuthorizationStateType {
  AuthorizationStateClosed,
  AuthorizationStateClosing,
  AuthorizationStateLoggingOut,
  AuthorizationStateReady,
  AuthorizationStateWaitCode,
  AuthorizationStateWaitEncryptionKey,
  AuthorizationStateWaitPassword,
  AuthorizationStateWaitPhoneNumber,
  AuthorizationStateWaitTdlibParameters,
  
}
impl RTDAuthorizationStateType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDAuthorizationStateType)(text.as_ref()) }
}



