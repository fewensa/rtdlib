
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A list of active notifications in a notification group has changed. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNotificationGroup
  /// Unique notification group identifier.
  notification_group_id: Option<i32>,
  /// New type of the notification group.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<NotificationGroupType>>,
  /// Identifier of a chat to which all notifications in the group belong.
  chat_id: Option<i64>,
  /// Chat identifier, which notification settings must be applied to the added notifications.
  notification_settings_chat_id: Option<i64>,
  /// True, if the notifications should be shown without sound.
  is_silent: Option<bool>,
  /// Total number of unread notifications in the group, can be bigger than number of active notifications.
  total_count: Option<i32>,
  /// List of added group notifications, sorted by notification ID.
  added_notifications: Option<Vec<Notification>>,
  /// Identifiers of removed group notifications, sorted by notification ID.
  removed_notification_ids: Option<Vec<i32>>,
  
}


impl Clone for UpdateNotificationGroup {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateNotificationGroup {}
impl RObject for UpdateNotificationGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNotificationGroup" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNotificationGroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNotificationGroup {}


impl UpdateNotificationGroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNotificationGroup".to_string(),
      notification_group_id: None,
      type_: None,
      chat_id: None,
      notification_settings_chat_id: None,
      is_silent: None,
      total_count: None,
      added_notifications: None,
      removed_notification_ids: None,
      
    }
  }
  
  pub fn notification_group_id(&self) -> Option<i32> { self.notification_group_id.clone() }
  #[doc(hidden)] pub fn _set_notification_group_id(&mut self, notification_group_id: i32) -> &mut Self { self.notification_group_id = Some(notification_group_id); self }
  
  pub fn type_(&self) -> Option<Box<NotificationGroupType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<NotificationGroupType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn notification_settings_chat_id(&self) -> Option<i64> { self.notification_settings_chat_id.clone() }
  #[doc(hidden)] pub fn _set_notification_settings_chat_id(&mut self, notification_settings_chat_id: i64) -> &mut Self { self.notification_settings_chat_id = Some(notification_settings_chat_id); self }
  
  pub fn is_silent(&self) -> Option<bool> { self.is_silent.clone() }
  #[doc(hidden)] pub fn _set_is_silent(&mut self, is_silent: bool) -> &mut Self { self.is_silent = Some(is_silent); self }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn added_notifications(&self) -> Option<Vec<Notification>> { self.added_notifications.clone() }
  #[doc(hidden)] pub fn _set_added_notifications(&mut self, added_notifications: Vec<Notification>) -> &mut Self { self.added_notifications = Some(added_notifications); self }
  
  pub fn removed_notification_ids(&self) -> Option<Vec<i32>> { self.removed_notification_ids.clone() }
  #[doc(hidden)] pub fn _set_removed_notification_ids(&mut self, removed_notification_ids: Vec<i32>) -> &mut Self { self.removed_notification_ids = Some(removed_notification_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



