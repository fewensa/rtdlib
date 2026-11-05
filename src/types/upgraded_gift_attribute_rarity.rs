use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes rarity of an upgraded gift attribute
pub trait TDUpgradedGiftAttributeRarity: Debug + RObject {}

/// Describes rarity of an upgraded gift attribute
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UpgradedGiftAttributeRarity {
  #[doc(hidden)]
  _Default(()),
  /// The attribute is epic
  Epic(UpgradedGiftAttributeRarityEpic),
  /// The attribute is legendary
  Legendary(UpgradedGiftAttributeRarityLegendary),
  /// The rarity is represented as the numeric frequency of the model
  PerMille(UpgradedGiftAttributeRarityPerMille),
  /// The attribute is rare
  Rare(UpgradedGiftAttributeRarityRare),
  /// The attribute is uncommon
  Uncommon(UpgradedGiftAttributeRarityUncommon),
}

impl Default for UpgradedGiftAttributeRarity {
  fn default() -> Self {
    UpgradedGiftAttributeRarity::_Default(())
  }
}

impl<'de> Deserialize<'de> for UpgradedGiftAttributeRarity {
  fn deserialize<D>(deserializer: D) -> Result<UpgradedGiftAttributeRarity, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UpgradedGiftAttributeRarity,
      (upgradedGiftAttributeRarityEpic, Epic);
      (upgradedGiftAttributeRarityLegendary, Legendary);
      (upgradedGiftAttributeRarityPerMille, PerMille);
      (upgradedGiftAttributeRarityRare, Rare);
      (upgradedGiftAttributeRarityUncommon, Uncommon);

    )(deserializer)
  }
}

