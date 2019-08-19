
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents result of checking whether a username can be set for a chat. 
#[typetag::serde(tag = "@struct")]
pub trait CheckChatUsernameResult: Object + RObject + Debug {}






impl CheckChatUsernameResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<CheckChatUsernameResult> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDCheckChatUsernameResultType {
  CheckChatUsernameResultOk,
  CheckChatUsernameResultPublicChatsTooMuch,
  CheckChatUsernameResultPublicGroupsUnavailable,
  CheckChatUsernameResultUsernameInvalid,
  CheckChatUsernameResultUsernameOccupied,
  
}
impl RTDCheckChatUsernameResultType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDCheckChatUsernameResultType)(text.as_ref()) }
}



