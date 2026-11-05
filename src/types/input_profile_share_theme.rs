use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a theme of the screen for sharing a link to a user or a chat to be set
pub trait TDInputProfileShareTheme: Debug + RObject {}

/// Describes a theme of the screen for sharing a link to a user or a chat to be set
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputProfileShareTheme {
  #[doc(hidden)]
  _Default(()),
  /// A theme with emoji placed on a background
  Emoji(InputProfileShareThemeEmoji),
  /// A theme based on an upgraded gift
  UpgradedGift(InputProfileShareThemeUpgradedGift),
}

impl Default for InputProfileShareTheme {
  fn default() -> Self {
    InputProfileShareTheme::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputProfileShareTheme {
  fn deserialize<D>(deserializer: D) -> Result<InputProfileShareTheme, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputProfileShareTheme,
      (inputProfileShareThemeEmoji, Emoji);
      (inputProfileShareThemeUpgradedGift, UpgradedGift);

    )(deserializer)
  }
}

impl RObject for InputProfileShareTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputProfileShareTheme::Emoji(t) => t.td_name(),
      InputProfileShareTheme::UpgradedGift(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputProfileShareTheme::Emoji(t) => t.extra(),
      InputProfileShareTheme::UpgradedGift(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputProfileShareTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputProfileShareTheme::_Default(_))
  }

  pub fn is_emoji(&self) -> bool {
    matches!(self, InputProfileShareTheme::Emoji(_))
  }
  pub fn is_upgraded_gift(&self) -> bool {
    matches!(self, InputProfileShareTheme::UpgradedGift(_))
  }

  pub fn on_emoji<F: FnOnce(&InputProfileShareThemeEmoji)>(&self, fnc: F) -> &Self {
    if let InputProfileShareTheme::Emoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift<F: FnOnce(&InputProfileShareThemeUpgradedGift)>(&self, fnc: F) -> &Self {
    if let InputProfileShareTheme::UpgradedGift(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_emoji(&self) -> Option<&InputProfileShareThemeEmoji> {
    if let InputProfileShareTheme::Emoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift(&self) -> Option<&InputProfileShareThemeUpgradedGift> {
    if let InputProfileShareTheme::UpgradedGift(t) = self {
      return Some(t);
    }
    None
  }

  pub fn emoji<T: AsRef<InputProfileShareThemeEmoji>>(t: T) -> Self {
    InputProfileShareTheme::Emoji(t.as_ref().clone())
  }

  pub fn upgraded_gift<T: AsRef<InputProfileShareThemeUpgradedGift>>(t: T) -> Self {
    InputProfileShareTheme::UpgradedGift(t.as_ref().clone())
  }
}

impl AsRef<InputProfileShareTheme> for InputProfileShareTheme {
  fn as_ref(&self) -> &InputProfileShareTheme {
    self
  }
}

/// A theme with emoji placed on a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputProfileShareThemeEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The emoji; 0-6 regular or custom emoji. If empty, then use_default_background must be false
  emojis: Vec<EmojiType>,
  /// Pass true to use the default background; otherwise, the backdrop with identifier background_backdrop_id will be used as background
  use_default_background: bool,
  /// Identifier of the backdrop to be used as background; ignored if use_default_background is true. Use getProfileShareThemeBackdrops to get the list of available backdrops
  background_backdrop_id: i64,
}

impl RObject for InputProfileShareThemeEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputProfileShareThemeEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputProfileShareTheme for InputProfileShareThemeEmoji {}

impl InputProfileShareThemeEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputProfileShareThemeEmojiBuilder {
    let inner = InputProfileShareThemeEmoji {
      td_name: "inputProfileShareThemeEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputProfileShareThemeEmojiBuilder { inner }
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
pub struct RTDInputProfileShareThemeEmojiBuilder {
  inner: InputProfileShareThemeEmoji,
}

impl RTDInputProfileShareThemeEmojiBuilder {
  pub fn build(&self) -> InputProfileShareThemeEmoji {
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

impl AsRef<InputProfileShareThemeEmoji> for InputProfileShareThemeEmoji {
  fn as_ref(&self) -> &InputProfileShareThemeEmoji {
    self
  }
}

impl AsRef<InputProfileShareThemeEmoji> for RTDInputProfileShareThemeEmojiBuilder {
  fn as_ref(&self) -> &InputProfileShareThemeEmoji {
    &self.inner
  }
}

/// A theme based on an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputProfileShareThemeUpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the upgraded gift, which is owned by the user or the chat, for which the theme is set
  received_gift_id: String,
}

impl RObject for InputProfileShareThemeUpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputProfileShareThemeUpgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputProfileShareTheme for InputProfileShareThemeUpgradedGift {}

impl InputProfileShareThemeUpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputProfileShareThemeUpgradedGiftBuilder {
    let inner = InputProfileShareThemeUpgradedGift {
      td_name: "inputProfileShareThemeUpgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputProfileShareThemeUpgradedGiftBuilder { inner }
  }

  pub fn received_gift_id(&self) -> &String {
    &self.received_gift_id
  }
}

#[doc(hidden)]
pub struct RTDInputProfileShareThemeUpgradedGiftBuilder {
  inner: InputProfileShareThemeUpgradedGift,
}

impl RTDInputProfileShareThemeUpgradedGiftBuilder {
  pub fn build(&self) -> InputProfileShareThemeUpgradedGift {
    self.inner.clone()
  }

  pub fn received_gift_id<T: AsRef<str>>(&mut self, received_gift_id: T) -> &mut Self {
    self.inner.received_gift_id = received_gift_id.as_ref().to_string();
    self
  }
}

impl AsRef<InputProfileShareThemeUpgradedGift> for InputProfileShareThemeUpgradedGift {
  fn as_ref(&self) -> &InputProfileShareThemeUpgradedGift {
    self
  }
}

impl AsRef<InputProfileShareThemeUpgradedGift> for RTDInputProfileShareThemeUpgradedGiftBuilder {
  fn as_ref(&self) -> &InputProfileShareThemeUpgradedGift {
    &self.inner
  }
}
