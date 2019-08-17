
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// New secret chat was created. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTypeNewSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationTypeNewSecretChat
  
}



impl Object for NotificationTypeNewSecretChat {}
impl RObject for NotificationTypeNewSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationTypeNewSecretChat" }
  fn td_type(&self) -> RTDType { RTDType::NotificationTypeNewSecretChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationType for NotificationTypeNewSecretChat {}


impl NotificationTypeNewSecretChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationTypeNewSecretChat".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



