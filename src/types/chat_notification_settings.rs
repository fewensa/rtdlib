
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about notification settings for a chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatNotificationSettings
  /// If true, mute_for is ignored and the value for the relevant type of chat is used instead.
  use_default_mute_for: Option<bool>,
  /// Time left before notifications will be unmuted, in seconds.
  mute_for: Option<i32>,
  /// If true, sound is ignored and the value for the relevant type of chat is used instead.
  use_default_sound: Option<bool>,
  /// The name of an audio file to be used for notification sounds; only applies to iOS applications.
  sound: Option<String>,
  /// If true, show_preview is ignored and the value for the relevant type of chat is used instead.
  use_default_show_preview: Option<bool>,
  /// True, if message content should be displayed in notifications.
  show_preview: Option<bool>,
  /// If true, disable_pinned_message_notifications is ignored and the value for the relevant type of chat is used instead.
  use_default_disable_pinned_message_notifications: Option<bool>,
  /// If true, notifications for incoming pinned messages will be created as for an ordinary unread message.
  disable_pinned_message_notifications: Option<bool>,
  /// If true, disable_mention_notifications is ignored and the value for the relevant type of chat is used instead.
  use_default_disable_mention_notifications: Option<bool>,
  /// If true, notifications for messages with mentions will be created as for an ordinary unread message.
  disable_mention_notifications: Option<bool>,
  
}



impl Object for ChatNotificationSettings {}
impl RObject for ChatNotificationSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatNotificationSettings" }
  fn td_type(&self) -> RTDType { RTDType::ChatNotificationSettings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatNotificationSettings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatNotificationSettings".to_string(),
      use_default_mute_for: None,
      mute_for: None,
      use_default_sound: None,
      sound: None,
      use_default_show_preview: None,
      show_preview: None,
      use_default_disable_pinned_message_notifications: None,
      disable_pinned_message_notifications: None,
      use_default_disable_mention_notifications: None,
      disable_mention_notifications: None,
      
    }
  }
  
  pub fn use_default_mute_for(&self) -> Option<bool> { self.use_default_mute_for.clone() }
  #[doc(hidden)] pub fn _set_use_default_mute_for(&mut self, use_default_mute_for: bool) -> &mut Self { self.use_default_mute_for = Some(use_default_mute_for); self }
  
  pub fn mute_for(&self) -> Option<i32> { self.mute_for.clone() }
  #[doc(hidden)] pub fn _set_mute_for(&mut self, mute_for: i32) -> &mut Self { self.mute_for = Some(mute_for); self }
  
  pub fn use_default_sound(&self) -> Option<bool> { self.use_default_sound.clone() }
  #[doc(hidden)] pub fn _set_use_default_sound(&mut self, use_default_sound: bool) -> &mut Self { self.use_default_sound = Some(use_default_sound); self }
  
  pub fn sound(&self) -> Option<String> { self.sound.clone() }
  #[doc(hidden)] pub fn _set_sound(&mut self, sound: String) -> &mut Self { self.sound = Some(sound); self }
  
  pub fn use_default_show_preview(&self) -> Option<bool> { self.use_default_show_preview.clone() }
  #[doc(hidden)] pub fn _set_use_default_show_preview(&mut self, use_default_show_preview: bool) -> &mut Self { self.use_default_show_preview = Some(use_default_show_preview); self }
  
  pub fn show_preview(&self) -> Option<bool> { self.show_preview.clone() }
  #[doc(hidden)] pub fn _set_show_preview(&mut self, show_preview: bool) -> &mut Self { self.show_preview = Some(show_preview); self }
  
  pub fn use_default_disable_pinned_message_notifications(&self) -> Option<bool> { self.use_default_disable_pinned_message_notifications.clone() }
  #[doc(hidden)] pub fn _set_use_default_disable_pinned_message_notifications(&mut self, use_default_disable_pinned_message_notifications: bool) -> &mut Self { self.use_default_disable_pinned_message_notifications = Some(use_default_disable_pinned_message_notifications); self }
  
  pub fn disable_pinned_message_notifications(&self) -> Option<bool> { self.disable_pinned_message_notifications.clone() }
  #[doc(hidden)] pub fn _set_disable_pinned_message_notifications(&mut self, disable_pinned_message_notifications: bool) -> &mut Self { self.disable_pinned_message_notifications = Some(disable_pinned_message_notifications); self }
  
  pub fn use_default_disable_mention_notifications(&self) -> Option<bool> { self.use_default_disable_mention_notifications.clone() }
  #[doc(hidden)] pub fn _set_use_default_disable_mention_notifications(&mut self, use_default_disable_mention_notifications: bool) -> &mut Self { self.use_default_disable_mention_notifications = Some(use_default_disable_mention_notifications); self }
  
  pub fn disable_mention_notifications(&self) -> Option<bool> { self.disable_mention_notifications.clone() }
  #[doc(hidden)] pub fn _set_disable_mention_notifications(&mut self, disable_mention_notifications: bool) -> &mut Self { self.disable_mention_notifications = Some(disable_mention_notifications); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



