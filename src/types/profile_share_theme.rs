use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a theme of the screen for sharing a link to a user or a chat
pub trait TDProfileShareTheme: Debug + RObject {}

/// Describes a theme of the screen for sharing a link to a user or a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ProfileShareTheme {
  #[doc(hidden)]
  _Default(()),
  /// A theme with emoji placed on a background
  Emoji(ProfileShareThemeEmoji),
  /// A theme based on an upgraded gift owned by the user or the chat
  UpgradedGift(ProfileShareThemeUpgradedGift),
}

impl Default for ProfileShareTheme {
  fn default() -> Self {
    ProfileShareTheme::_Default(())
  }
}

impl<'de> Deserialize<'de> for ProfileShareTheme {
  fn deserialize<D>(deserializer: D) -> Result<ProfileShareTheme, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ProfileShareTheme,
      (profileShareThemeEmoji, Emoji);
      (profileShareThemeUpgradedGift, UpgradedGift);

    )(deserializer)
  }
}

impl RObject for ProfileShareTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ProfileShareTheme::Emoji(t) => t.td_name(),
      ProfileShareTheme::UpgradedGift(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ProfileShareTheme::Emoji(t) => t.extra(),
      ProfileShareTheme::UpgradedGift(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ProfileShareTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ProfileShareTheme::_Default(_))
  }

  pub fn is_emoji(&self) -> bool {
    matches!(self, ProfileShareTheme::Emoji(_))
  }
  pub fn is_upgraded_gift(&self) -> bool {
    matches!(self, ProfileShareTheme::UpgradedGift(_))
  }

  pub fn on_emoji<F: FnOnce(&ProfileShareThemeEmoji)>(&self, fnc: F) -> &Self {
    if let ProfileShareTheme::Emoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift<F: FnOnce(&ProfileShareThemeUpgradedGift)>(&self, fnc: F) -> &Self {
    if let ProfileShareTheme::UpgradedGift(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_emoji(&self) -> Option<&ProfileShareThemeEmoji> {
    if let ProfileShareTheme::Emoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift(&self) -> Option<&ProfileShareThemeUpgradedGift> {
    if let ProfileShareTheme::UpgradedGift(t) = self {
      return Some(t);
    }
    None
  }

  pub fn emoji<T: AsRef<ProfileShareThemeEmoji>>(t: T) -> Self {
    ProfileShareTheme::Emoji(t.as_ref().clone())
  }

  pub fn upgraded_gift<T: AsRef<ProfileShareThemeUpgradedGift>>(t: T) -> Self {
    ProfileShareTheme::UpgradedGift(t.as_ref().clone())
  }
}

impl AsRef<ProfileShareTheme> for ProfileShareTheme {
  fn as_ref(&self) -> &ProfileShareTheme {
    self
  }
}

/// A theme with emoji placed on a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileShareThemeEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The emoji; 0-6 regular or custom emoji. If empty, then use_default_background is false
  emojis: Vec<EmojiType>,
  /// True, if the default background is used; otherwise, the backdrop with identifier background_backdrop_id is used as background
  use_default_background: bool,
  /// Identifier of the backdrop used as background; must be ignored if use_default_background is true. Use getProfileShareThemeBackdrops to get details about the backdrop
  background_backdrop_id: i64,
}

impl RObject for ProfileShareThemeEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileShareThemeEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileShareTheme for ProfileShareThemeEmoji {}

impl ProfileShareThemeEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileShareThemeEmojiBuilder {
    let inner = ProfileShareThemeEmoji {
      td_name: "profileShareThemeEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDProfileShareThemeEmojiBuilder { inner }
  }

  pub fn emojis(&self) -> &Vec<EmojiType> {
    &self.emojis
  }

  pub fn use_default_background(&self) -> bool {
    self.use_default_background
  }

  pub fn background_backdrop_id(&self) -> i64 {
    self.background_backdrop_id
  }
}

#[doc(hidden)]
pub struct RTDProfileShareThemeEmojiBuilder {
  inner: ProfileShareThemeEmoji,
}

impl RTDProfileShareThemeEmojiBuilder {
  pub fn build(&self) -> ProfileShareThemeEmoji {
    self.inner.clone()
  }

  pub fn emojis(&mut self, emojis: Vec<EmojiType>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }

  pub fn use_default_background(&mut self, use_default_background: bool) -> &mut Self {
    self.inner.use_default_background = use_default_background;
    self
  }

  pub fn background_backdrop_id(&mut self, background_backdrop_id: i64) -> &mut Self {
    self.inner.background_backdrop_id = background_backdrop_id;
    self
  }
}

impl AsRef<ProfileShareThemeEmoji> for ProfileShareThemeEmoji {
  fn as_ref(&self) -> &ProfileShareThemeEmoji {
    self
  }
}

impl AsRef<ProfileShareThemeEmoji> for RTDProfileShareThemeEmojiBuilder {
  fn as_ref(&self) -> &ProfileShareThemeEmoji {
    &self.inner
  }
}

/// A theme based on an upgraded gift owned by the user or the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileShareThemeUpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The upgraded gift
  gift: UpgradedGift,
}

impl RObject for ProfileShareThemeUpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileShareThemeUpgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDProfileShareTheme for ProfileShareThemeUpgradedGift {}

impl ProfileShareThemeUpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileShareThemeUpgradedGiftBuilder {
    let inner = ProfileShareThemeUpgradedGift {
      td_name: "profileShareThemeUpgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDProfileShareThemeUpgradedGiftBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDProfileShareThemeUpgradedGiftBuilder {
  inner: ProfileShareThemeUpgradedGift,
}

impl RTDProfileShareThemeUpgradedGiftBuilder {
  pub fn build(&self) -> ProfileShareThemeUpgradedGift {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<ProfileShareThemeUpgradedGift> for ProfileShareThemeUpgradedGift {
  fn as_ref(&self) -> &ProfileShareThemeUpgradedGift {
    self
  }
}

impl AsRef<ProfileShareThemeUpgradedGift> for RTDProfileShareThemeUpgradedGiftBuilder {
  fn as_ref(&self) -> &ProfileShareThemeUpgradedGift {
    &self.inner
  }
}
