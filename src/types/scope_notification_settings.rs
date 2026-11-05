use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about notification settings for several chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScopeNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Time left before notifications will be unmuted, in seconds
  mute_for: i64,
  /// Identifier of the notification sound to be played; 0 if sound is disabled; pass 1 to use the app-dependent default sound
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  sound_id: isize,
  /// True, if message content must be displayed in notifications
  show_preview: bool,
  /// If true, story notifications are received only for the first 5 chats from topChatCategoryUsers regardless of the value of mute_stories
  use_default_mute_stories: bool,
  /// True, if story notifications are disabled
  mute_stories: bool,
  /// Identifier of the notification sound to be played for stories; 0 if sound is disabled; pass 1 to use the app-dependent default sound
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  story_sound_id: isize,
  /// True, if the chat that posted a story must be displayed in notifications
  show_story_poster: bool,
  /// True, if notifications for incoming pinned messages will be created as for an ordinary unread message
  disable_pinned_message_notifications: bool,
  /// True, if notifications for messages with mentions will be created as for an ordinary unread message
  disable_mention_notifications: bool,
}

impl RObject for ScopeNotificationSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "scopeNotificationSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ScopeNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDScopeNotificationSettingsBuilder {
    let inner = ScopeNotificationSettings {
      td_name: "scopeNotificationSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDScopeNotificationSettingsBuilder { inner }
  }

  pub fn mute_for(&self) -> i64 {
    self.mute_for
  }

  pub fn sound_id(&self) -> isize {
    self.sound_id
  }

  pub fn show_preview(&self) -> bool {
    self.show_preview
  }

  pub fn use_default_mute_stories(&self) -> bool {
    self.use_default_mute_stories
  }

  pub fn mute_stories(&self) -> bool {
    self.mute_stories
  }

  pub fn story_sound_id(&self) -> isize {
    self.story_sound_id
  }

  pub fn show_story_poster(&self) -> bool {
    self.show_story_poster
  }

  pub fn disable_pinned_message_notifications(&self) -> bool {
    self.disable_pinned_message_notifications
  }

  pub fn disable_mention_notifications(&self) -> bool {
    self.disable_mention_notifications
  }
}

#[doc(hidden)]
pub struct RTDScopeNotificationSettingsBuilder {
  inner: ScopeNotificationSettings,
}

impl RTDScopeNotificationSettingsBuilder {
  pub fn build(&self) -> ScopeNotificationSettings {
    self.inner.clone()
  }

  pub fn mute_for(&mut self, mute_for: i64) -> &mut Self {
    self.inner.mute_for = mute_for;
    self
  }

  pub fn sound_id(&mut self, sound_id: isize) -> &mut Self {
    self.inner.sound_id = sound_id;
    self
  }

  pub fn show_preview(&mut self, show_preview: bool) -> &mut Self {
    self.inner.show_preview = show_preview;
    self
  }

  pub fn use_default_mute_stories(&mut self, use_default_mute_stories: bool) -> &mut Self {
    self.inner.use_default_mute_stories = use_default_mute_stories;
    self
  }

  pub fn mute_stories(&mut self, mute_stories: bool) -> &mut Self {
    self.inner.mute_stories = mute_stories;
    self
  }

  pub fn story_sound_id(&mut self, story_sound_id: isize) -> &mut Self {
    self.inner.story_sound_id = story_sound_id;
    self
  }

  pub fn show_story_poster(&mut self, show_story_poster: bool) -> &mut Self {
    self.inner.show_story_poster = show_story_poster;
    self
  }

  pub fn disable_pinned_message_notifications(
    &mut self,
    disable_pinned_message_notifications: bool,
  ) -> &mut Self {
    self.inner.disable_pinned_message_notifications = disable_pinned_message_notifications;
    self
  }

  pub fn disable_mention_notifications(
    &mut self,
    disable_mention_notifications: bool,
  ) -> &mut Self {
    self.inner.disable_mention_notifications = disable_mention_notifications;
    self
  }
}

impl AsRef<ScopeNotificationSettings> for ScopeNotificationSettings {
  fn as_ref(&self) -> &ScopeNotificationSettings {
    self
  }
}

impl AsRef<ScopeNotificationSettings> for RTDScopeNotificationSettingsBuilder {
  fn as_ref(&self) -> &ScopeNotificationSettings {
    &self.inner
  }
}
