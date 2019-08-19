
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains information about the sending state of the message. 
#[typetag::serde(tag = "@struct")]
pub trait MessageSendingState: Object + RObject + Debug {}






impl MessageSendingState {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<MessageSendingState> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDMessageSendingStateType {
  MessageSendingStateFailed,
  MessageSendingStatePending,
  
}
impl RTDMessageSendingStateType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDMessageSendingStateType)(text.as_ref()) }
}



