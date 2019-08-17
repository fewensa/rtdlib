
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes type of notifications in the group. 
#[typetag::serde(tag = "@struct")]
pub trait NotificationGroupType: Object + RObject + Debug {}






impl NotificationGroupType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<NotificationGroupType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDNotificationGroupTypeType {
  NotificationGroupTypeCalls,
  NotificationGroupTypeMentions,
  NotificationGroupTypeMessages,
  NotificationGroupTypeSecretChat,
  
}
impl RTDNotificationGroupTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDNotificationGroupTypeType)(text.as_ref()) }
}



