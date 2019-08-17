
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains information about the origin of a forwarded message. 
#[typetag::serde(tag = "@struct")]
pub trait MessageForwardOrigin: Object + RObject + Debug {}






impl MessageForwardOrigin {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<MessageForwardOrigin> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDMessageForwardOriginType {
  MessageForwardOriginChannel,
  MessageForwardOriginHiddenUser,
  MessageForwardOriginUser,
  
}
impl RTDMessageForwardOriginType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDMessageForwardOriginType)(text.as_ref()) }
}



