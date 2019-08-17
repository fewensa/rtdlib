
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the current state of the connection to Telegram servers. 
#[typetag::serde(tag = "@struct")]
pub trait ConnectionState: Object + RObject + Debug {}






impl ConnectionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<ConnectionState> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDConnectionStateType {
  ConnectionStateConnecting,
  ConnectionStateConnectingToProxy,
  ConnectionStateReady,
  ConnectionStateUpdating,
  ConnectionStateWaitingForNetwork,
  
}
impl RTDConnectionStateType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDConnectionStateType)(text.as_ref()) }
}



