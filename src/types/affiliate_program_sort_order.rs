use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the order of the found affiliate programs
pub trait TDAffiliateProgramSortOrder: Debug + RObject {}

/// Describes the order of the found affiliate programs
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AffiliateProgramSortOrder {
  #[doc(hidden)]
  _Default(()),
  /// The affiliate programs must be sorted by creation date
  CreationDate(AffiliateProgramSortOrderCreationDate),
  /// The affiliate programs must be sorted by the profitability
  Profitability(AffiliateProgramSortOrderProfitability),
  /// The affiliate programs must be sorted by the expected revenue
  Revenue(AffiliateProgramSortOrderRevenue),
}

impl Default for AffiliateProgramSortOrder {
  fn default() -> Self {
    AffiliateProgramSortOrder::_Default(())
  }
}

impl<'de> Deserialize<'de> for AffiliateProgramSortOrder {
  fn deserialize<D>(deserializer: D) -> Result<AffiliateProgramSortOrder, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AffiliateProgramSortOrder,
      (affiliateProgramSortOrderCreationDate, CreationDate);
      (affiliateProgramSortOrderProfitability, Profitability);
      (affiliateProgramSortOrderRevenue, Revenue);

    )(deserializer)
  }
}

impl RObject for AffiliateProgramSortOrder {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      AffiliateProgramSortOrder::CreationDate(t) => t.td_name(),
      AffiliateProgramSortOrder::Profitability(t) => t.td_name(),
      AffiliateProgramSortOrder::Revenue(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      AffiliateProgramSortOrder::CreationDate(t) => t.extra(),
      AffiliateProgramSortOrder::Profitability(t) => t.extra(),
      AffiliateProgramSortOrder::Revenue(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AffiliateProgramSortOrder {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, AffiliateProgramSortOrder::_Default(_))
  }

  pub fn is_creation_date(&self) -> bool {
    matches!(self, AffiliateProgramSortOrder::CreationDate(_))
  }
  pub fn is_profitability(&self) -> bool {
    matches!(self, AffiliateProgramSortOrder::Profitability(_))
  }
  pub fn is_revenue(&self) -> bool {
    matches!(self, AffiliateProgramSortOrder::Revenue(_))
  }

  pub fn on_creation_date<F: FnOnce(&AffiliateProgramSortOrderCreationDate)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AffiliateProgramSortOrder::CreationDate(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_profitability<F: FnOnce(&AffiliateProgramSortOrderProfitability)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AffiliateProgramSortOrder::Profitability(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_revenue<F: FnOnce(&AffiliateProgramSortOrderRevenue)>(&self, fnc: F) -> &Self {
    if let AffiliateProgramSortOrder::Revenue(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_creation_date(&self) -> Option<&AffiliateProgramSortOrderCreationDate> {
    if let AffiliateProgramSortOrder::CreationDate(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_profitability(&self) -> Option<&AffiliateProgramSortOrderProfitability> {
    if let AffiliateProgramSortOrder::Profitability(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_revenue(&self) -> Option<&AffiliateProgramSortOrderRevenue> {
    if let AffiliateProgramSortOrder::Revenue(t) = self {
      return Some(t);
    }
    None
  }

  pub fn creation_date<T: AsRef<AffiliateProgramSortOrderCreationDate>>(t: T) -> Self {
    AffiliateProgramSortOrder::CreationDate(t.as_ref().clone())
  }

  pub fn profitability<T: AsRef<AffiliateProgramSortOrderProfitability>>(t: T) -> Self {
    AffiliateProgramSortOrder::Profitability(t.as_ref().clone())
  }

  pub fn revenue<T: AsRef<AffiliateProgramSortOrderRevenue>>(t: T) -> Self {
    AffiliateProgramSortOrder::Revenue(t.as_ref().clone())
  }
}

impl AsRef<AffiliateProgramSortOrder> for AffiliateProgramSortOrder {
  fn as_ref(&self) -> &AffiliateProgramSortOrder {
    self
  }
}

/// The affiliate programs must be sorted by creation date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateProgramSortOrderCreationDate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AffiliateProgramSortOrderCreationDate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateProgramSortOrderCreationDate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAffiliateProgramSortOrder for AffiliateProgramSortOrderCreationDate {}

impl AffiliateProgramSortOrderCreationDate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateProgramSortOrderCreationDateBuilder {
    let inner = AffiliateProgramSortOrderCreationDate {
      td_name: "affiliateProgramSortOrderCreationDate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAffiliateProgramSortOrderCreationDateBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAffiliateProgramSortOrderCreationDateBuilder {
  inner: AffiliateProgramSortOrderCreationDate,
}

impl RTDAffiliateProgramSortOrderCreationDateBuilder {
  pub fn build(&self) -> AffiliateProgramSortOrderCreationDate {
    self.inner.clone()
  }
}

impl AsRef<AffiliateProgramSortOrderCreationDate> for AffiliateProgramSortOrderCreationDate {
  fn as_ref(&self) -> &AffiliateProgramSortOrderCreationDate {
    self
  }
}

impl AsRef<AffiliateProgramSortOrderCreationDate>
  for RTDAffiliateProgramSortOrderCreationDateBuilder
{
  fn as_ref(&self) -> &AffiliateProgramSortOrderCreationDate {
    &self.inner
  }
}

/// The affiliate programs must be sorted by the profitability
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateProgramSortOrderProfitability {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AffiliateProgramSortOrderProfitability {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateProgramSortOrderProfitability"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAffiliateProgramSortOrder for AffiliateProgramSortOrderProfitability {}

impl AffiliateProgramSortOrderProfitability {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateProgramSortOrderProfitabilityBuilder {
    let inner = AffiliateProgramSortOrderProfitability {
      td_name: "affiliateProgramSortOrderProfitability".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAffiliateProgramSortOrderProfitabilityBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAffiliateProgramSortOrderProfitabilityBuilder {
  inner: AffiliateProgramSortOrderProfitability,
}

impl RTDAffiliateProgramSortOrderProfitabilityBuilder {
  pub fn build(&self) -> AffiliateProgramSortOrderProfitability {
    self.inner.clone()
  }
}

impl AsRef<AffiliateProgramSortOrderProfitability> for AffiliateProgramSortOrderProfitability {
  fn as_ref(&self) -> &AffiliateProgramSortOrderProfitability {
    self
  }
}

impl AsRef<AffiliateProgramSortOrderProfitability>
  for RTDAffiliateProgramSortOrderProfitabilityBuilder
{
  fn as_ref(&self) -> &AffiliateProgramSortOrderProfitability {
    &self.inner
  }
}

/// The affiliate programs must be sorted by the expected revenue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateProgramSortOrderRevenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AffiliateProgramSortOrderRevenue {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateProgramSortOrderRevenue"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAffiliateProgramSortOrder for AffiliateProgramSortOrderRevenue {}

impl AffiliateProgramSortOrderRevenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateProgramSortOrderRevenueBuilder {
    let inner = AffiliateProgramSortOrderRevenue {
      td_name: "affiliateProgramSortOrderRevenue".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAffiliateProgramSortOrderRevenueBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAffiliateProgramSortOrderRevenueBuilder {
  inner: AffiliateProgramSortOrderRevenue,
}

impl RTDAffiliateProgramSortOrderRevenueBuilder {
  pub fn build(&self) -> AffiliateProgramSortOrderRevenue {
    self.inner.clone()
  }
}

impl AsRef<AffiliateProgramSortOrderRevenue> for AffiliateProgramSortOrderRevenue {
  fn as_ref(&self) -> &AffiliateProgramSortOrderRevenue {
    self
  }
}

impl AsRef<AffiliateProgramSortOrderRevenue> for RTDAffiliateProgramSortOrderRevenueBuilder {
  fn as_ref(&self) -> &AffiliateProgramSortOrderRevenue {
    &self.inner
  }
}
