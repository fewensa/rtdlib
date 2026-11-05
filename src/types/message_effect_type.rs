use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of emoji effect
pub trait TDMessageEffectType: Debug + RObject {}

/// Describes type of emoji effect
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageEffectType {
  #[doc(hidden)]
  _Default(()),
  /// An effect from an emoji reaction
  EmojiReaction(MessageEffectTypeEmojiReaction),
  /// An effect from a premium sticker
  PremiumSticker(MessageEffectTypePremiumSticker),
}

impl Default for MessageEffectType {
  fn default() -> Self {
    MessageEffectType::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageEffectType {
  fn deserialize<D>(deserializer: D) -> Result<MessageEffectType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageEffectType,
      (messageEffectTypeEmojiReaction, EmojiReaction);
      (messageEffectTypePremiumSticker, PremiumSticker);

    )(deserializer)
  }
}

impl RObject for MessageEffectType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageEffectType::EmojiReaction(t) => t.td_name(),
      MessageEffectType::PremiumSticker(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageEffectType::EmojiReaction(t) => t.extra(),
      MessageEffectType::PremiumSticker(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageEffectType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageEffectType::_Default(_))
  }

  pub fn is_emoji_reaction(&self) -> bool {
    matches!(self, MessageEffectType::EmojiReaction(_))
  }
  pub fn is_premium_sticker(&self) -> bool {
    matches!(self, MessageEffectType::PremiumSticker(_))
  }

  pub fn on_emoji_reaction<F: FnOnce(&MessageEffectTypeEmojiReaction)>(&self, fnc: F) -> &Self {
    if let MessageEffectType::EmojiReaction(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_sticker<F: FnOnce(&MessageEffectTypePremiumSticker)>(&self, fnc: F) -> &Self {
    if let MessageEffectType::PremiumSticker(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_emoji_reaction(&self) -> Option<&MessageEffectTypeEmojiReaction> {
    if let MessageEffectType::EmojiReaction(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_sticker(&self) -> Option<&MessageEffectTypePremiumSticker> {
    if let MessageEffectType::PremiumSticker(t) = self {
      return Some(t);
    }
    None
  }

  pub fn emoji_reaction<T: AsRef<MessageEffectTypeEmojiReaction>>(t: T) -> Self {
    MessageEffectType::EmojiReaction(t.as_ref().clone())
  }

  pub fn premium_sticker<T: AsRef<MessageEffectTypePremiumSticker>>(t: T) -> Self {
    MessageEffectType::PremiumSticker(t.as_ref().clone())
  }
}

impl AsRef<MessageEffectType> for MessageEffectType {
  fn as_ref(&self) -> &MessageEffectType {
    self
  }
}

/// An effect from an emoji reaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageEffectTypeEmojiReaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Select animation for the effect in TGS format
  select_animation: Sticker,
  /// Effect animation for the effect in TGS format
  effect_animation: Sticker,
}

impl RObject for MessageEffectTypeEmojiReaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageEffectTypeEmojiReaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageEffectType for MessageEffectTypeEmojiReaction {}

impl MessageEffectTypeEmojiReaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageEffectTypeEmojiReactionBuilder {
    let inner = MessageEffectTypeEmojiReaction {
      td_name: "messageEffectTypeEmojiReaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageEffectTypeEmojiReactionBuilder { inner }
  }

  pub fn select_animation(&self) -> &Sticker {
    &self.select_animation
  }

  pub fn effect_animation(&self) -> &Sticker {
    &self.effect_animation
  }
}

#[doc(hidden)]
pub struct RTDMessageEffectTypeEmojiReactionBuilder {
  inner: MessageEffectTypeEmojiReaction,
}

impl RTDMessageEffectTypeEmojiReactionBuilder {
  pub fn build(&self) -> MessageEffectTypeEmojiReaction {
    self.inner.clone()
  }

  pub fn select_animation<T: AsRef<Sticker>>(&mut self, select_animation: T) -> &mut Self {
    self.inner.select_animation = select_animation.as_ref().clone();
    self
  }

  pub fn effect_animation<T: AsRef<Sticker>>(&mut self, effect_animation: T) -> &mut Self {
    self.inner.effect_animation = effect_animation.as_ref().clone();
    self
  }
}

impl AsRef<MessageEffectTypeEmojiReaction> for MessageEffectTypeEmojiReaction {
  fn as_ref(&self) -> &MessageEffectTypeEmojiReaction {
    self
  }
}

impl AsRef<MessageEffectTypeEmojiReaction> for RTDMessageEffectTypeEmojiReactionBuilder {
  fn as_ref(&self) -> &MessageEffectTypeEmojiReaction {
    &self.inner
  }
}

/// An effect from a premium sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageEffectTypePremiumSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The premium sticker. The effect can be found at sticker.full_type.premium_animation
  sticker: Sticker,
}

impl RObject for MessageEffectTypePremiumSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageEffectTypePremiumSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageEffectType for MessageEffectTypePremiumSticker {}

impl MessageEffectTypePremiumSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageEffectTypePremiumStickerBuilder {
    let inner = MessageEffectTypePremiumSticker {
      td_name: "messageEffectTypePremiumSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageEffectTypePremiumStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &Sticker {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDMessageEffectTypePremiumStickerBuilder {
  inner: MessageEffectTypePremiumSticker,
}

impl RTDMessageEffectTypePremiumStickerBuilder {
  pub fn build(&self) -> MessageEffectTypePremiumSticker {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }
}

impl AsRef<MessageEffectTypePremiumSticker> for MessageEffectTypePremiumSticker {
  fn as_ref(&self) -> &MessageEffectTypePremiumSticker {
    self
  }
}

impl AsRef<MessageEffectTypePremiumSticker> for RTDMessageEffectTypePremiumStickerBuilder {
  fn as_ref(&self) -> &MessageEffectTypePremiumSticker {
    &self.inner
  }
}
