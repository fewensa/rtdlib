
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about notification settings for several chats. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // scopeNotificationSettings
  /// Time left before notifications will be unmuted, in seconds.
  mute_for: Option<i32>,
  /// The name of an audio file to be used for notification sounds; only applies to iOS applications.
  sound: Option<String>,
  /// True, if message content should be displayed in notifications.
  show_preview: Option<bool>,
  /// True, if notifications for incoming pinned messages will be created as for an ordinary unread message.
  disable_pinned_message_notifications: Option<bool>,
  /// True, if notifications for messages with mentions will be created as for an ordinary unread message.
  disable_mention_notifications: Option<bool>,
  
}



impl Object for ScopeNotificationSettings {}
impl RObject for ScopeNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "scopeNotificationSettings" }
  fn td_type(&self) -> RTDType { RTDType::ScopeNotificationSettings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ScopeNotificationSettings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "scopeNotificationSettings".to_string(),
      mute_for: None,
      sound: None,
      show_preview: None,
      disable_pinned_message_notifications: None,
      disable_mention_notifications: None,
      
    }
  }
  
  pub fn mute_for(&self) -> Option<i32> { self.mute_for.clone() }
  #[doc(hidden)] pub fn _set_mute_for(&mut self, mute_for: i32) -> &mut Self { self.mute_for = Some(mute_for); self }
  
  pub fn sound(&self) -> Option<String> { self.sound.clone() }
  #[doc(hidden)] pub fn _set_sound(&mut self, sound: String) -> &mut Self { self.sound = Some(sound); self }
  
  pub fn show_preview(&self) -> Option<bool> { self.show_preview.clone() }
  #[doc(hidden)] pub fn _set_show_preview(&mut self, show_preview: bool) -> &mut Self { self.show_preview = Some(show_preview); self }
  
  pub fn disable_pinned_message_notifications(&self) -> Option<bool> { self.disable_pinned_message_notifications.clone() }
  #[doc(hidden)] pub fn _set_disable_pinned_message_notifications(&mut self, disable_pinned_message_notifications: bool) -> &mut Self { self.disable_pinned_message_notifications = Some(disable_pinned_message_notifications); self }
  
  pub fn disable_mention_notifications(&self) -> Option<bool> { self.disable_mention_notifications.clone() }
  #[doc(hidden)] pub fn _set_disable_mention_notifications(&mut self, disable_mention_notifications: bool) -> &mut Self { self.disable_mention_notifications = Some(disable_mention_notifications); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



