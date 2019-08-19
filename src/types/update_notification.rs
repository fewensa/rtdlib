
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A notification was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNotification
  /// Unique notification group identifier.
  notification_group_id: Option<i32>,
  /// Changed notification.
  notification: Option<Notification>,
  
}



impl Object for UpdateNotification {}
impl RObject for UpdateNotification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNotification" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNotification }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNotification {}


impl UpdateNotification {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNotification".to_string(),
      notification_group_id: None,
      notification: None,
      
    }
  }
  
  pub fn notification_group_id(&self) -> Option<i32> { self.notification_group_id.clone() }
  #[doc(hidden)] pub fn _set_notification_group_id(&mut self, notification_group_id: i32) -> &mut Self { self.notification_group_id = Some(notification_group_id); self }
  
  pub fn notification(&self) -> Option<Notification> { self.notification.clone() }
  #[doc(hidden)] pub fn _set_notification(&mut self, notification: Notification) -> &mut Self { self.notification = Some(notification); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



