
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Notification settings applied to all channels when the corresponding chat setting has a default value. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettingsScopeChannelChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationSettingsScopeChannelChats
  
}



impl Object for NotificationSettingsScopeChannelChats {}
impl RObject for NotificationSettingsScopeChannelChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationSettingsScopeChannelChats" }
  fn td_type(&self) -> RTDType { RTDType::NotificationSettingsScopeChannelChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationSettingsScope for NotificationSettingsScopeChannelChats {}


impl NotificationSettingsScopeChannelChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationSettingsScopeChannelChats".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



