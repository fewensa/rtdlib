
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a notification. 
#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notification
  /// Unique persistent identifier of this notification.
  id: Option<i32>,
  /// Notification date.
  date: Option<i32>,
  /// Notification type.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<NotificationType>>,
  
}


impl Clone for Notification {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for Notification {}
impl RObject for Notification {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notification" }
  fn td_type(&self) -> RTDType { RTDType::Notification }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Notification {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notification".to_string(),
      id: None,
      date: None,
      type_: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn type_(&self) -> Option<Box<NotificationType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<NotificationType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



