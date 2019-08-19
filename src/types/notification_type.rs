
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains detailed information about a notification. 
#[typetag::serde(tag = "@struct")]
pub trait NotificationType: Object + RObject + Debug {}






impl NotificationType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<NotificationType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDNotificationTypeType {
  NotificationTypeNewCall,
  NotificationTypeNewMessage,
  NotificationTypeNewPushMessage,
  NotificationTypeNewSecretChat,
  
}
impl RTDNotificationTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDNotificationTypeType)(text.as_ref()) }
}



