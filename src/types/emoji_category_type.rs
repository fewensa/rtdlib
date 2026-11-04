use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of emoji category
pub trait TDEmojiCategoryType: Debug + RObject {}

/// Describes type of emoji category
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EmojiCategoryType {
  #[doc(hidden)]
  _Default(()),
  /// The category must be used for chat photo emoji selection
  ChatPhoto(EmojiCategoryTypeChatPhoto),
  /// The category must be used by default (e.g., for custom emoji or animation search)
  Default(EmojiCategoryTypeDefault),
  /// The category must be used for emoji status selection
  EmojiStatus(EmojiCategoryTypeEmojiStatus),
  /// The category must be used by default for regular sticker selection. It may contain greeting emoji category and premium stickers
  RegularStickers(EmojiCategoryTypeRegularStickers),
}

impl Default for EmojiCategoryType {
  fn default() -> Self {
    EmojiCategoryType::_Default(())
  }
}

impl<'de> Deserialize<'de> for EmojiCategoryType {
  fn deserialize<D>(deserializer: D) -> Result<EmojiCategoryType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      EmojiCategoryType,
      (emojiCategoryTypeChatPhoto, ChatPhoto);
      (emojiCategoryTypeDefault, Default);
      (emojiCategoryTypeEmojiStatus, EmojiStatus);
      (emojiCategoryTypeRegularStickers, RegularStickers);

    )(deserializer)
  }
}

