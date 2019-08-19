
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A group containing a notification of type 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationGroupTypeSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationGroupTypeSecretChat
  
}



impl Object for NotificationGroupTypeSecretChat {}
impl RObject for NotificationGroupTypeSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationGroupTypeSecretChat" }
  fn td_type(&self) -> RTDType { RTDType::NotificationGroupTypeSecretChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationGroupType for NotificationGroupTypeSecretChat {}


impl NotificationGroupTypeSecretChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationGroupTypeSecretChat".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



