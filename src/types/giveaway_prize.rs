use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about a giveaway prize
pub trait TDGiveawayPrize: Debug + RObject {}

/// Contains information about a giveaway prize
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiveawayPrize {
  #[doc(hidden)]
  _Default(()),
  /// The giveaway sends Telegram Premium subscriptions to the winners
  Premium(GiveawayPrizePremium),
  /// The giveaway sends Telegram Stars to the winners
  Stars(GiveawayPrizeStars),
}

impl Default for GiveawayPrize {
  fn default() -> Self {
    GiveawayPrize::_Default(())
  }
}

impl<'de> Deserialize<'de> for GiveawayPrize {
  fn deserialize<D>(deserializer: D) -> Result<GiveawayPrize, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GiveawayPrize,
      (giveawayPrizePremium, Premium);
      (giveawayPrizeStars, Stars);

    )(deserializer)
  }
}

impl RObject for GiveawayPrize {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      GiveawayPrize::Premium(t) => t.td_name(),
      GiveawayPrize::Stars(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      GiveawayPrize::Premium(t) => t.extra(),
      GiveawayPrize::Stars(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiveawayPrize {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, GiveawayPrize::_Default(_))
  }

  pub fn is_premium(&self) -> bool {
    matches!(self, GiveawayPrize::Premium(_))
  }
  pub fn is_stars(&self) -> bool {
    matches!(self, GiveawayPrize::Stars(_))
  }

  pub fn on_premium<F: FnOnce(&GiveawayPrizePremium)>(&self, fnc: F) -> &Self {
    if let GiveawayPrize::Premium(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stars<F: FnOnce(&GiveawayPrizeStars)>(&self, fnc: F) -> &Self {
    if let GiveawayPrize::Stars(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_premium(&self) -> Option<&GiveawayPrizePremium> {
    if let GiveawayPrize::Premium(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stars(&self) -> Option<&GiveawayPrizeStars> {
    if let GiveawayPrize::Stars(t) = self {
      return Some(t);
    }
    None
  }

  pub fn premium<T: AsRef<GiveawayPrizePremium>>(t: T) -> Self {
    GiveawayPrize::Premium(t.as_ref().clone())
  }

  pub fn stars<T: AsRef<GiveawayPrizeStars>>(t: T) -> Self {
    GiveawayPrize::Stars(t.as_ref().clone())
  }
}

impl AsRef<GiveawayPrize> for GiveawayPrize {
  fn as_ref(&self) -> &GiveawayPrize {
    self
  }
}

/// The giveaway sends Telegram Premium subscriptions to the winners
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayPrizePremium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of months the Telegram Premium subscription will be active after code activation
  month_count: i64,
}

impl RObject for GiveawayPrizePremium {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayPrizePremium"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayPrize for GiveawayPrizePremium {}

impl GiveawayPrizePremium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayPrizePremiumBuilder {
    let inner = GiveawayPrizePremium {
      td_name: "giveawayPrizePremium".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiveawayPrizePremiumBuilder { inner }
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }
}

#[doc(hidden)]
pub struct RTDGiveawayPrizePremiumBuilder {
  inner: GiveawayPrizePremium,
}

impl RTDGiveawayPrizePremiumBuilder {
  pub fn build(&self) -> GiveawayPrizePremium {
    self.inner.clone()
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }
}

impl AsRef<GiveawayPrizePremium> for GiveawayPrizePremium {
  fn as_ref(&self) -> &GiveawayPrizePremium {
    self
  }
}

impl AsRef<GiveawayPrizePremium> for RTDGiveawayPrizePremiumBuilder {
  fn as_ref(&self) -> &GiveawayPrizePremium {
    &self.inner
  }
}

/// The giveaway sends Telegram Stars to the winners
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayPrizeStars {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of Telegram Stars that will be shared by all winners
  star_count: i64,
}

impl RObject for GiveawayPrizeStars {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayPrizeStars"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayPrize for GiveawayPrizeStars {}

impl GiveawayPrizeStars {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayPrizeStarsBuilder {
    let inner = GiveawayPrizeStars {
      td_name: "giveawayPrizeStars".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiveawayPrizeStarsBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDGiveawayPrizeStarsBuilder {
  inner: GiveawayPrizeStars,
}

impl RTDGiveawayPrizeStarsBuilder {
  pub fn build(&self) -> GiveawayPrizeStars {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<GiveawayPrizeStars> for GiveawayPrizeStars {
  fn as_ref(&self) -> &GiveawayPrizeStars {
    self
  }
}

impl AsRef<GiveawayPrizeStars> for RTDGiveawayPrizeStarsBuilder {
  fn as_ref(&self) -> &GiveawayPrizeStars {
    &self.inner
  }
}
