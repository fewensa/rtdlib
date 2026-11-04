use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about notification settings for a chat or a forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatNotificationSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// If true, the value for the relevant type of chat or the forum chat is used instead of mute_for
  use_default_mute_for: bool,
  /// Time left before notifications will be unmuted, in seconds
  mute_for: i64,
  /// If true, the value for the relevant type of chat or the forum chat is used instead of sound_id
  use_default_sound: bool,
  /// Identifier of the notification sound to be played for messages; 0 if sound is disabled
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  sound_id: isize,
  /// If true, the value for the relevant type of chat or the forum chat is used instead of show_preview
  use_default_show_preview: bool,
  /// True, if message content must be displayed in notifications
  show_preview: bool,
  /// If true, the value for the relevant type of chat is used instead of mute_stories
  use_default_mute_stories: bool,
  /// True, if story notifications are disabled for the chat
  mute_stories: bool,
  /// If true, the value for the relevant type of chat is used instead of story_sound_id
  use_default_story_sound: bool,
  /// Identifier of the notification sound to be played for stories; 0 if sound is disabled
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  story_sound_id: isize,
  /// If true, the value for the relevant type of chat is used instead of show_story_poster
  use_default_show_story_poster: bool,
  /// True, if the chat that posted a story must be displayed in notifications
  show_story_poster: bool,
  /// If true, the value for the relevant type of chat or the forum chat is used instead of disable_pinned_message_notifications
  use_default_disable_pinned_message_notifications: bool,
  /// If true, notifications for incoming pinned messages will be created as for an ordinary unread message
  disable_pinned_message_notifications: bool,
  /// If true, the value for the relevant type of chat or the forum chat is used instead of disable_mention_notifications
  use_default_disable_mention_notifications: bool,
  /// If true, notifications for messages with mentions will be created as for an ordinary unread message
  disable_mention_notifications: bool,
}

impl RObject for ChatNotificationSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatNotificationSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatNotificationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatNotificationSettingsBuilder {
    let inner = ChatNotificationSettings {
      td_name: "chatNotificationSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatNotificationSettingsBuilder { inner }
  }

  pub fn use_default_mute_for(&self) -> bool {
    self.use_default_mute_for
  }

  pub fn mute_for(&self) -> i64 {
    self.mute_for
  }

  pub fn use_default_sound(&self) -> bool {
    self.use_default_sound
  }

  pub fn sound_id(&self) -> isize {
    self.sound_id
  }

  pub fn use_default_show_preview(&self) -> bool {
    self.use_default_show_preview
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

  pub fn use_default_story_sound(&self) -> bool {
    self.use_default_story_sound
  }

  pub fn story_sound_id(&self) -> isize {
    self.story_sound_id
  }

  pub fn use_default_show_story_poster(&self) -> bool {
    self.use_default_show_story_poster
  }

  pub fn show_story_poster(&self) -> bool {
    self.show_story_poster
  }

  pub fn use_default_disable_pinned_message_notifications(&self) -> bool {
    self.use_default_disable_pinned_message_notifications
  }

  pub fn disable_pinned_message_notifications(&self) -> bool {
    self.disable_pinned_message_notifications
  }

  pub fn use_default_disable_mention_notifications(&self) -> bool {
    self.use_default_disable_mention_notifications
  }

  pub fn disable_mention_notifications(&self) -> bool {
    self.disable_mention_notifications
  }
}

#[doc(hidden)]
pub struct RTDChatNotificationSettingsBuilder {
  inner: ChatNotificationSettings,
}

impl RTDChatNotificationSettingsBuilder {
  pub fn build(&self) -> ChatNotificationSettings {
    self.inner.clone()
  }

  pub fn use_default_mute_for(&mut self, use_default_mute_for: bool) -> &mut Self {
    self.inner.use_default_mute_for = use_default_mute_for;
    self
  }

  pub fn mute_for(&mut self, mute_for: i64) -> &mut Self {
    self.inner.mute_for = mute_for;
    self
  }

  pub fn use_default_sound(&mut self, use_default_sound: bool) -> &mut Self {
    self.inner.use_default_sound = use_default_sound;
    self
  }

  pub fn sound_id(&mut self, sound_id: isize) -> &mut Self {
    self.inner.sound_id = sound_id;
    self
  }

  pub fn use_default_show_preview(&mut self, use_default_show_preview: bool) -> &mut Self {
    self.inner.use_default_show_preview = use_default_show_preview;
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

  pub fn use_default_story_sound(&mut self, use_default_story_sound: bool) -> &mut Self {
    self.inner.use_default_story_sound = use_default_story_sound;
    self
  }

  pub fn story_sound_id(&mut self, story_sound_id: isize) -> &mut Self {
    self.inner.story_sound_id = story_sound_id;
    self
  }

  pub fn use_default_show_story_poster(
    &mut self,
    use_default_show_story_poster: bool,
  ) -> &mut Self {
    self.inner.use_default_show_story_poster = use_default_show_story_poster;
    self
  }

  pub fn show_story_poster(&mut self, show_story_poster: bool) -> &mut Self {
    self.inner.show_story_poster = show_story_poster;
    self
  }

  pub fn use_default_disable_pinned_message_notifications(
    &mut self,
    use_default_disable_pinned_message_notifications: bool,
  ) -> &mut Self {
    self.inner.use_default_disable_pinned_message_notifications =
      use_default_disable_pinned_message_notifications;
    self
  }

  pub fn disable_pinned_message_notifications(
    &mut self,
    disable_pinned_message_notifications: bool,
  ) -> &mut Self {
    self.inner.disable_pinned_message_notifications = disable_pinned_message_notifications;
    self
  }

  pub fn use_default_disable_mention_notifications(
    &mut self,
    use_default_disable_mention_notifications: bool,
  ) -> &mut Self {
    self.inner.use_default_disable_mention_notifications =
      use_default_disable_mention_notifications;
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

impl AsRef<ChatNotificationSettings> for ChatNotificationSettings {
  fn as_ref(&self) -> &ChatNotificationSettings {
    self
  }
}

impl AsRef<ChatNotificationSettings> for RTDChatNotificationSettingsBuilder {
  fn as_ref(&self) -> &ChatNotificationSettings {
    &self.inner
  }
}
