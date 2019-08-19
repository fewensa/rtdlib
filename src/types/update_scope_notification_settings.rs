
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Notification settings for some type of chats were updated. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateScopeNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateScopeNotificationSettings
  /// Types of chats for which notification settings were updated.
  scope: Option<Box<NotificationSettingsScope>>,
  /// The new notification settings.
  notification_settings: Option<ScopeNotificationSettings>,
  
}


impl Clone for UpdateScopeNotificationSettings {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateScopeNotificationSettings {}
impl RObject for UpdateScopeNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateScopeNotificationSettings" }
  fn td_type(&self) -> RTDType { RTDType::UpdateScopeNotificationSettings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateScopeNotificationSettings {}


impl UpdateScopeNotificationSettings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateScopeNotificationSettings".to_string(),
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



