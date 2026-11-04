use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of message reaction
pub trait TDReactionType: Debug + RObject {}

/// Describes type of message reaction
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReactionType {
  #[doc(hidden)]
  _Default(()),
  /// A reaction with a custom emoji
  CustomEmoji(ReactionTypeCustomEmoji),
  /// A reaction with an emoji
  Emoji(ReactionTypeEmoji),
  /// The paid reaction in a channel chat
  Paid(ReactionTypePaid),
}

impl Default for ReactionType {
  fn default() -> Self {
    ReactionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for ReactionType {
  fn deserialize<D>(deserializer: D) -> Result<ReactionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ReactionType,
      (reactionTypeCustomEmoji, CustomEmoji);
      (reactionTypeEmoji, Emoji);
      (reactionTypePaid, Paid);

    )(deserializer)
  }
}

impl RObject for ReactionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ReactionType::CustomEmoji(t) => t.td_name(),
      ReactionType::Emoji(t) => t.td_name(),
      ReactionType::Paid(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ReactionType::CustomEmoji(t) => t.extra(),
      ReactionType::Emoji(t) => t.extra(),
      ReactionType::Paid(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReactionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ReactionType::_Default(_))
  }

  pub fn is_custom_emoji(&self) -> bool {
    matches!(self, ReactionType::CustomEmoji(_))
  }
  pub fn is_emoji(&self) -> bool {
    matches!(self, ReactionType::Emoji(_))
  }
  pub fn is_paid(&self) -> bool {
    matches!(self, ReactionType::Paid(_))
  }

  pub fn on_custom_emoji<F: FnOnce(&ReactionTypeCustomEmoji)>(&self, fnc: F) -> &Self {
    if let ReactionType::CustomEmoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_emoji<F: FnOnce(&ReactionTypeEmoji)>(&self, fnc: F) -> &Self {
    if let ReactionType::Emoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid<F: FnOnce(&ReactionTypePaid)>(&self, fnc: F) -> &Self {
    if let ReactionType::Paid(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_custom_emoji(&self) -> Option<&ReactionTypeCustomEmoji> {
    if let ReactionType::CustomEmoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_emoji(&self) -> Option<&ReactionTypeEmoji> {
    if let ReactionType::Emoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid(&self) -> Option<&ReactionTypePaid> {
    if let ReactionType::Paid(t) = self {
      return Some(t);
    }
    None
  }

  pub fn custom_emoji<T: AsRef<ReactionTypeCustomEmoji>>(t: T) -> Self {
    ReactionType::CustomEmoji(t.as_ref().clone())
  }

  pub fn emoji<T: AsRef<ReactionTypeEmoji>>(t: T) -> Self {
    ReactionType::Emoji(t.as_ref().clone())
  }

  pub fn paid<T: AsRef<ReactionTypePaid>>(t: T) -> Self {
    ReactionType::Paid(t.as_ref().clone())
  }
}

impl AsRef<ReactionType> for ReactionType {
  fn as_ref(&self) -> &ReactionType {
    self
  }
}

/// A reaction with a custom emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionTypeCustomEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the custom emoji
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_id: isize,
}

impl RObject for ReactionTypeCustomEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionTypeCustomEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionType for ReactionTypeCustomEmoji {}

impl ReactionTypeCustomEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionTypeCustomEmojiBuilder {
    let inner = ReactionTypeCustomEmoji {
      td_name: "reactionTypeCustomEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReactionTypeCustomEmojiBuilder { inner }
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDReactionTypeCustomEmojiBuilder {
  inner: ReactionTypeCustomEmoji,
}

impl RTDReactionTypeCustomEmojiBuilder {
  pub fn build(&self) -> ReactionTypeCustomEmoji {
    self.inner.clone()
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }
}

impl AsRef<ReactionTypeCustomEmoji> for ReactionTypeCustomEmoji {
  fn as_ref(&self) -> &ReactionTypeCustomEmoji {
    self
  }
}

impl AsRef<ReactionTypeCustomEmoji> for RTDReactionTypeCustomEmojiBuilder {
  fn as_ref(&self) -> &ReactionTypeCustomEmoji {
    &self.inner
  }
}

/// A reaction with an emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionTypeEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text representation of the reaction
  emoji: String,
}

impl RObject for ReactionTypeEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionTypeEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionType for ReactionTypeEmoji {}

impl ReactionTypeEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionTypeEmojiBuilder {
    let inner = ReactionTypeEmoji {
      td_name: "reactionTypeEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReactionTypeEmojiBuilder { inner }
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }
}

#[doc(hidden)]
pub struct RTDReactionTypeEmojiBuilder {
  inner: ReactionTypeEmoji,
}

impl RTDReactionTypeEmojiBuilder {
  pub fn build(&self) -> ReactionTypeEmoji {
    self.inner.clone()
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }
}

impl AsRef<ReactionTypeEmoji> for ReactionTypeEmoji {
  fn as_ref(&self) -> &ReactionTypeEmoji {
    self
  }
}

impl AsRef<ReactionTypeEmoji> for RTDReactionTypeEmojiBuilder {
  fn as_ref(&self) -> &ReactionTypeEmoji {
    &self.inner
  }
}

/// The paid reaction in a channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionTypePaid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReactionTypePaid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionTypePaid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionType for ReactionTypePaid {}

impl ReactionTypePaid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionTypePaidBuilder {
    let inner = ReactionTypePaid {
      td_name: "reactionTypePaid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReactionTypePaidBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReactionTypePaidBuilder {
  inner: ReactionTypePaid,
}

impl RTDReactionTypePaidBuilder {
  pub fn build(&self) -> ReactionTypePaid {
    self.inner.clone()
  }
}

impl AsRef<ReactionTypePaid> for ReactionTypePaid {
  fn as_ref(&self) -> &ReactionTypePaid {
    self
  }
}

impl AsRef<ReactionTypePaid> for RTDReactionTypePaidBuilder {
  fn as_ref(&self) -> &ReactionTypePaid {
    &self.inner
  }
}
