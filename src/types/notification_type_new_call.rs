
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// New call was received. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTypeNewCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationTypeNewCall
  /// Call identifier.
  call_id: Option<i32>,
  
}



impl Object for NotificationTypeNewCall {}
impl RObject for NotificationTypeNewCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationTypeNewCall" }
  fn td_type(&self) -> RTDType { RTDType::NotificationTypeNewCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationType for NotificationTypeNewCall {}


impl NotificationTypeNewCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationTypeNewCall".to_string(),
      call_id: None,
      
    }
  }
  
  pub fn call_id(&self) -> Option<i32> { self.call_id.clone() }
  #[doc(hidden)] pub fn _set_call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



