
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the notification settings for chats of a given type.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetScopeNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getScopeNotificationSettings
  /// Types of chats for which to return the notification settings information.
  scope: Option<Box<NotificationSettingsScope>>,
  
}


impl Clone for GetScopeNotificationSettings {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetScopeNotificationSettings {}
impl RObject for GetScopeNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getScopeNotificationSettings" }
  fn td_type(&self) -> RTDType { RTDType::GetScopeNotificationSettings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetScopeNotificationSettings {}


impl GetScopeNotificationSettings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getScopeNotificationSettings".to_string(),
      scope: None,
      
    }
  }
  
  pub fn scope(&self) -> Option<Box<NotificationSettingsScope>> { self.scope.clone() }
  #[doc(hidden)] pub fn _set_scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self { self.scope = Some(scope); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



