use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of emoji status
pub trait TDEmojiStatusType: Debug + RObject {}

/// Describes type of emoji status
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EmojiStatusType {
  #[doc(hidden)]
  _Default(()),
  /// A custom emoji set as emoji status
  CustomEmoji(EmojiStatusTypeCustomEmoji),
  /// An upgraded gift set as emoji status
  UpgradedGift(EmojiStatusTypeUpgradedGift),
}

impl Default for EmojiStatusType {
  fn default() -> Self {
    EmojiStatusType::_Default(())
  }
}

impl<'de> Deserialize<'de> for EmojiStatusType {
  fn deserialize<D>(deserializer: D) -> Result<EmojiStatusType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      EmojiStatusType,
      (emojiStatusTypeCustomEmoji, CustomEmoji);
      (emojiStatusTypeUpgradedGift, UpgradedGift);

    )(deserializer)
  }
}

impl RObject for EmojiStatusType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      EmojiStatusType::CustomEmoji(t) => t.td_name(),
      EmojiStatusType::UpgradedGift(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      EmojiStatusType::CustomEmoji(t) => t.extra(),
      EmojiStatusType::UpgradedGift(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiStatusType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, EmojiStatusType::_Default(_))
  }

  pub fn is_custom_emoji(&self) -> bool {
    matches!(self, EmojiStatusType::CustomEmoji(_))
  }
  pub fn is_upgraded_gift(&self) -> bool {
    matches!(self, EmojiStatusType::UpgradedGift(_))
  }

  pub fn on_custom_emoji<F: FnOnce(&EmojiStatusTypeCustomEmoji)>(&self, fnc: F) -> &Self {
    if let EmojiStatusType::CustomEmoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift<F: FnOnce(&EmojiStatusTypeUpgradedGift)>(&self, fnc: F) -> &Self {
    if let EmojiStatusType::UpgradedGift(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_custom_emoji(&self) -> Option<&EmojiStatusTypeCustomEmoji> {
    if let EmojiStatusType::CustomEmoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift(&self) -> Option<&EmojiStatusTypeUpgradedGift> {
    if let EmojiStatusType::UpgradedGift(t) = self {
      return Some(t);
    }
    None
  }

  pub fn custom_emoji<T: AsRef<EmojiStatusTypeCustomEmoji>>(t: T) -> Self {
    EmojiStatusType::CustomEmoji(t.as_ref().clone())
  }

  pub fn upgraded_gift<T: AsRef<EmojiStatusTypeUpgradedGift>>(t: T) -> Self {
    EmojiStatusType::UpgradedGift(t.as_ref().clone())
  }
}

impl AsRef<EmojiStatusType> for EmojiStatusType {
  fn as_ref(&self) -> &EmojiStatusType {
    self
  }
}

/// A custom emoji set as emoji status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiStatusTypeCustomEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the custom emoji in stickerFormatTgs format
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_id: isize,
}

impl RObject for EmojiStatusTypeCustomEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiStatusTypeCustomEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiStatusType for EmojiStatusTypeCustomEmoji {}

impl EmojiStatusTypeCustomEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiStatusTypeCustomEmojiBuilder {
    let inner = EmojiStatusTypeCustomEmoji {
      td_name: "emojiStatusTypeCustomEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiStatusTypeCustomEmojiBuilder { inner }
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDEmojiStatusTypeCustomEmojiBuilder {
  inner: EmojiStatusTypeCustomEmoji,
}

impl RTDEmojiStatusTypeCustomEmojiBuilder {
  pub fn build(&self) -> EmojiStatusTypeCustomEmoji {
    self.inner.clone()
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }
}

impl AsRef<EmojiStatusTypeCustomEmoji> for EmojiStatusTypeCustomEmoji {
  fn as_ref(&self) -> &EmojiStatusTypeCustomEmoji {
    self
  }
}

impl AsRef<EmojiStatusTypeCustomEmoji> for RTDEmojiStatusTypeCustomEmojiBuilder {
  fn as_ref(&self) -> &EmojiStatusTypeCustomEmoji {
    &self.inner
  }
}

/// An upgraded gift set as emoji status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiStatusTypeUpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the upgraded gift
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  upgraded_gift_id: isize,
  /// The title of the upgraded gift
  gift_title: String,
  /// Unique name of the upgraded gift that can be used with internalLinkTypeUpgradedGift
  gift_name: String,
  /// Custom emoji identifier of the model of the upgraded gift
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  model_custom_emoji_id: isize,
  /// Custom emoji identifier of the symbol of the upgraded gift
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  symbol_custom_emoji_id: isize,
  /// Colors of the backdrop of the upgraded gift
  backdrop_colors: UpgradedGiftBackdropColors,
}

impl RObject for EmojiStatusTypeUpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiStatusTypeUpgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiStatusType for EmojiStatusTypeUpgradedGift {}

impl EmojiStatusTypeUpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiStatusTypeUpgradedGiftBuilder {
    let inner = EmojiStatusTypeUpgradedGift {
      td_name: "emojiStatusTypeUpgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiStatusTypeUpgradedGiftBuilder { inner }
  }

  pub fn upgraded_gift_id(&self) -> isize {
    self.upgraded_gift_id
  }

  pub fn gift_title(&self) -> &String {
    &self.gift_title
  }

  pub fn gift_name(&self) -> &String {
    &self.gift_name
  }

  pub fn model_custom_emoji_id(&self) -> isize {
    self.model_custom_emoji_id
  }

  pub fn symbol_custom_emoji_id(&self) -> isize {
    self.symbol_custom_emoji_id
  }

  pub fn backdrop_colors(&self) -> &UpgradedGiftBackdropColors {
    &self.backdrop_colors
  }
}

#[doc(hidden)]
pub struct RTDEmojiStatusTypeUpgradedGiftBuilder {
  inner: EmojiStatusTypeUpgradedGift,
}

impl RTDEmojiStatusTypeUpgradedGiftBuilder {
  pub fn build(&self) -> EmojiStatusTypeUpgradedGift {
    self.inner.clone()
  }

  pub fn upgraded_gift_id(&mut self, upgraded_gift_id: isize) -> &mut Self {
    self.inner.upgraded_gift_id = upgraded_gift_id;
    self
  }

  pub fn gift_title<T: AsRef<str>>(&mut self, gift_title: T) -> &mut Self {
    self.inner.gift_title = gift_title.as_ref().to_string();
    self
  }

  pub fn gift_name<T: AsRef<str>>(&mut self, gift_name: T) -> &mut Self {
    self.inner.gift_name = gift_name.as_ref().to_string();
    self
  }

  pub fn model_custom_emoji_id(&mut self, model_custom_emoji_id: isize) -> &mut Self {
    self.inner.model_custom_emoji_id = model_custom_emoji_id;
    self
  }

  pub fn symbol_custom_emoji_id(&mut self, symbol_custom_emoji_id: isize) -> &mut Self {
    self.inner.symbol_custom_emoji_id = symbol_custom_emoji_id;
    self
  }

  pub fn backdrop_colors<T: AsRef<UpgradedGiftBackdropColors>>(
    &mut self,
    backdrop_colors: T,
  ) -> &mut Self {
    self.inner.backdrop_colors = backdrop_colors.as_ref().clone();
    self
  }
}

impl AsRef<EmojiStatusTypeUpgradedGift> for EmojiStatusTypeUpgradedGift {
  fn as_ref(&self) -> &EmojiStatusTypeUpgradedGift {
    self
  }
}

impl AsRef<EmojiStatusTypeUpgradedGift> for RTDEmojiStatusTypeUpgradedGiftBuilder {
  fn as_ref(&self) -> &EmojiStatusTypeUpgradedGift {
    &self.inner
  }
}
