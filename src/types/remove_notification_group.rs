
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNotificationGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeNotificationGroup
  /// Notification group identifier.
  notification_group_id: Option<i32>,
  /// Maximum identifier of removed notifications.
  max_notification_id: Option<i32>,
  
}



impl Object for RemoveNotificationGroup {}
impl RObject for RemoveNotificationGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeNotificationGroup" }
  fn td_type(&self) -> RTDType { RTDType::RemoveNotificationGroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveNotificationGroup {}


impl RemoveNotificationGroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeNotificationGroup".to_string(),
      notification_group_id: None,
      max_notification_id: None,
      
    }
  }
  
  pub fn notification_group_id(&self) -> Option<i32> { self.notification_group_id.clone() }
  #[doc(hidden)] pub fn _set_notification_group_id(&mut self, notification_group_id: i32) -> &mut Self { self.notification_group_id = Some(notification_group_id); self }
  
  pub fn max_notification_id(&self) -> Option<i32> { self.max_notification_id.clone() }
  #[doc(hidden)] pub fn _set_max_notification_id(&mut self, max_notification_id: i32) -> &mut Self { self.max_notification_id = Some(max_notification_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



