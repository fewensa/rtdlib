use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of sticker, which was used to create a chat photo
pub trait TDChatPhotoStickerType: Debug + RObject {}

/// Describes type of sticker, which was used to create a chat photo
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatPhotoStickerType {
  #[doc(hidden)]
  _Default(()),
  /// Information about the custom emoji, which was used to create the chat photo
  CustomEmoji(ChatPhotoStickerTypeCustomEmoji),
  /// Information about the sticker, which was used to create the chat photo
  RegularOrMask(ChatPhotoStickerTypeRegularOrMask),
}

impl Default for ChatPhotoStickerType {
  fn default() -> Self {
    ChatPhotoStickerType::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatPhotoStickerType {
  fn deserialize<D>(deserializer: D) -> Result<ChatPhotoStickerType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatPhotoStickerType,
      (chatPhotoStickerTypeCustomEmoji, CustomEmoji);
      (chatPhotoStickerTypeRegularOrMask, RegularOrMask);

    )(deserializer)
  }
}

impl RObject for ChatPhotoStickerType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatPhotoStickerType::CustomEmoji(t) => t.td_name(),
      ChatPhotoStickerType::RegularOrMask(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatPhotoStickerType::CustomEmoji(t) => t.extra(),
      ChatPhotoStickerType::RegularOrMask(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatPhotoStickerType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatPhotoStickerType::_Default(_))
  }

  pub fn is_custom_emoji(&self) -> bool {
    matches!(self, ChatPhotoStickerType::CustomEmoji(_))
  }
  pub fn is_regular_or_mask(&self) -> bool {
    matches!(self, ChatPhotoStickerType::RegularOrMask(_))
  }

  pub fn on_custom_emoji<F: FnOnce(&ChatPhotoStickerTypeCustomEmoji)>(&self, fnc: F) -> &Self {
    if let ChatPhotoStickerType::CustomEmoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_regular_or_mask<F: FnOnce(&ChatPhotoStickerTypeRegularOrMask)>(&self, fnc: F) -> &Self {
    if let ChatPhotoStickerType::RegularOrMask(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_custom_emoji(&self) -> Option<&ChatPhotoStickerTypeCustomEmoji> {
    if let ChatPhotoStickerType::CustomEmoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_regular_or_mask(&self) -> Option<&ChatPhotoStickerTypeRegularOrMask> {
    if let ChatPhotoStickerType::RegularOrMask(t) = self {
      return Some(t);
    }
    None
  }

  pub fn custom_emoji<T: AsRef<ChatPhotoStickerTypeCustomEmoji>>(t: T) -> Self {
    ChatPhotoStickerType::CustomEmoji(t.as_ref().clone())
  }

  pub fn regular_or_mask<T: AsRef<ChatPhotoStickerTypeRegularOrMask>>(t: T) -> Self {
    ChatPhotoStickerType::RegularOrMask(t.as_ref().clone())
  }
}

impl AsRef<ChatPhotoStickerType> for ChatPhotoStickerType {
  fn as_ref(&self) -> &ChatPhotoStickerType {
    self
  }
}

/// Information about the custom emoji, which was used to create the chat photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotoStickerTypeCustomEmoji {
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

impl RObject for ChatPhotoStickerTypeCustomEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatPhotoStickerTypeCustomEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatPhotoStickerType for ChatPhotoStickerTypeCustomEmoji {}

impl ChatPhotoStickerTypeCustomEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatPhotoStickerTypeCustomEmojiBuilder {
    let inner = ChatPhotoStickerTypeCustomEmoji {
      td_name: "chatPhotoStickerTypeCustomEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatPhotoStickerTypeCustomEmojiBuilder { inner }
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDChatPhotoStickerTypeCustomEmojiBuilder {
  inner: ChatPhotoStickerTypeCustomEmoji,
}

impl RTDChatPhotoStickerTypeCustomEmojiBuilder {
  pub fn build(&self) -> ChatPhotoStickerTypeCustomEmoji {
    self.inner.clone()
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }
}

impl AsRef<ChatPhotoStickerTypeCustomEmoji> for ChatPhotoStickerTypeCustomEmoji {
  fn as_ref(&self) -> &ChatPhotoStickerTypeCustomEmoji {
    self
  }
}

impl AsRef<ChatPhotoStickerTypeCustomEmoji> for RTDChatPhotoStickerTypeCustomEmojiBuilder {
  fn as_ref(&self) -> &ChatPhotoStickerTypeCustomEmoji {
    &self.inner
  }
}

/// Information about the sticker, which was used to create the chat photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotoStickerTypeRegularOrMask {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker set identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  sticker_set_id: isize,
  /// Identifier of the sticker in the set
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  sticker_id: isize,
}

impl RObject for ChatPhotoStickerTypeRegularOrMask {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatPhotoStickerTypeRegularOrMask"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatPhotoStickerType for ChatPhotoStickerTypeRegularOrMask {}

impl ChatPhotoStickerTypeRegularOrMask {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatPhotoStickerTypeRegularOrMaskBuilder {
    let inner = ChatPhotoStickerTypeRegularOrMask {
      td_name: "chatPhotoStickerTypeRegularOrMask".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatPhotoStickerTypeRegularOrMaskBuilder { inner }
  }

  pub fn sticker_set_id(&self) -> isize {
    self.sticker_set_id
  }

  pub fn sticker_id(&self) -> isize {
    self.sticker_id
  }
}

#[doc(hidden)]
pub struct RTDChatPhotoStickerTypeRegularOrMaskBuilder {
  inner: ChatPhotoStickerTypeRegularOrMask,
}

impl RTDChatPhotoStickerTypeRegularOrMaskBuilder {
  pub fn build(&self) -> ChatPhotoStickerTypeRegularOrMask {
    self.inner.clone()
  }

  pub fn sticker_set_id(&mut self, sticker_set_id: isize) -> &mut Self {
    self.inner.sticker_set_id = sticker_set_id;
    self
  }

  pub fn sticker_id(&mut self, sticker_id: isize) -> &mut Self {
    self.inner.sticker_id = sticker_id;
    self
  }
}

impl AsRef<ChatPhotoStickerTypeRegularOrMask> for ChatPhotoStickerTypeRegularOrMask {
  fn as_ref(&self) -> &ChatPhotoStickerTypeRegularOrMask {
    self
  }
}

impl AsRef<ChatPhotoStickerTypeRegularOrMask> for RTDChatPhotoStickerTypeRegularOrMaskBuilder {
  fn as_ref(&self) -> &ChatPhotoStickerTypeRegularOrMask {
    &self.inner
  }
}
