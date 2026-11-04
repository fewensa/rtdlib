use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains identifier of an upgraded gift attribute to search for
pub trait TDUpgradedGiftAttributeId: Debug + RObject {}

/// Contains identifier of an upgraded gift attribute to search for
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UpgradedGiftAttributeId {
  #[doc(hidden)]
  _Default(()),
  /// Identifier of a gift backdrop
  Backdrop(UpgradedGiftAttributeIdBackdrop),
  /// Identifier of a gift model
  Model(UpgradedGiftAttributeIdModel),
  /// Identifier of a gift symbol
  Symbol(UpgradedGiftAttributeIdSymbol),
}

impl Default for UpgradedGiftAttributeId {
  fn default() -> Self {
    UpgradedGiftAttributeId::_Default(())
  }
}

impl<'de> Deserialize<'de> for UpgradedGiftAttributeId {
  fn deserialize<D>(deserializer: D) -> Result<UpgradedGiftAttributeId, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UpgradedGiftAttributeId,
      (upgradedGiftAttributeIdBackdrop, Backdrop);
      (upgradedGiftAttributeIdModel, Model);
      (upgradedGiftAttributeIdSymbol, Symbol);

    )(deserializer)
  }
}

impl RObject for UpgradedGiftAttributeId {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      UpgradedGiftAttributeId::Backdrop(t) => t.td_name(),
      UpgradedGiftAttributeId::Model(t) => t.td_name(),
      UpgradedGiftAttributeId::Symbol(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      UpgradedGiftAttributeId::Backdrop(t) => t.extra(),
      UpgradedGiftAttributeId::Model(t) => t.extra(),
      UpgradedGiftAttributeId::Symbol(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftAttributeId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, UpgradedGiftAttributeId::_Default(_))
  }

  pub fn is_backdrop(&self) -> bool {
    matches!(self, UpgradedGiftAttributeId::Backdrop(_))
  }
  pub fn is_model(&self) -> bool {
    matches!(self, UpgradedGiftAttributeId::Model(_))
  }
  pub fn is_symbol(&self) -> bool {
    matches!(self, UpgradedGiftAttributeId::Symbol(_))
  }

