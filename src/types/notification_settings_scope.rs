
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the types of chats to which notification settings are applied. 
#[typetag::serde(tag = "@struct")]
pub trait NotificationSettingsScope: Object + RObject + Debug {}






impl NotificationSettingsScope {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<NotificationSettingsScope> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDNotificationSettingsScopeType {
  NotificationSettingsScopeChannelChats,
  NotificationSettingsScopeGroupChats,
  NotificationSettingsScopePrivateChats,
  
}
impl RTDNotificationSettingsScopeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDNotificationSettingsScopeType)(text.as_ref()) }
}



