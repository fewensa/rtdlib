
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettingsScopePrivateChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationSettingsScopePrivateChats
  
}



impl Object for NotificationSettingsScopePrivateChats {}
impl RObject for NotificationSettingsScopePrivateChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationSettingsScopePrivateChats" }
  fn td_type(&self) -> RTDType { RTDType::NotificationSettingsScopePrivateChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationSettingsScope for NotificationSettingsScopePrivateChats {}


impl NotificationSettingsScopePrivateChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationSettingsScopePrivateChats".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



