use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat theme based on an emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiChatTheme {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Theme name
  name: String,
  /// Theme settings for a light chat theme
  light_settings: ThemeSettings,
  /// Theme settings for a dark chat theme
  dark_settings: ThemeSettings,
}

impl RObject for EmojiChatTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiChatTheme"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiChatTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiChatThemeBuilder {
    let inner = EmojiChatTheme {
      td_name: "emojiChatTheme".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiChatThemeBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn light_settings(&self) -> &ThemeSettings {
    &self.light_settings
  }

  pub fn dark_settings(&self) -> &ThemeSettings {
    &self.dark_settings
  }
}

#[doc(hidden)]
pub struct RTDEmojiChatThemeBuilder {
  inner: EmojiChatTheme,
}

impl RTDEmojiChatThemeBuilder {
  pub fn build(&self) -> EmojiChatTheme {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn light_settings<T: AsRef<ThemeSettings>>(&mut self, light_settings: T) -> &mut Self {
    self.inner.light_settings = light_settings.as_ref().clone();
    self
  }

  pub fn dark_settings<T: AsRef<ThemeSettings>>(&mut self, dark_settings: T) -> &mut Self {
    self.inner.dark_settings = dark_settings.as_ref().clone();
    self
  }
}

impl AsRef<EmojiChatTheme> for EmojiChatTheme {
  fn as_ref(&self) -> &EmojiChatTheme {
    self
  }
}

impl AsRef<EmojiChatTheme> for RTDEmojiChatThemeBuilder {
  fn as_ref(&self) -> &EmojiChatTheme {
    &self.inner
  }
}
