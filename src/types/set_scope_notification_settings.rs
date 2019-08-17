
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes notification settings for chats of a given type.
#[derive(Debug, Serialize, Deserialize)]
pub struct SetScopeNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setScopeNotificationSettings
  /// Types of chats for which to change the notification settings.
  scope: Option<Box<NotificationSettingsScope>>,
  /// The new notification settings for the given scope.
  notification_settings: Option<ScopeNotificationSettings>,
  
}


impl Clone for SetScopeNotificationSettings {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetScopeNotificationSettings {}
impl RObject for SetScopeNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setScopeNotificationSettings" }
  fn td_type(&self) -> RTDType { RTDType::SetScopeNotificationSettings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetScopeNotificationSettings {}


impl SetScopeNotificationSettings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setScopeNotificationSettings".to_string(),
      scope: None,
      notification_settings: None,
      
    }
  }
  
  pub fn scope(&self) -> Option<Box<NotificationSettingsScope>> { self.scope.clone() }
  #[doc(hidden)] pub fn _set_scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self { self.scope = Some(scope); self }
  
  pub fn notification_settings(&self) -> Option<ScopeNotificationSettings> { self.notification_settings.clone() }
  #[doc(hidden)] pub fn _set_notification_settings(&mut self, notification_settings: ScopeNotificationSettings) -> &mut Self { self.notification_settings = Some(notification_settings); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