impl RObject for UpgradedGiftAttributeRarity {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      UpgradedGiftAttributeRarity::Epic(t) => t.td_name(),
      UpgradedGiftAttributeRarity::Legendary(t) => t.td_name(),
      UpgradedGiftAttributeRarity::PerMille(t) => t.td_name(),
      UpgradedGiftAttributeRarity::Rare(t) => t.td_name(),
      UpgradedGiftAttributeRarity::Uncommon(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      UpgradedGiftAttributeRarity::Epic(t) => t.extra(),
      UpgradedGiftAttributeRarity::Legendary(t) => t.extra(),
      UpgradedGiftAttributeRarity::PerMille(t) => t.extra(),
      UpgradedGiftAttributeRarity::Rare(t) => t.extra(),
      UpgradedGiftAttributeRarity::Uncommon(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftAttributeRarity {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, UpgradedGiftAttributeRarity::_Default(_))
  }

  pub fn is_epic(&self) -> bool {
    matches!(self, UpgradedGiftAttributeRarity::Epic(_))
  }
  pub fn is_legendary(&self) -> bool {
    matches!(self, UpgradedGiftAttributeRarity::Legendary(_))
  }
  pub fn is_per_mille(&self) -> bool {
    matches!(self, UpgradedGiftAttributeRarity::PerMille(_))
  }
  pub fn is_rare(&self) -> bool {
    matches!(self, UpgradedGiftAttributeRarity::Rare(_))
  }
  pub fn is_uncommon(&self) -> bool {
    matches!(self, UpgradedGiftAttributeRarity::Uncommon(_))
  }

  pub fn on_epic<F: FnOnce(&UpgradedGiftAttributeRarityEpic)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftAttributeRarity::Epic(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_legendary<F: FnOnce(&UpgradedGiftAttributeRarityLegendary)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftAttributeRarity::Legendary(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_per_mille<F: FnOnce(&UpgradedGiftAttributeRarityPerMille)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftAttributeRarity::PerMille(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_rare<F: FnOnce(&UpgradedGiftAttributeRarityRare)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftAttributeRarity::Rare(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_uncommon<F: FnOnce(&UpgradedGiftAttributeRarityUncommon)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftAttributeRarity::Uncommon(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_epic(&self) -> Option<&UpgradedGiftAttributeRarityEpic> {
    if let UpgradedGiftAttributeRarity::Epic(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_legendary(&self) -> Option<&UpgradedGiftAttributeRarityLegendary> {
    if let UpgradedGiftAttributeRarity::Legendary(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_per_mille(&self) -> Option<&UpgradedGiftAttributeRarityPerMille> {
    if let UpgradedGiftAttributeRarity::PerMille(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_rare(&self) -> Option<&UpgradedGiftAttributeRarityRare> {
    if let UpgradedGiftAttributeRarity::Rare(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_uncommon(&self) -> Option<&UpgradedGiftAttributeRarityUncommon> {
    if let UpgradedGiftAttributeRarity::Uncommon(t) = self {
      return Some(t);
    }
    None
  }

  pub fn epic<T: AsRef<UpgradedGiftAttributeRarityEpic>>(t: T) -> Self {
    UpgradedGiftAttributeRarity::Epic(t.as_ref().clone())
  }

  pub fn legendary<T: AsRef<UpgradedGiftAttributeRarityLegendary>>(t: T) -> Self {
    UpgradedGiftAttributeRarity::Legendary(t.as_ref().clone())
  }

  pub fn per_mille<T: AsRef<UpgradedGiftAttributeRarityPerMille>>(t: T) -> Self {
    UpgradedGiftAttributeRarity::PerMille(t.as_ref().clone())
  }

  pub fn rare<T: AsRef<UpgradedGiftAttributeRarityRare>>(t: T) -> Self {
    UpgradedGiftAttributeRarity::Rare(t.as_ref().clone())
  }

  pub fn uncommon<T: AsRef<UpgradedGiftAttributeRarityUncommon>>(t: T) -> Self {
    UpgradedGiftAttributeRarity::Uncommon(t.as_ref().clone())
  }
}

impl AsRef<UpgradedGiftAttributeRarity> for UpgradedGiftAttributeRarity {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarity {
    self
  }
}

/// The attribute is epic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftAttributeRarityEpic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UpgradedGiftAttributeRarityEpic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftAttributeRarityEpic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftAttributeRarity for UpgradedGiftAttributeRarityEpic {}

impl UpgradedGiftAttributeRarityEpic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftAttributeRarityEpicBuilder {
    let inner = UpgradedGiftAttributeRarityEpic {
      td_name: "upgradedGiftAttributeRarityEpic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUpgradedGiftAttributeRarityEpicBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftAttributeRarityEpicBuilder {
  inner: UpgradedGiftAttributeRarityEpic,
}

impl RTDUpgradedGiftAttributeRarityEpicBuilder {
  pub fn build(&self) -> UpgradedGiftAttributeRarityEpic {
    self.inner.clone()
  }
}

impl AsRef<UpgradedGiftAttributeRarityEpic> for UpgradedGiftAttributeRarityEpic {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityEpic {
    self
  }
}

impl AsRef<UpgradedGiftAttributeRarityEpic> for RTDUpgradedGiftAttributeRarityEpicBuilder {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityEpic {
    &self.inner
  }
}

/// The attribute is legendary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftAttributeRarityLegendary {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UpgradedGiftAttributeRarityLegendary {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftAttributeRarityLegendary"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftAttributeRarity for UpgradedGiftAttributeRarityLegendary {}

impl UpgradedGiftAttributeRarityLegendary {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftAttributeRarityLegendaryBuilder {
    let inner = UpgradedGiftAttributeRarityLegendary {
      td_name: "upgradedGiftAttributeRarityLegendary".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUpgradedGiftAttributeRarityLegendaryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftAttributeRarityLegendaryBuilder {
  inner: UpgradedGiftAttributeRarityLegendary,
}

impl RTDUpgradedGiftAttributeRarityLegendaryBuilder {
  pub fn build(&self) -> UpgradedGiftAttributeRarityLegendary {
    self.inner.clone()
  }
}

impl AsRef<UpgradedGiftAttributeRarityLegendary> for UpgradedGiftAttributeRarityLegendary {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityLegendary {
    self
  }
}

impl AsRef<UpgradedGiftAttributeRarityLegendary>
  for RTDUpgradedGiftAttributeRarityLegendaryBuilder
{
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityLegendary {
    &self.inner
  }
}

/// The rarity is represented as the numeric frequency of the model
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftAttributeRarityPerMille {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of upgraded gifts that receive this attribute for each 1000 gifts upgraded; if 0, then it can be shown as "<0.1%"
  per_mille: i64,
}

impl RObject for UpgradedGiftAttributeRarityPerMille {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftAttributeRarityPerMille"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftAttributeRarity for UpgradedGiftAttributeRarityPerMille {}

impl UpgradedGiftAttributeRarityPerMille {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftAttributeRarityPerMilleBuilder {
    let inner = UpgradedGiftAttributeRarityPerMille {
      td_name: "upgradedGiftAttributeRarityPerMille".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftAttributeRarityPerMilleBuilder { inner }
  }

  pub fn per_mille(&self) -> i64 {
    self.per_mille
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftAttributeRarityPerMilleBuilder {
  inner: UpgradedGiftAttributeRarityPerMille,
}

impl RTDUpgradedGiftAttributeRarityPerMilleBuilder {
  pub fn build(&self) -> UpgradedGiftAttributeRarityPerMille {
    self.inner.clone()
  }

  pub fn per_mille(&mut self, per_mille: i64) -> &mut Self {
    self.inner.per_mille = per_mille;
    self
  }
}

impl AsRef<UpgradedGiftAttributeRarityPerMille> for UpgradedGiftAttributeRarityPerMille {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityPerMille {
    self
  }
}

impl AsRef<UpgradedGiftAttributeRarityPerMille> for RTDUpgradedGiftAttributeRarityPerMilleBuilder {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityPerMille {
    &self.inner
  }
}

/// The attribute is rare
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftAttributeRarityRare {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UpgradedGiftAttributeRarityRare {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftAttributeRarityRare"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftAttributeRarity for UpgradedGiftAttributeRarityRare {}

impl UpgradedGiftAttributeRarityRare {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftAttributeRarityRareBuilder {
    let inner = UpgradedGiftAttributeRarityRare {
      td_name: "upgradedGiftAttributeRarityRare".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUpgradedGiftAttributeRarityRareBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftAttributeRarityRareBuilder {
  inner: UpgradedGiftAttributeRarityRare,
}

impl RTDUpgradedGiftAttributeRarityRareBuilder {
  pub fn build(&self) -> UpgradedGiftAttributeRarityRare {
    self.inner.clone()
  }
}

impl AsRef<UpgradedGiftAttributeRarityRare> for UpgradedGiftAttributeRarityRare {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityRare {
    self
  }
}

impl AsRef<UpgradedGiftAttributeRarityRare> for RTDUpgradedGiftAttributeRarityRareBuilder {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityRare {
    &self.inner
  }
}

/// The attribute is uncommon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftAttributeRarityUncommon {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UpgradedGiftAttributeRarityUncommon {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftAttributeRarityUncommon"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftAttributeRarity for UpgradedGiftAttributeRarityUncommon {}

impl UpgradedGiftAttributeRarityUncommon {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftAttributeRarityUncommonBuilder {
    let inner = UpgradedGiftAttributeRarityUncommon {
      td_name: "upgradedGiftAttributeRarityUncommon".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUpgradedGiftAttributeRarityUncommonBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftAttributeRarityUncommonBuilder {
  inner: UpgradedGiftAttributeRarityUncommon,
}

impl RTDUpgradedGiftAttributeRarityUncommonBuilder {
  pub fn build(&self) -> UpgradedGiftAttributeRarityUncommon {
    self.inner.clone()
  }
}

impl AsRef<UpgradedGiftAttributeRarityUncommon> for UpgradedGiftAttributeRarityUncommon {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityUncommon {
    self
  }
}

impl AsRef<UpgradedGiftAttributeRarityUncommon> for RTDUpgradedGiftAttributeRarityUncommonBuilder {
  fn as_ref(&self) -> &UpgradedGiftAttributeRarityUncommon {
    &self.inner
  }
}
