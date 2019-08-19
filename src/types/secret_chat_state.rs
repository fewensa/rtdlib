
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the current secret chat state. 
#[typetag::serde(tag = "@struct")]
pub trait SecretChatState: Object + RObject + Debug {}






impl SecretChatState {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<SecretChatState> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDSecretChatStateType {
  SecretChatStateClosed,
  SecretChatStatePending,
  SecretChatStateReady,
  
}
impl RTDSecretChatStateType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDSecretChatStateType)(text.as_ref()) }
}



