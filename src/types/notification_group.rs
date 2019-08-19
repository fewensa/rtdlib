
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a group of notifications. 
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationGroup
  /// Unique persistent auto-incremented from 1 identifier of the notification group.
  id: Option<i32>,
  /// Type of the group.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<NotificationGroupType>>,
  /// Identifier of a chat to which all notifications in the group belong.
  chat_id: Option<i64>,
  /// Total number of active notifications in the group.
  total_count: Option<i32>,
  /// The list of active notifications.
  notifications: Option<Vec<Notification>>,
  
}


impl Clone for NotificationGroup {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for NotificationGroup {}
impl RObject for NotificationGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationGroup" }
  fn td_type(&self) -> RTDType { RTDType::NotificationGroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl NotificationGroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationGroup".to_string(),
      id: None,
      type_: None,
      chat_id: None,
      total_count: None,
      notifications: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn type_(&self) -> Option<Box<NotificationGroupType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<NotificationGroupType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn notifications(&self) -> Option<Vec<Notification>> { self.notifications.clone() }
  #[doc(hidden)] pub fn _set_notifications(&mut self, notifications: Vec<Notification>) -> &mut Self { self.notifications = Some(notifications); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



