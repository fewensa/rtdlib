
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeNotification
  /// Identifier of notification group to which the notification belongs.
  notification_group_id: Option<i32>,
  /// Identifier of removed notification.
  notification_id: Option<i32>,
  
}



impl Object for RemoveNotification {}
impl RObject for RemoveNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeNotification" }
  fn td_type(&self) -> RTDType { RTDType::RemoveNotification }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveNotification {}


impl RemoveNotification {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeNotification".to_string(),
      notification_group_id: None,
      notification_id: None,
      
    }
  }
  
  pub fn notification_group_id(&self) -> Option<i32> { self.notification_group_id.clone() }
  #[doc(hidden)] pub fn _set_notification_group_id(&mut self, notification_group_id: i32) -> &mut Self { self.notification_group_id = Some(notification_group_id); self }
  
  pub fn notification_id(&self) -> Option<i32> { self.notification_id.clone() }
  #[doc(hidden)] pub fn _set_notification_id(&mut self, notification_id: i32) -> &mut Self { self.notification_id = Some(notification_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



