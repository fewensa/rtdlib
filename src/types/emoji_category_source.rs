use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes source of stickers for an emoji category
pub trait TDEmojiCategorySource: Debug + RObject {}

/// Describes source of stickers for an emoji category
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EmojiCategorySource {
  #[doc(hidden)]
  _Default(()),
  /// The category contains premium stickers that must be found by getPremiumStickers
  Premium(EmojiCategorySourcePremium),
  /// The category contains a list of similar emoji to search for in getStickers and searchStickers for stickers, or getInlineQueryResults with the bot getOption("animation_search_bot_username") for animations
  Search(EmojiCategorySourceSearch),
}

impl Default for EmojiCategorySource {
  fn default() -> Self {
    EmojiCategorySource::_Default(())
  }
}

impl<'de> Deserialize<'de> for EmojiCategorySource {
  fn deserialize<D>(deserializer: D) -> Result<EmojiCategorySource, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      EmojiCategorySource,
      (emojiCategorySourcePremium, Premium);
      (emojiCategorySourceSearch, Search);

    )(deserializer)
  }
}

impl RObject for EmojiCategorySource {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      EmojiCategorySource::Premium(t) => t.td_name(),
      EmojiCategorySource::Search(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      EmojiCategorySource::Premium(t) => t.extra(),
      EmojiCategorySource::Search(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiCategorySource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, EmojiCategorySource::_Default(_))
  }

  pub fn is_premium(&self) -> bool {
    matches!(self, EmojiCategorySource::Premium(_))
  }
  pub fn is_search(&self) -> bool {
    matches!(self, EmojiCategorySource::Search(_))
  }

  pub fn on_premium<F: FnOnce(&EmojiCategorySourcePremium)>(&self, fnc: F) -> &Self {
    if let EmojiCategorySource::Premium(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_search<F: FnOnce(&EmojiCategorySourceSearch)>(&self, fnc: F) -> &Self {
    if let EmojiCategorySource::Search(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_premium(&self) -> Option<&EmojiCategorySourcePremium> {
    if let EmojiCategorySource::Premium(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_search(&self) -> Option<&EmojiCategorySourceSearch> {
    if let EmojiCategorySource::Search(t) = self {
      return Some(t);
    }
    None
  }

  pub fn premium<T: AsRef<EmojiCategorySourcePremium>>(t: T) -> Self {
    EmojiCategorySource::Premium(t.as_ref().clone())
  }

  pub fn search<T: AsRef<EmojiCategorySourceSearch>>(t: T) -> Self {
    EmojiCategorySource::Search(t.as_ref().clone())
  }
}

impl AsRef<EmojiCategorySource> for EmojiCategorySource {
  fn as_ref(&self) -> &EmojiCategorySource {
    self
  }
}

/// The category contains premium stickers that must be found by getPremiumStickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategorySourcePremium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for EmojiCategorySourcePremium {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiCategorySourcePremium"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiCategorySource for EmojiCategorySourcePremium {}

impl EmojiCategorySourcePremium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiCategorySourcePremiumBuilder {
    let inner = EmojiCategorySourcePremium {
      td_name: "emojiCategorySourcePremium".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDEmojiCategorySourcePremiumBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDEmojiCategorySourcePremiumBuilder {
  inner: EmojiCategorySourcePremium,
}

impl RTDEmojiCategorySourcePremiumBuilder {
  pub fn build(&self) -> EmojiCategorySourcePremium {
    self.inner.clone()
  }
}

impl AsRef<EmojiCategorySourcePremium> for EmojiCategorySourcePremium {
  fn as_ref(&self) -> &EmojiCategorySourcePremium {
    self
  }
}

impl AsRef<EmojiCategorySourcePremium> for RTDEmojiCategorySourcePremiumBuilder {
  fn as_ref(&self) -> &EmojiCategorySourcePremium {
    &self.inner
  }
}

/// The category contains a list of similar emoji to search for in getStickers and searchStickers for stickers, or getInlineQueryResults with the bot getOption("animation_search_bot_username") for animations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategorySourceSearch {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of emojis to search for
  emojis: Vec<String>,
}

impl RObject for EmojiCategorySourceSearch {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiCategorySourceSearch"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmojiCategorySource for EmojiCategorySourceSearch {}

impl EmojiCategorySourceSearch {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiCategorySourceSearchBuilder {
    let inner = EmojiCategorySourceSearch {
      td_name: "emojiCategorySourceSearch".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiCategorySourceSearchBuilder { inner }
  }

  pub fn emojis(&self) -> &Vec<String> {
    &self.emojis
  }
}

#[doc(hidden)]
pub struct RTDEmojiCategorySourceSearchBuilder {
  inner: EmojiCategorySourceSearch,
}

impl RTDEmojiCategorySourceSearchBuilder {
  pub fn build(&self) -> EmojiCategorySourceSearch {
    self.inner.clone()
  }

  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }
}

impl AsRef<EmojiCategorySourceSearch> for EmojiCategorySourceSearch {
  fn as_ref(&self) -> &EmojiCategorySourceSearch {
    self
  }
}

impl AsRef<EmojiCategorySourceSearch> for RTDEmojiCategorySourceSearchBuilder {
  fn as_ref(&self) -> &EmojiCategorySourceSearch {
    &self.inner
  }
}
