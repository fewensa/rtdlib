
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to "default" and message previews are shown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetAllNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // resetAllNotificationSettings
  
}



impl Object for ResetAllNotificationSettings {}
impl RObject for ResetAllNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resetAllNotificationSettings" }
  fn td_type(&self) -> RTDType { RTDType::ResetAllNotificationSettings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ResetAllNotificationSettings {}


impl ResetAllNotificationSettings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "resetAllNotificationSettings".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



