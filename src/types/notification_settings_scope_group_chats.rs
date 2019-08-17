
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Notification settings applied to all basic groups and supergroups when the corresponding chat setting has a default value. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettingsScopeGroupChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationSettingsScopeGroupChats
  
}



impl Object for NotificationSettingsScopeGroupChats {}
impl RObject for NotificationSettingsScopeGroupChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationSettingsScopeGroupChats" }
  fn td_type(&self) -> RTDType { RTDType::NotificationSettingsScopeGroupChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationSettingsScope for NotificationSettingsScopeGroupChats {}


impl NotificationSettingsScopeGroupChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationSettingsScopeGroupChats".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