  pub fn on_backdrop<F: FnOnce(&UpgradedGiftAttributeIdBackdrop)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftAttributeId::Backdrop(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_model<F: FnOnce(&UpgradedGiftAttributeIdModel)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftAttributeId::Model(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_symbol<F: FnOnce(&UpgradedGiftAttributeIdSymbol)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftAttributeId::Symbol(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_backdrop(&self) -> Option<&UpgradedGiftAttributeIdBackdrop> {
    if let UpgradedGiftAttributeId::Backdrop(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_model(&self) -> Option<&UpgradedGiftAttributeIdModel> {
    if let UpgradedGiftAttributeId::Model(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_symbol(&self) -> Option<&UpgradedGiftAttributeIdSymbol> {
    if let UpgradedGiftAttributeId::Symbol(t) = self {
      return Some(t);
    }
    None
  }

  pub fn backdrop<T: AsRef<UpgradedGiftAttributeIdBackdrop>>(t: T) -> Self {
    UpgradedGiftAttributeId::Backdrop(t.as_ref().clone())
  }

  pub fn model<T: AsRef<UpgradedGiftAttributeIdModel>>(t: T) -> Self {
    UpgradedGiftAttributeId::Model(t.as_ref().clone())
  }

  pub fn symbol<T: AsRef<UpgradedGiftAttributeIdSymbol>>(t: T) -> Self {
    UpgradedGiftAttributeId::Symbol(t.as_ref().clone())
  }
}

impl AsRef<UpgradedGiftAttributeId> for UpgradedGiftAttributeId {
  fn as_ref(&self) -> &UpgradedGiftAttributeId {
    self
  }
}

/// Identifier of a gift backdrop
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftAttributeIdBackdrop {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the backdrop
  backdrop_id: i64,
}

impl RObject for UpgradedGiftAttributeIdBackdrop {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftAttributeIdBackdrop"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftAttributeId for UpgradedGiftAttributeIdBackdrop {}

impl UpgradedGiftAttributeIdBackdrop {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftAttributeIdBackdropBuilder {
    let inner = UpgradedGiftAttributeIdBackdrop {
      td_name: "upgradedGiftAttributeIdBackdrop".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftAttributeIdBackdropBuilder { inner }
  }

  pub fn backdrop_id(&self) -> i64 {
    self.backdrop_id
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftAttributeIdBackdropBuilder {
  inner: UpgradedGiftAttributeIdBackdrop,
}

impl RTDUpgradedGiftAttributeIdBackdropBuilder {
  pub fn build(&self) -> UpgradedGiftAttributeIdBackdrop {
    self.inner.clone()
  }

  pub fn backdrop_id(&mut self, backdrop_id: i64) -> &mut Self {
    self.inner.backdrop_id = backdrop_id;
    self
  }
}

impl AsRef<UpgradedGiftAttributeIdBackdrop> for UpgradedGiftAttributeIdBackdrop {
  fn as_ref(&self) -> &UpgradedGiftAttributeIdBackdrop {
    self
  }
}

impl AsRef<UpgradedGiftAttributeIdBackdrop> for RTDUpgradedGiftAttributeIdBackdropBuilder {
  fn as_ref(&self) -> &UpgradedGiftAttributeIdBackdrop {
    &self.inner
  }
}

/// Identifier of a gift model
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftAttributeIdModel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the sticker representing the model
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  sticker_id: isize,
}

impl RObject for UpgradedGiftAttributeIdModel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftAttributeIdModel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftAttributeId for UpgradedGiftAttributeIdModel {}

impl UpgradedGiftAttributeIdModel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftAttributeIdModelBuilder {
    let inner = UpgradedGiftAttributeIdModel {
      td_name: "upgradedGiftAttributeIdModel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftAttributeIdModelBuilder { inner }
  }

  pub fn sticker_id(&self) -> isize {
    self.sticker_id
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftAttributeIdModelBuilder {
  inner: UpgradedGiftAttributeIdModel,
}

impl RTDUpgradedGiftAttributeIdModelBuilder {
  pub fn build(&self) -> UpgradedGiftAttributeIdModel {
    self.inner.clone()
  }

  pub fn sticker_id(&mut self, sticker_id: isize) -> &mut Self {
    self.inner.sticker_id = sticker_id;
    self
  }
}

impl AsRef<UpgradedGiftAttributeIdModel> for UpgradedGiftAttributeIdModel {
  fn as_ref(&self) -> &UpgradedGiftAttributeIdModel {
    self
  }
}

impl AsRef<UpgradedGiftAttributeIdModel> for RTDUpgradedGiftAttributeIdModelBuilder {
  fn as_ref(&self) -> &UpgradedGiftAttributeIdModel {
    &self.inner
  }
}

/// Identifier of a gift symbol
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftAttributeIdSymbol {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the sticker representing the symbol
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  sticker_id: isize,
}

impl RObject for UpgradedGiftAttributeIdSymbol {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftAttributeIdSymbol"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftAttributeId for UpgradedGiftAttributeIdSymbol {}

impl UpgradedGiftAttributeIdSymbol {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftAttributeIdSymbolBuilder {
    let inner = UpgradedGiftAttributeIdSymbol {
      td_name: "upgradedGiftAttributeIdSymbol".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftAttributeIdSymbolBuilder { inner }
  }

  pub fn sticker_id(&self) -> isize {
    self.sticker_id
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftAttributeIdSymbolBuilder {
  inner: UpgradedGiftAttributeIdSymbol,
}

impl RTDUpgradedGiftAttributeIdSymbolBuilder {
  pub fn build(&self) -> UpgradedGiftAttributeIdSymbol {
    self.inner.clone()
  }

  pub fn sticker_id(&mut self, sticker_id: isize) -> &mut Self {
    self.inner.sticker_id = sticker_id;
    self
  }
}

impl AsRef<UpgradedGiftAttributeIdSymbol> for UpgradedGiftAttributeIdSymbol {
  fn as_ref(&self) -> &UpgradedGiftAttributeIdSymbol {
    self
  }
}

impl AsRef<UpgradedGiftAttributeIdSymbol> for RTDUpgradedGiftAttributeIdSymbolBuilder {
  fn as_ref(&self) -> &UpgradedGiftAttributeIdSymbol {
    &self.inner
  }
}
