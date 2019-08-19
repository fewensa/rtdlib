
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns list of chats with non-default notification settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChatNotificationSettingsExceptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatNotificationSettingsExceptions
  /// If specified, only chats from the specified scope will be returned.
  scope: Option<Box<NotificationSettingsScope>>,
  /// If true, also chats with non-default sound will be returned.
  compare_sound: Option<bool>,
  
}


impl Clone for GetChatNotificationSettingsExceptions {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetChatNotificationSettingsExceptions {}
impl RObject for GetChatNotificationSettingsExceptions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatNotificationSettingsExceptions" }
  fn td_type(&self) -> RTDType { RTDType::GetChatNotificationSettingsExceptions }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatNotificationSettingsExceptions {}


impl GetChatNotificationSettingsExceptions {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatNotificationSettingsExceptions".to_string(),
      scope: None,
      compare_sound: None,
      
    }
  }
  
  pub fn scope(&self) -> Option<Box<NotificationSettingsScope>> { self.scope.clone() }
  #[doc(hidden)] pub fn _set_scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self { self.scope = Some(scope); self }
  
  pub fn compare_sound(&self) -> Option<bool> { self.compare_sound.clone() }
  #[doc(hidden)] pub fn _set_compare_sound(&mut self, compare_sound: bool) -> &mut Self { self.compare_sound = Some(compare_sound); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



