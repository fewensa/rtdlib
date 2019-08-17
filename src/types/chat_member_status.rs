
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Provides information about the status of a member in a chat. 
#[typetag::serde(tag = "@struct")]
pub trait ChatMemberStatus: Object + RObject + Debug {}






impl ChatMemberStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<ChatMemberStatus> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDChatMemberStatusType {
  ChatMemberStatusAdministrator,
  ChatMemberStatusBanned,
  ChatMemberStatusCreator,
  ChatMemberStatusLeft,
  ChatMemberStatusMember,
  ChatMemberStatusRestricted,
  
}
impl RTDChatMemberStatusType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDChatMemberStatusType)(text.as_ref()) }
}



