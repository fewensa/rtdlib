use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of sticker
pub trait TDStickerType: Debug + RObject {}

/// Describes type of sticker
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StickerType {
  #[doc(hidden)]
  _Default(()),
  /// The sticker is a custom emoji to be used inside message text and caption
  CustomEmoji(StickerTypeCustomEmoji),
  /// The sticker is a mask in WEBP format to be placed on photos or videos
  Mask(StickerTypeMask),
  /// The sticker is a regular sticker
  Regular(StickerTypeRegular),
}

impl Default for StickerType {
  fn default() -> Self {
    StickerType::_Default(())
  }
}

impl<'de> Deserialize<'de> for StickerType {
  fn deserialize<D>(deserializer: D) -> Result<StickerType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StickerType,
      (stickerTypeCustomEmoji, CustomEmoji);
      (stickerTypeMask, Mask);
      (stickerTypeRegular, Regular);

    )(deserializer)
  }
}

impl RObject for StickerType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StickerType::CustomEmoji(t) => t.td_name(),
      StickerType::Mask(t) => t.td_name(),
      StickerType::Regular(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StickerType::CustomEmoji(t) => t.extra(),
      StickerType::Mask(t) => t.extra(),
      StickerType::Regular(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StickerType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StickerType::_Default(_))
  }

  pub fn is_custom_emoji(&self) -> bool {
    matches!(self, StickerType::CustomEmoji(_))
  }
  pub fn is_mask(&self) -> bool {
    matches!(self, StickerType::Mask(_))
  }
  pub fn is_regular(&self) -> bool {
    matches!(self, StickerType::Regular(_))
  }

  pub fn on_custom_emoji<F: FnOnce(&StickerTypeCustomEmoji)>(&self, fnc: F) -> &Self {
    if let StickerType::CustomEmoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_mask<F: FnOnce(&StickerTypeMask)>(&self, fnc: F) -> &Self {
    if let StickerType::Mask(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_regular<F: FnOnce(&StickerTypeRegular)>(&self, fnc: F) -> &Self {
    if let StickerType::Regular(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_custom_emoji(&self) -> Option<&StickerTypeCustomEmoji> {
    if let StickerType::CustomEmoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_mask(&self) -> Option<&StickerTypeMask> {
    if let StickerType::Mask(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_regular(&self) -> Option<&StickerTypeRegular> {
    if let StickerType::Regular(t) = self {
      return Some(t);
    }
    None
  }

  pub fn custom_emoji<T: AsRef<StickerTypeCustomEmoji>>(t: T) -> Self {
    StickerType::CustomEmoji(t.as_ref().clone())
  }

  pub fn mask<T: AsRef<StickerTypeMask>>(t: T) -> Self {
    StickerType::Mask(t.as_ref().clone())
  }

  pub fn regular<T: AsRef<StickerTypeRegular>>(t: T) -> Self {
    StickerType::Regular(t.as_ref().clone())
  }
}

impl AsRef<StickerType> for StickerType {
  fn as_ref(&self) -> &StickerType {
    self
  }
}

/// The sticker is a custom emoji to be used inside message text and caption
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeCustomEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StickerTypeCustomEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerTypeCustomEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerType for StickerTypeCustomEmoji {}

impl StickerTypeCustomEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerTypeCustomEmojiBuilder {
    let inner = StickerTypeCustomEmoji {
      td_name: "stickerTypeCustomEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStickerTypeCustomEmojiBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStickerTypeCustomEmojiBuilder {
  inner: StickerTypeCustomEmoji,
}

impl RTDStickerTypeCustomEmojiBuilder {
  pub fn build(&self) -> StickerTypeCustomEmoji {
    self.inner.clone()
  }
}

impl AsRef<StickerTypeCustomEmoji> for StickerTypeCustomEmoji {
  fn as_ref(&self) -> &StickerTypeCustomEmoji {
    self
  }
}

impl AsRef<StickerTypeCustomEmoji> for RTDStickerTypeCustomEmojiBuilder {
  fn as_ref(&self) -> &StickerTypeCustomEmoji {
    &self.inner
  }
}

/// The sticker is a mask in WEBP format to be placed on photos or videos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeMask {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StickerTypeMask {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerTypeMask"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerType for StickerTypeMask {}

impl StickerTypeMask {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerTypeMaskBuilder {
    let inner = StickerTypeMask {
      td_name: "stickerTypeMask".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStickerTypeMaskBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStickerTypeMaskBuilder {
  inner: StickerTypeMask,
}

impl RTDStickerTypeMaskBuilder {
  pub fn build(&self) -> StickerTypeMask {
    self.inner.clone()
  }
}

impl AsRef<StickerTypeMask> for StickerTypeMask {
  fn as_ref(&self) -> &StickerTypeMask {
    self
  }
}

impl AsRef<StickerTypeMask> for RTDStickerTypeMaskBuilder {
  fn as_ref(&self) -> &StickerTypeMask {
    &self.inner
  }
}

/// The sticker is a regular sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StickerTypeRegular {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerTypeRegular"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerType for StickerTypeRegular {}

impl StickerTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerTypeRegularBuilder {
    let inner = StickerTypeRegular {
      td_name: "stickerTypeRegular".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStickerTypeRegularBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStickerTypeRegularBuilder {
  inner: StickerTypeRegular,
}

impl RTDStickerTypeRegularBuilder {
  pub fn build(&self) -> StickerTypeRegular {
    self.inner.clone()
  }
}

impl AsRef<StickerTypeRegular> for StickerTypeRegular {
  fn as_ref(&self) -> &StickerTypeRegular {
    self
  }
}

impl AsRef<StickerTypeRegular> for RTDStickerTypeRegularBuilder {
  fn as_ref(&self) -> &StickerTypeRegular {
    &self.inner
  }
}