impl RObject for EmojiCategoryType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      EmojiCategoryType::ChatPhoto(t) => t.td_name(),
      EmojiCategoryType::Default(t) => t.td_name(),
      EmojiCategoryType::EmojiStatus(t) => t.td_name(),
      EmojiCategoryType::RegularStickers(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      EmojiCategoryType::ChatPhoto(t) => t.extra(),
      EmojiCategoryType::Default(t) => t.extra(),
      EmojiCategoryType::EmojiStatus(t) => t.extra(),
      EmojiCategoryType::RegularStickers(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiCategoryType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, EmojiCategoryType::_Default(_))
  }

  pub fn is_chat_photo(&self) -> bool {
    matches!(self, EmojiCategoryType::ChatPhoto(_))
  }
  pub fn is_default(&self) -> bool {
    matches!(self, EmojiCategoryType::Default(_))
  }
  pub fn is_emoji_status(&self) -> bool {
    matches!(self, EmojiCategoryType::EmojiStatus(_))
  }
  pub fn is_regular_stickers(&self) -> bool {
    matches!(self, EmojiCategoryType::RegularStickers(_))
  }

  pub fn on_chat_photo<F: FnOnce(&EmojiCategoryTypeChatPhoto)>(&self, fnc: F) -> &Self {
    if let EmojiCategoryType::ChatPhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_default<F: FnOnce(&EmojiCategoryTypeDefault)>(&self, fnc: F) -> &Self {
    if let EmojiCategoryType::Default(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_emoji_status<F: FnOnce(&EmojiCategoryTypeEmojiStatus)>(&self, fnc: F) -> &Self {
    if let EmojiCategoryType::EmojiStatus(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_regular_stickers<F: FnOnce(&EmojiCategoryTypeRegularStickers)>(&self, fnc: F) -> &Self {
    if let EmojiCategoryType::RegularStickers(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_chat_photo(&self) -> Option<&EmojiCategoryTypeChatPhoto> {
    if let EmojiCategoryType::ChatPhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_default(&self) -> Option<&EmojiCategoryTypeDefault> {
    if let EmojiCategoryType::Default(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_emoji_status(&self) -> Option<&EmojiCategoryTypeEmojiStatus> {
    if let EmojiCategoryType::EmojiStatus(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_regular_stickers(&self) -> Option<&EmojiCategoryTypeRegularStickers> {
    if let EmojiCategoryType::RegularStickers(t) = self {
      return Some(t);
    }
    None
  }

  pub fn chat_photo<T: AsRef<EmojiCategoryTypeChatPhoto>>(t: T) -> Self {
    EmojiCategoryType::ChatPhoto(t.as_ref().clone())
  }

  pub fn default<T: AsRef<EmojiCategoryTypeDefault>>(t: T) -> Self {
    EmojiCategoryType::Default(t.as_ref().clone())
  }

  pub fn emoji_status<T: AsRef<EmojiCategoryTypeEmojiStatus>>(t: T) -> Self {
    EmojiCategoryType::EmojiStatus(t.as_ref().clone())
  }

  pub fn regular_stickers<T: AsRef<EmojiCategoryTypeRegularStickers>>(t: T) -> Self {
    EmojiCategoryType::RegularStickers(t.as_ref().clone())
  }
}

impl AsRef<EmojiCategoryType> for EmojiCategoryType {
  fn as_ref(&self) -> &EmojiCategoryType {
    self
  }
}

/// The category must be used for chat photo emoji selection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategoryTypeChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for EmojiCategoryTypeChatPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiCategoryTypeChatPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiCategoryType for EmojiCategoryTypeChatPhoto {}

impl EmojiCategoryTypeChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiCategoryTypeChatPhotoBuilder {
    let inner = EmojiCategoryTypeChatPhoto {
      td_name: "emojiCategoryTypeChatPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDEmojiCategoryTypeChatPhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDEmojiCategoryTypeChatPhotoBuilder {
  inner: EmojiCategoryTypeChatPhoto,
}

impl RTDEmojiCategoryTypeChatPhotoBuilder {
  pub fn build(&self) -> EmojiCategoryTypeChatPhoto {
    self.inner.clone()
  }
}

impl AsRef<EmojiCategoryTypeChatPhoto> for EmojiCategoryTypeChatPhoto {
  fn as_ref(&self) -> &EmojiCategoryTypeChatPhoto {
    self
  }
}

impl AsRef<EmojiCategoryTypeChatPhoto> for RTDEmojiCategoryTypeChatPhotoBuilder {
  fn as_ref(&self) -> &EmojiCategoryTypeChatPhoto {
    &self.inner
  }
}

/// The category must be used by default (e.g., for custom emoji or animation search)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategoryTypeDefault {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for EmojiCategoryTypeDefault {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiCategoryTypeDefault"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiCategoryType for EmojiCategoryTypeDefault {}

impl EmojiCategoryTypeDefault {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiCategoryTypeDefaultBuilder {
    let inner = EmojiCategoryTypeDefault {
      td_name: "emojiCategoryTypeDefault".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDEmojiCategoryTypeDefaultBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDEmojiCategoryTypeDefaultBuilder {
  inner: EmojiCategoryTypeDefault,
}

impl RTDEmojiCategoryTypeDefaultBuilder {
  pub fn build(&self) -> EmojiCategoryTypeDefault {
    self.inner.clone()
  }
}

impl AsRef<EmojiCategoryTypeDefault> for EmojiCategoryTypeDefault {
  fn as_ref(&self) -> &EmojiCategoryTypeDefault {
    self
  }
}

impl AsRef<EmojiCategoryTypeDefault> for RTDEmojiCategoryTypeDefaultBuilder {
  fn as_ref(&self) -> &EmojiCategoryTypeDefault {
    &self.inner
  }
}

/// The category must be used for emoji status selection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategoryTypeEmojiStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for EmojiCategoryTypeEmojiStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiCategoryTypeEmojiStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiCategoryType for EmojiCategoryTypeEmojiStatus {}

impl EmojiCategoryTypeEmojiStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiCategoryTypeEmojiStatusBuilder {
    let inner = EmojiCategoryTypeEmojiStatus {
      td_name: "emojiCategoryTypeEmojiStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDEmojiCategoryTypeEmojiStatusBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDEmojiCategoryTypeEmojiStatusBuilder {
  inner: EmojiCategoryTypeEmojiStatus,
}

impl RTDEmojiCategoryTypeEmojiStatusBuilder {
  pub fn build(&self) -> EmojiCategoryTypeEmojiStatus {
    self.inner.clone()
  }
}

impl AsRef<EmojiCategoryTypeEmojiStatus> for EmojiCategoryTypeEmojiStatus {
  fn as_ref(&self) -> &EmojiCategoryTypeEmojiStatus {
    self
  }
}

impl AsRef<EmojiCategoryTypeEmojiStatus> for RTDEmojiCategoryTypeEmojiStatusBuilder {
  fn as_ref(&self) -> &EmojiCategoryTypeEmojiStatus {
    &self.inner
  }
}

/// The category must be used by default for regular sticker selection. It may contain greeting emoji category and premium stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategoryTypeRegularStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for EmojiCategoryTypeRegularStickers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiCategoryTypeRegularStickers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiCategoryType for EmojiCategoryTypeRegularStickers {}

impl EmojiCategoryTypeRegularStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiCategoryTypeRegularStickersBuilder {
    let inner = EmojiCategoryTypeRegularStickers {
      td_name: "emojiCategoryTypeRegularStickers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDEmojiCategoryTypeRegularStickersBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDEmojiCategoryTypeRegularStickersBuilder {
  inner: EmojiCategoryTypeRegularStickers,
}

impl RTDEmojiCategoryTypeRegularStickersBuilder {
  pub fn build(&self) -> EmojiCategoryTypeRegularStickers {
    self.inner.clone()
  }
}

impl AsRef<EmojiCategoryTypeRegularStickers> for EmojiCategoryTypeRegularStickers {
  fn as_ref(&self) -> &EmojiCategoryTypeRegularStickers {
    self
  }
}

impl AsRef<EmojiCategoryTypeRegularStickers> for RTDEmojiCategoryTypeRegularStickersBuilder {
  fn as_ref(&self) -> &EmojiCategoryTypeRegularStickers {
    &self.inner
  }
}
