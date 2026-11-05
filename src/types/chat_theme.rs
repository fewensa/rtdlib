use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a chat theme
pub trait TDChatTheme: Debug + RObject {}

/// Describes a chat theme
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatTheme {
  #[doc(hidden)]
  _Default(()),
  /// A chat theme based on an emoji
  Emoji(ChatThemeEmoji),
  /// A chat theme based on an upgraded gift
  Gift(ChatThemeGift),
}

impl Default for ChatTheme {
  fn default() -> Self {
    ChatTheme::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatTheme {
  fn deserialize<D>(deserializer: D) -> Result<ChatTheme, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatTheme,
      (chatThemeEmoji, Emoji);
      (chatThemeGift, Gift);

    )(deserializer)
  }
}

impl RObject for ChatTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatTheme::Emoji(t) => t.td_name(),
      ChatTheme::Gift(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatTheme::Emoji(t) => t.extra(),
      ChatTheme::Gift(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatTheme::_Default(_))
  }

  pub fn is_emoji(&self) -> bool {
    matches!(self, ChatTheme::Emoji(_))
  }
  pub fn is_gift(&self) -> bool {
    matches!(self, ChatTheme::Gift(_))
  }

  pub fn on_emoji<F: FnOnce(&ChatThemeEmoji)>(&self, fnc: F) -> &Self {
    if let ChatTheme::Emoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift<F: FnOnce(&ChatThemeGift)>(&self, fnc: F) -> &Self {
    if let ChatTheme::Gift(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_emoji(&self) -> Option<&ChatThemeEmoji> {
    if let ChatTheme::Emoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift(&self) -> Option<&ChatThemeGift> {
    if let ChatTheme::Gift(t) = self {
      return Some(t);
    }
    None
  }

  pub fn emoji<T: AsRef<ChatThemeEmoji>>(t: T) -> Self {
    ChatTheme::Emoji(t.as_ref().clone())
  }

  pub fn gift<T: AsRef<ChatThemeGift>>(t: T) -> Self {
    ChatTheme::Gift(t.as_ref().clone())
  }
}

impl AsRef<ChatTheme> for ChatTheme {
  fn as_ref(&self) -> &ChatTheme {
    self
  }
}

/// A chat theme based on an emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatThemeEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the theme; full theme description is received through updateEmojiChatThemes
  name: String,
}

impl RObject for ChatThemeEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatThemeEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatTheme for ChatThemeEmoji {}

impl ChatThemeEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatThemeEmojiBuilder {
    let inner = ChatThemeEmoji {
      td_name: "chatThemeEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatThemeEmojiBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDChatThemeEmojiBuilder {
  inner: ChatThemeEmoji,
}

impl RTDChatThemeEmojiBuilder {
  pub fn build(&self) -> ChatThemeEmoji {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<ChatThemeEmoji> for ChatThemeEmoji {
  fn as_ref(&self) -> &ChatThemeEmoji {
    self
  }
}

impl AsRef<ChatThemeEmoji> for RTDChatThemeEmojiBuilder {
  fn as_ref(&self) -> &ChatThemeEmoji {
    &self.inner
  }
}

/// A chat theme based on an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatThemeGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat theme
  gift_theme: GiftChatTheme,
}

impl RObject for ChatThemeGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatThemeGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatTheme for ChatThemeGift {}

impl ChatThemeGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatThemeGiftBuilder {
    let inner = ChatThemeGift {
      td_name: "chatThemeGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatThemeGiftBuilder { inner }
  }

  pub fn gift_theme(&self) -> &GiftChatTheme {
    &self.gift_theme
  }
}

#[doc(hidden)]
pub struct RTDChatThemeGiftBuilder {
  inner: ChatThemeGift,
}

impl RTDChatThemeGiftBuilder {
  pub fn build(&self) -> ChatThemeGift {
    self.inner.clone()
  }

  pub fn gift_theme<T: AsRef<GiftChatTheme>>(&mut self, gift_theme: T) -> &mut Self {
    self.inner.gift_theme = gift_theme.as_ref().clone();
    self
  }
}

impl AsRef<ChatThemeGift> for ChatThemeGift {
  fn as_ref(&self) -> &ChatThemeGift {
    self
  }
}

impl AsRef<ChatThemeGift> for RTDChatThemeGiftBuilder {
  fn as_ref(&self) -> &ChatThemeGift {
    &self.inner
  }
}
