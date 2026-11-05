use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains full information about sticker type
pub trait TDStickerFullType: Debug + RObject {}

/// Contains full information about sticker type
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StickerFullType {
  #[doc(hidden)]
  _Default(()),
  /// The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
  CustomEmoji(StickerFullTypeCustomEmoji),
  /// The sticker is a mask in WEBP format to be placed on photos or videos
  Mask(StickerFullTypeMask),
  /// The sticker is a regular sticker
  Regular(StickerFullTypeRegular),
}

impl Default for StickerFullType {
  fn default() -> Self {
    StickerFullType::_Default(())
  }
}

impl<'de> Deserialize<'de> for StickerFullType {
  fn deserialize<D>(deserializer: D) -> Result<StickerFullType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StickerFullType,
      (stickerFullTypeCustomEmoji, CustomEmoji);
      (stickerFullTypeMask, Mask);
      (stickerFullTypeRegular, Regular);

    )(deserializer)
  }
}

impl RObject for StickerFullType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StickerFullType::CustomEmoji(t) => t.td_name(),
      StickerFullType::Mask(t) => t.td_name(),
      StickerFullType::Regular(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StickerFullType::CustomEmoji(t) => t.extra(),
      StickerFullType::Mask(t) => t.extra(),
      StickerFullType::Regular(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StickerFullType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StickerFullType::_Default(_))
  }

  pub fn is_custom_emoji(&self) -> bool {
    matches!(self, StickerFullType::CustomEmoji(_))
  }
  pub fn is_mask(&self) -> bool {
    matches!(self, StickerFullType::Mask(_))
  }
  pub fn is_regular(&self) -> bool {
    matches!(self, StickerFullType::Regular(_))
  }

  pub fn on_custom_emoji<F: FnOnce(&StickerFullTypeCustomEmoji)>(&self, fnc: F) -> &Self {
    if let StickerFullType::CustomEmoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_mask<F: FnOnce(&StickerFullTypeMask)>(&self, fnc: F) -> &Self {
    if let StickerFullType::Mask(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_regular<F: FnOnce(&StickerFullTypeRegular)>(&self, fnc: F) -> &Self {
    if let StickerFullType::Regular(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_custom_emoji(&self) -> Option<&StickerFullTypeCustomEmoji> {
    if let StickerFullType::CustomEmoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_mask(&self) -> Option<&StickerFullTypeMask> {
    if let StickerFullType::Mask(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_regular(&self) -> Option<&StickerFullTypeRegular> {
    if let StickerFullType::Regular(t) = self {
      return Some(t);
    }
    None
  }

  pub fn custom_emoji<T: AsRef<StickerFullTypeCustomEmoji>>(t: T) -> Self {
    StickerFullType::CustomEmoji(t.as_ref().clone())
  }

  pub fn mask<T: AsRef<StickerFullTypeMask>>(t: T) -> Self {
    StickerFullType::Mask(t.as_ref().clone())
  }

  pub fn regular<T: AsRef<StickerFullTypeRegular>>(t: T) -> Self {
    StickerFullType::Regular(t.as_ref().clone())
  }
}

impl AsRef<StickerFullType> for StickerFullType {
  fn as_ref(&self) -> &StickerFullType {
    self
  }
}

/// The sticker is a custom emoji to be used inside message text and caption. Currently, only Telegram Premium users can use custom emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFullTypeCustomEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the custom emoji
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_id: isize,
  /// True, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
  needs_repainting: bool,
}

impl RObject for StickerFullTypeCustomEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerFullTypeCustomEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerFullType for StickerFullTypeCustomEmoji {}

impl StickerFullTypeCustomEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerFullTypeCustomEmojiBuilder {
    let inner = StickerFullTypeCustomEmoji {
      td_name: "stickerFullTypeCustomEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStickerFullTypeCustomEmojiBuilder { inner }
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }

  pub fn needs_repainting(&self) -> bool {
    self.needs_repainting
  }
}

#[doc(hidden)]
pub struct RTDStickerFullTypeCustomEmojiBuilder {
  inner: StickerFullTypeCustomEmoji,
}

impl RTDStickerFullTypeCustomEmojiBuilder {
  pub fn build(&self) -> StickerFullTypeCustomEmoji {
    self.inner.clone()
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }

  pub fn needs_repainting(&mut self, needs_repainting: bool) -> &mut Self {
    self.inner.needs_repainting = needs_repainting;
    self
  }
}

impl AsRef<StickerFullTypeCustomEmoji> for StickerFullTypeCustomEmoji {
  fn as_ref(&self) -> &StickerFullTypeCustomEmoji {
    self
  }
}

impl AsRef<StickerFullTypeCustomEmoji> for RTDStickerFullTypeCustomEmojiBuilder {
  fn as_ref(&self) -> &StickerFullTypeCustomEmoji {
    &self.inner
  }
}

/// The sticker is a mask in WEBP format to be placed on photos or videos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFullTypeMask {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Position where the mask is placed; may be null
  mask_position: Option<MaskPosition>,
}

impl RObject for StickerFullTypeMask {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerFullTypeMask"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerFullType for StickerFullTypeMask {}

impl StickerFullTypeMask {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerFullTypeMaskBuilder {
    let inner = StickerFullTypeMask {
      td_name: "stickerFullTypeMask".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStickerFullTypeMaskBuilder { inner }
  }

  pub fn mask_position(&self) -> &Option<MaskPosition> {
    &self.mask_position
  }
}

#[doc(hidden)]
pub struct RTDStickerFullTypeMaskBuilder {
  inner: StickerFullTypeMask,
}

impl RTDStickerFullTypeMaskBuilder {
  pub fn build(&self) -> StickerFullTypeMask {
    self.inner.clone()
  }

  pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
    self.inner.mask_position = Some(mask_position.as_ref().clone());
    self
  }
}

impl AsRef<StickerFullTypeMask> for StickerFullTypeMask {
  fn as_ref(&self) -> &StickerFullTypeMask {
    self
  }
}

impl AsRef<StickerFullTypeMask> for RTDStickerFullTypeMaskBuilder {
  fn as_ref(&self) -> &StickerFullTypeMask {
    &self.inner
  }
}

/// The sticker is a regular sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerFullTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Premium animation of the sticker; may be null. If present, only Telegram Premium users can use the sticker
  premium_animation: Option<File>,
}

impl RObject for StickerFullTypeRegular {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stickerFullTypeRegular"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStickerFullType for StickerFullTypeRegular {}

impl StickerFullTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerFullTypeRegularBuilder {
    let inner = StickerFullTypeRegular {
      td_name: "stickerFullTypeRegular".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStickerFullTypeRegularBuilder { inner }
  }

  pub fn premium_animation(&self) -> &Option<File> {
    &self.premium_animation
  }
}

#[doc(hidden)]
pub struct RTDStickerFullTypeRegularBuilder {
  inner: StickerFullTypeRegular,
}

impl RTDStickerFullTypeRegularBuilder {
  pub fn build(&self) -> StickerFullTypeRegular {
    self.inner.clone()
  }

  pub fn premium_animation<T: AsRef<File>>(&mut self, premium_animation: T) -> &mut Self {
    self.inner.premium_animation = Some(premium_animation.as_ref().clone());
    self
  }
}

impl AsRef<StickerFullTypeRegular> for StickerFullTypeRegular {
  fn as_ref(&self) -> &StickerFullTypeRegular {
    self
  }
}

impl AsRef<StickerFullTypeRegular> for RTDStickerFullTypeRegularBuilder {
  fn as_ref(&self) -> &StickerFullTypeRegular {
    &self.inner
  }
}
