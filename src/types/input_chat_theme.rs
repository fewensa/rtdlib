use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a chat theme to set
pub trait TDInputChatTheme: Debug + RObject {}

/// Describes a chat theme to set
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputChatTheme {
  #[doc(hidden)]
  _Default(()),
  /// A theme based on an emoji
  Emoji(InputChatThemeEmoji),
  /// A theme based on an upgraded gift
  Gift(InputChatThemeGift),
}

impl Default for InputChatTheme {
  fn default() -> Self {
    InputChatTheme::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputChatTheme {
  fn deserialize<D>(deserializer: D) -> Result<InputChatTheme, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputChatTheme,
      (inputChatThemeEmoji, Emoji);
      (inputChatThemeGift, Gift);

    )(deserializer)
  }
}

impl RObject for InputChatTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputChatTheme::Emoji(t) => t.td_name(),
      InputChatTheme::Gift(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputChatTheme::Emoji(t) => t.extra(),
      InputChatTheme::Gift(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputChatTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputChatTheme::_Default(_))
  }

  pub fn is_emoji(&self) -> bool {
    matches!(self, InputChatTheme::Emoji(_))
  }
  pub fn is_gift(&self) -> bool {
    matches!(self, InputChatTheme::Gift(_))
  }

  pub fn on_emoji<F: FnOnce(&InputChatThemeEmoji)>(&self, fnc: F) -> &Self {
    if let InputChatTheme::Emoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift<F: FnOnce(&InputChatThemeGift)>(&self, fnc: F) -> &Self {
    if let InputChatTheme::Gift(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_emoji(&self) -> Option<&InputChatThemeEmoji> {
    if let InputChatTheme::Emoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift(&self) -> Option<&InputChatThemeGift> {
    if let InputChatTheme::Gift(t) = self {
      return Some(t);
    }
    None
  }

  pub fn emoji<T: AsRef<InputChatThemeEmoji>>(t: T) -> Self {
    InputChatTheme::Emoji(t.as_ref().clone())
  }

  pub fn gift<T: AsRef<InputChatThemeGift>>(t: T) -> Self {
    InputChatTheme::Gift(t.as_ref().clone())
  }
}

impl AsRef<InputChatTheme> for InputChatTheme {
  fn as_ref(&self) -> &InputChatTheme {
    self
  }
}

/// A theme based on an emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChatThemeEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the theme
  name: String,
}

impl RObject for InputChatThemeEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputChatThemeEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputChatTheme for InputChatThemeEmoji {}

impl InputChatThemeEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputChatThemeEmojiBuilder {
    let inner = InputChatThemeEmoji {
      td_name: "inputChatThemeEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputChatThemeEmojiBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDInputChatThemeEmojiBuilder {
  inner: InputChatThemeEmoji,
}

impl RTDInputChatThemeEmojiBuilder {
  pub fn build(&self) -> InputChatThemeEmoji {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<InputChatThemeEmoji> for InputChatThemeEmoji {
  fn as_ref(&self) -> &InputChatThemeEmoji {
    self
  }
}

impl AsRef<InputChatThemeEmoji> for RTDInputChatThemeEmojiBuilder {
  fn as_ref(&self) -> &InputChatThemeEmoji {
    &self.inner
  }
}

/// A theme based on an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChatThemeGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the upgraded gift. A gift can be used only in one chat at a time. When the same gift is used in another chat, theme in the previous chat is reset to default
  name: String,
}

impl RObject for InputChatThemeGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputChatThemeGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputChatTheme for InputChatThemeGift {}

impl InputChatThemeGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputChatThemeGiftBuilder {
    let inner = InputChatThemeGift {
      td_name: "inputChatThemeGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputChatThemeGiftBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDInputChatThemeGiftBuilder {
  inner: InputChatThemeGift,
}

impl RTDInputChatThemeGiftBuilder {
  pub fn build(&self) -> InputChatThemeGift {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<InputChatThemeGift> for InputChatThemeGift {
  fn as_ref(&self) -> &InputChatThemeGift {
    self
  }
}

impl AsRef<InputChatThemeGift> for RTDInputChatThemeGiftBuilder {
  fn as_ref(&self) -> &InputChatThemeGift {
    &self.inner
  }
}
