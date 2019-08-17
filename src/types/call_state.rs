
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the current call state. 
#[typetag::serde(tag = "@struct")]
pub trait CallState: Object + RObject + Debug {}






impl CallState {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<CallState> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDCallStateType {
  CallStateDiscarded,
  CallStateError,
  CallStateExchangingKeys,
  CallStateHangingUp,
  CallStatePending,
  CallStateReady,
  
}
impl RTDCallStateType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDCallStateType)(text.as_ref()) }
}



