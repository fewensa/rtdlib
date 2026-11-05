use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of features available on the first chat boost levels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostFeatures {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of features
  features: Vec<ChatBoostLevelFeatures>,
  /// The minimum boost level required to set custom emoji for profile background
  min_profile_background_custom_emoji_boost_level: i64,
  /// The minimum boost level required to set custom emoji for reply header and link preview background; for channel chats only
  min_background_custom_emoji_boost_level: i64,
  /// The minimum boost level required to set emoji status
  min_emoji_status_boost_level: i64,
  /// The minimum boost level required to set a chat theme background as chat background
  min_chat_theme_background_boost_level: i64,
  /// The minimum boost level required to set custom chat background
  min_custom_background_boost_level: i64,
  /// The minimum boost level required to set custom emoji sticker set for the chat; for supergroup chats only
  min_custom_emoji_sticker_set_boost_level: i64,
  /// The minimum boost level allowing to enable automatic translation of messages for non-Premium users; for channel chats only
  min_automatic_translation_boost_level: i64,
  /// The minimum boost level allowing to recognize speech in video note and voice note messages for non-Premium users; for supergroup chats only
  min_speech_recognition_boost_level: i64,
  /// The minimum boost level allowing to disable sponsored messages in the chat; for channel chats only
  min_sponsored_message_disable_boost_level: i64,
}

impl RObject for ChatBoostFeatures {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostFeatures"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoostFeatures {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostFeaturesBuilder {
    let inner = ChatBoostFeatures {
      td_name: "chatBoostFeatures".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostFeaturesBuilder { inner }
  }

  pub fn features(&self) -> &Vec<ChatBoostLevelFeatures> {
    &self.features
  }

  pub fn min_profile_background_custom_emoji_boost_level(&self) -> i64 {
    self.min_profile_background_custom_emoji_boost_level
  }

  pub fn min_background_custom_emoji_boost_level(&self) -> i64 {
    self.min_background_custom_emoji_boost_level
  }

  pub fn min_emoji_status_boost_level(&self) -> i64 {
    self.min_emoji_status_boost_level
  }

  pub fn min_chat_theme_background_boost_level(&self) -> i64 {
    self.min_chat_theme_background_boost_level
  }

  pub fn min_custom_background_boost_level(&self) -> i64 {
    self.min_custom_background_boost_level
  }

  pub fn min_custom_emoji_sticker_set_boost_level(&self) -> i64 {
    self.min_custom_emoji_sticker_set_boost_level
  }

  pub fn min_automatic_translation_boost_level(&self) -> i64 {
    self.min_automatic_translation_boost_level
  }

  pub fn min_speech_recognition_boost_level(&self) -> i64 {
    self.min_speech_recognition_boost_level
  }

  pub fn min_sponsored_message_disable_boost_level(&self) -> i64 {
    self.min_sponsored_message_disable_boost_level
  }
}

#[doc(hidden)]
pub struct RTDChatBoostFeaturesBuilder {
  inner: ChatBoostFeatures,
}

impl RTDChatBoostFeaturesBuilder {
  pub fn build(&self) -> ChatBoostFeatures {
    self.inner.clone()
  }

  pub fn features(&mut self, features: Vec<ChatBoostLevelFeatures>) -> &mut Self {
    self.inner.features = features;
    self
  }

  pub fn min_profile_background_custom_emoji_boost_level(
    &mut self,
    min_profile_background_custom_emoji_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_profile_background_custom_emoji_boost_level =
      min_profile_background_custom_emoji_boost_level;
    self
  }

  pub fn min_background_custom_emoji_boost_level(
    &mut self,
    min_background_custom_emoji_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_background_custom_emoji_boost_level = min_background_custom_emoji_boost_level;
    self
  }

  pub fn min_emoji_status_boost_level(&mut self, min_emoji_status_boost_level: i64) -> &mut Self {
    self.inner.min_emoji_status_boost_level = min_emoji_status_boost_level;
    self
  }

  pub fn min_chat_theme_background_boost_level(
    &mut self,
    min_chat_theme_background_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_chat_theme_background_boost_level = min_chat_theme_background_boost_level;
    self
  }

  pub fn min_custom_background_boost_level(
    &mut self,
    min_custom_background_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_custom_background_boost_level = min_custom_background_boost_level;
    self
  }

  pub fn min_custom_emoji_sticker_set_boost_level(
    &mut self,
    min_custom_emoji_sticker_set_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_custom_emoji_sticker_set_boost_level = min_custom_emoji_sticker_set_boost_level;
    self
  }

  pub fn min_automatic_translation_boost_level(
    &mut self,
    min_automatic_translation_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_automatic_translation_boost_level = min_automatic_translation_boost_level;
    self
  }

  pub fn min_speech_recognition_boost_level(
    &mut self,
    min_speech_recognition_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_speech_recognition_boost_level = min_speech_recognition_boost_level;
    self
  }

  pub fn min_sponsored_message_disable_boost_level(
    &mut self,
    min_sponsored_message_disable_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_sponsored_message_disable_boost_level =
      min_sponsored_message_disable_boost_level;
    self
  }
}

impl AsRef<ChatBoostFeatures> for ChatBoostFeatures {
  fn as_ref(&self) -> &ChatBoostFeatures {
    self
  }
}

impl AsRef<ChatBoostFeatures> for RTDChatBoostFeaturesBuilder {
  fn as_ref(&self) -> &ChatBoostFeatures {
    &self.inner
  }
}
