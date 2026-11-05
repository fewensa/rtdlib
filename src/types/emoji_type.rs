use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes an emoji
pub trait TDEmojiType: Debug + RObject {}

/// Describes an emoji
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EmojiType {
  #[doc(hidden)]
  _Default(()),
  /// A custom emoji
  CustomEmoji(EmojiTypeCustomEmoji),
  /// A regular emoji
  Regular(EmojiTypeRegular),
}

impl Default for EmojiType {
  fn default() -> Self {
    EmojiType::_Default(())
  }
}

impl<'de> Deserialize<'de> for EmojiType {
  fn deserialize<D>(deserializer: D) -> Result<EmojiType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      EmojiType,
      (emojiTypeCustomEmoji, CustomEmoji);
      (emojiTypeRegular, Regular);

    )(deserializer)
  }
}

impl RObject for EmojiType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      EmojiType::CustomEmoji(t) => t.td_name(),
      EmojiType::Regular(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      EmojiType::CustomEmoji(t) => t.extra(),
      EmojiType::Regular(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, EmojiType::_Default(_))
  }

  pub fn is_custom_emoji(&self) -> bool {
    matches!(self, EmojiType::CustomEmoji(_))
  }
  pub fn is_regular(&self) -> bool {
    matches!(self, EmojiType::Regular(_))
  }

  pub fn on_custom_emoji<F: FnOnce(&EmojiTypeCustomEmoji)>(&self, fnc: F) -> &Self {
    if let EmojiType::CustomEmoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_regular<F: FnOnce(&EmojiTypeRegular)>(&self, fnc: F) -> &Self {
    if let EmojiType::Regular(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_custom_emoji(&self) -> Option<&EmojiTypeCustomEmoji> {
    if let EmojiType::CustomEmoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_regular(&self) -> Option<&EmojiTypeRegular> {
    if let EmojiType::Regular(t) = self {
      return Some(t);
    }
    None
  }

  pub fn custom_emoji<T: AsRef<EmojiTypeCustomEmoji>>(t: T) -> Self {
    EmojiType::CustomEmoji(t.as_ref().clone())
  }

  pub fn regular<T: AsRef<EmojiTypeRegular>>(t: T) -> Self {
    EmojiType::Regular(t.as_ref().clone())
  }
}

impl AsRef<EmojiType> for EmojiType {
  fn as_ref(&self) -> &EmojiType {
    self
  }
}

/// A custom emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiTypeCustomEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the custom emoji
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_id: isize,
}

impl RObject for EmojiTypeCustomEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiTypeCustomEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiType for EmojiTypeCustomEmoji {}

impl EmojiTypeCustomEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiTypeCustomEmojiBuilder {
    let inner = EmojiTypeCustomEmoji {
      td_name: "emojiTypeCustomEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiTypeCustomEmojiBuilder { inner }
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDEmojiTypeCustomEmojiBuilder {
  inner: EmojiTypeCustomEmoji,
}

impl RTDEmojiTypeCustomEmojiBuilder {
  pub fn build(&self) -> EmojiTypeCustomEmoji {
    self.inner.clone()
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }
}

impl AsRef<EmojiTypeCustomEmoji> for EmojiTypeCustomEmoji {
  fn as_ref(&self) -> &EmojiTypeCustomEmoji {
    self
  }
}

impl AsRef<EmojiTypeCustomEmoji> for RTDEmojiTypeCustomEmojiBuilder {
  fn as_ref(&self) -> &EmojiTypeCustomEmoji {
    &self.inner
  }
}

/// A regular emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text representation of the emoji
  emoji: String,
}

impl RObject for EmojiTypeRegular {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiTypeRegular"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiType for EmojiTypeRegular {}

impl EmojiTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiTypeRegularBuilder {
    let inner = EmojiTypeRegular {
      td_name: "emojiTypeRegular".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiTypeRegularBuilder { inner }
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }
}

#[doc(hidden)]
pub struct RTDEmojiTypeRegularBuilder {
  inner: EmojiTypeRegular,
}

impl RTDEmojiTypeRegularBuilder {
  pub fn build(&self) -> EmojiTypeRegular {
    self.inner.clone()
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }
}

impl AsRef<EmojiTypeRegular> for EmojiTypeRegular {
  fn as_ref(&self) -> &EmojiTypeRegular {
    self
  }
}

impl AsRef<EmojiTypeRegular> for RTDEmojiTypeRegularBuilder {
  fn as_ref(&self) -> &EmojiTypeRegular {
    &self.inner
  }
}
