
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the type of a chat. 
#[typetag::serde(tag = "@struct")]
pub trait ChatType: Object + RObject + Debug {}






impl ChatType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<ChatType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDChatTypeType {
  ChatTypeBasicGroup,
  ChatTypePrivate,
  ChatTypeSecret,
  ChatTypeSupergroup,
  
}
impl RTDChatTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDChatTypeType)(text.as_ref()) }
}



