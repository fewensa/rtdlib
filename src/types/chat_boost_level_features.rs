use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of features available on a specific chat boost level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostLevelFeatures {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Target chat boost level
  level: i64,
  /// Number of stories that the chat can publish daily
  story_per_day_count: i64,
  /// Number of custom emoji reactions that can be added to the list of available reactions
  custom_emoji_reaction_count: i64,
  /// Number of custom colors for chat title
  title_color_count: i64,
  /// Number of custom colors for profile photo background
  profile_accent_color_count: i64,
  /// True, if custom emoji for profile background can be set
  can_set_profile_background_custom_emoji: bool,
  /// Number of custom colors for background of empty chat photo, replies to messages and link previews
  accent_color_count: i64,
  /// True, if custom emoji for reply header and link preview background can be set
  can_set_background_custom_emoji: bool,
  /// True, if emoji status can be set
  can_set_emoji_status: bool,
  /// Number of chat theme backgrounds that can be set as chat background
  chat_theme_background_count: i64,
  /// True, if custom background can be set in the chat for all users
  can_set_custom_background: bool,
  /// True, if custom emoji sticker set can be set for the chat
  can_set_custom_emoji_sticker_set: bool,
  /// True, if automatic translation of messages can be enabled in the chat
  can_enable_automatic_translation: bool,
  /// True, if speech recognition can be used for video note and voice note messages by all users
  can_recognize_speech: bool,
  /// True, if sponsored messages can be disabled in the chat
  can_disable_sponsored_messages: bool,
}

impl RObject for ChatBoostLevelFeatures {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostLevelFeatures"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoostLevelFeatures {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostLevelFeaturesBuilder {
    let inner = ChatBoostLevelFeatures {
      td_name: "chatBoostLevelFeatures".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostLevelFeaturesBuilder { inner }
  }

  pub fn level(&self) -> i64 {
    self.level
  }

  pub fn story_per_day_count(&self) -> i64 {
    self.story_per_day_count
  }

  pub fn custom_emoji_reaction_count(&self) -> i64 {
    self.custom_emoji_reaction_count
  }

  pub fn title_color_count(&self) -> i64 {
    self.title_color_count
  }

  pub fn profile_accent_color_count(&self) -> i64 {
    self.profile_accent_color_count
  }

  pub fn can_set_profile_background_custom_emoji(&self) -> bool {
    self.can_set_profile_background_custom_emoji
  }

  pub fn accent_color_count(&self) -> i64 {
    self.accent_color_count
  }

  pub fn can_set_background_custom_emoji(&self) -> bool {
    self.can_set_background_custom_emoji
  }

  pub fn can_set_emoji_status(&self) -> bool {
    self.can_set_emoji_status
  }

  pub fn chat_theme_background_count(&self) -> i64 {
    self.chat_theme_background_count
  }

  pub fn can_set_custom_background(&self) -> bool {
    self.can_set_custom_background
  }

  pub fn can_set_custom_emoji_sticker_set(&self) -> bool {
    self.can_set_custom_emoji_sticker_set
  }

  pub fn can_enable_automatic_translation(&self) -> bool {
    self.can_enable_automatic_translation
  }

  pub fn can_recognize_speech(&self) -> bool {
    self.can_recognize_speech
  }

  pub fn can_disable_sponsored_messages(&self) -> bool {
    self.can_disable_sponsored_messages
  }
}

#[doc(hidden)]
pub struct RTDChatBoostLevelFeaturesBuilder {
  inner: ChatBoostLevelFeatures,
}

impl RTDChatBoostLevelFeaturesBuilder {
  pub fn build(&self) -> ChatBoostLevelFeatures {
    self.inner.clone()
  }

  pub fn level(&mut self, level: i64) -> &mut Self {
    self.inner.level = level;
    self
  }

  pub fn story_per_day_count(&mut self, story_per_day_count: i64) -> &mut Self {
    self.inner.story_per_day_count = story_per_day_count;
    self
  }

  pub fn custom_emoji_reaction_count(&mut self, custom_emoji_reaction_count: i64) -> &mut Self {
    self.inner.custom_emoji_reaction_count = custom_emoji_reaction_count;
    self
  }

  pub fn title_color_count(&mut self, title_color_count: i64) -> &mut Self {
    self.inner.title_color_count = title_color_count;
    self
  }

  pub fn profile_accent_color_count(&mut self, profile_accent_color_count: i64) -> &mut Self {
    self.inner.profile_accent_color_count = profile_accent_color_count;
    self
  }

  pub fn can_set_profile_background_custom_emoji(
    &mut self,
    can_set_profile_background_custom_emoji: bool,
  ) -> &mut Self {
    self.inner.can_set_profile_background_custom_emoji = can_set_profile_background_custom_emoji;
    self
  }

  pub fn accent_color_count(&mut self, accent_color_count: i64) -> &mut Self {
    self.inner.accent_color_count = accent_color_count;
    self
  }

  pub fn can_set_background_custom_emoji(
    &mut self,
    can_set_background_custom_emoji: bool,
  ) -> &mut Self {
    self.inner.can_set_background_custom_emoji = can_set_background_custom_emoji;
    self
  }

  pub fn can_set_emoji_status(&mut self, can_set_emoji_status: bool) -> &mut Self {
    self.inner.can_set_emoji_status = can_set_emoji_status;
    self
  }

  pub fn chat_theme_background_count(&mut self, chat_theme_background_count: i64) -> &mut Self {
    self.inner.chat_theme_background_count = chat_theme_background_count;
    self
  }

  pub fn can_set_custom_background(&mut self, can_set_custom_background: bool) -> &mut Self {
    self.inner.can_set_custom_background = can_set_custom_background;
    self
  }

  pub fn can_set_custom_emoji_sticker_set(
    &mut self,
    can_set_custom_emoji_sticker_set: bool,
  ) -> &mut Self {
    self.inner.can_set_custom_emoji_sticker_set = can_set_custom_emoji_sticker_set;
    self
  }

  pub fn can_enable_automatic_translation(
    &mut self,
    can_enable_automatic_translation: bool,
  ) -> &mut Self {
    self.inner.can_enable_automatic_translation = can_enable_automatic_translation;
    self
  }

  pub fn can_recognize_speech(&mut self, can_recognize_speech: bool) -> &mut Self {
    self.inner.can_recognize_speech = can_recognize_speech;
    self
  }

  pub fn can_disable_sponsored_messages(
    &mut self,
    can_disable_sponsored_messages: bool,
  ) -> &mut Self {
    self.inner.can_disable_sponsored_messages = can_disable_sponsored_messages;
    self
  }
}

impl AsRef<ChatBoostLevelFeatures> for ChatBoostLevelFeatures {
  fn as_ref(&self) -> &ChatBoostLevelFeatures {
    self
  }
}

impl AsRef<ChatBoostLevelFeatures> for RTDChatBoostLevelFeaturesBuilder {
  fn as_ref(&self) -> &ChatBoostLevelFeatures {
    &self.inner
  }
}
