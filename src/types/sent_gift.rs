use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents content of a gift received by a user or a channel chat
pub trait TDSentGift: Debug + RObject {}

/// Represents content of a gift received by a user or a channel chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SentGift {
  #[doc(hidden)]
  _Default(()),
  /// Regular gift
  Regular(SentGiftRegular),
  /// Upgraded gift
  Upgraded(SentGiftUpgraded),
}

impl Default for SentGift {
  fn default() -> Self {
    SentGift::_Default(())
  }
}

impl<'de> Deserialize<'de> for SentGift {
  fn deserialize<D>(deserializer: D) -> Result<SentGift, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SentGift,
      (sentGiftRegular, Regular);
      (sentGiftUpgraded, Upgraded);

    )(deserializer)
  }
}

impl RObject for SentGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SentGift::Regular(t) => t.td_name(),
      SentGift::Upgraded(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SentGift::Regular(t) => t.extra(),
      SentGift::Upgraded(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SentGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SentGift::_Default(_))
  }

  pub fn is_regular(&self) -> bool {
    matches!(self, SentGift::Regular(_))
  }
  pub fn is_upgraded(&self) -> bool {
    matches!(self, SentGift::Upgraded(_))
  }

  pub fn on_regular<F: FnOnce(&SentGiftRegular)>(&self, fnc: F) -> &Self {
    if let SentGift::Regular(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded<F: FnOnce(&SentGiftUpgraded)>(&self, fnc: F) -> &Self {
    if let SentGift::Upgraded(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_regular(&self) -> Option<&SentGiftRegular> {
    if let SentGift::Regular(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded(&self) -> Option<&SentGiftUpgraded> {
    if let SentGift::Upgraded(t) = self {
      return Some(t);
    }
    None
  }

  pub fn regular<T: AsRef<SentGiftRegular>>(t: T) -> Self {
    SentGift::Regular(t.as_ref().clone())
  }

  pub fn upgraded<T: AsRef<SentGiftUpgraded>>(t: T) -> Self {
    SentGift::Upgraded(t.as_ref().clone())
  }
}

impl AsRef<SentGift> for SentGift {
  fn as_ref(&self) -> &SentGift {
    self
  }
}

/// Regular gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SentGiftRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: Gift,
}

impl RObject for SentGiftRegular {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sentGiftRegular"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSentGift for SentGiftRegular {}

impl SentGiftRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSentGiftRegularBuilder {
    let inner = SentGiftRegular {
      td_name: "sentGiftRegular".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSentGiftRegularBuilder { inner }
  }

  pub fn gift(&self) -> &Gift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDSentGiftRegularBuilder {
  inner: SentGiftRegular,
}

impl RTDSentGiftRegularBuilder {
  pub fn build(&self) -> SentGiftRegular {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<Gift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<SentGiftRegular> for SentGiftRegular {
  fn as_ref(&self) -> &SentGiftRegular {
    self
  }
}

impl AsRef<SentGiftRegular> for RTDSentGiftRegularBuilder {
  fn as_ref(&self) -> &SentGiftRegular {
    &self.inner
  }
}

/// Upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SentGiftUpgraded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: UpgradedGift,
}

impl RObject for SentGiftUpgraded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sentGiftUpgraded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSentGift for SentGiftUpgraded {}

impl SentGiftUpgraded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSentGiftUpgradedBuilder {
    let inner = SentGiftUpgraded {
      td_name: "sentGiftUpgraded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSentGiftUpgradedBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDSentGiftUpgradedBuilder {
  inner: SentGiftUpgraded,
}

impl RTDSentGiftUpgradedBuilder {
  pub fn build(&self) -> SentGiftUpgraded {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<SentGiftUpgraded> for SentGiftUpgraded {
  fn as_ref(&self) -> &SentGiftUpgraded {
    self
  }
}

impl AsRef<SentGiftUpgraded> for RTDSentGiftUpgradedBuilder {
  fn as_ref(&self) -> &SentGiftUpgraded {
    &self.inner
  }
}
