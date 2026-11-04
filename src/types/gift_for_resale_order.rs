use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes order in which upgraded gifts for resale will be sorted
pub trait TDGiftForResaleOrder: Debug + RObject {}

/// Describes order in which upgraded gifts for resale will be sorted
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiftForResaleOrder {
  #[doc(hidden)]
  _Default(()),
  /// The gifts will be sorted by their number from the smallest to the largest
  Number(GiftForResaleOrderNumber),
  /// The gifts will be sorted by their price from the lowest to the highest
  Price(GiftForResaleOrderPrice),
  /// The gifts will be sorted by the last date when their price was changed from the newest to the oldest
  PriceChangeDate(GiftForResaleOrderPriceChangeDate),
}

impl Default for GiftForResaleOrder {
  fn default() -> Self {
    GiftForResaleOrder::_Default(())
  }
}

impl<'de> Deserialize<'de> for GiftForResaleOrder {
  fn deserialize<D>(deserializer: D) -> Result<GiftForResaleOrder, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GiftForResaleOrder,
      (giftForResaleOrderNumber, Number);
      (giftForResaleOrderPrice, Price);
      (giftForResaleOrderPriceChangeDate, PriceChangeDate);

    )(deserializer)
  }
}

impl RObject for GiftForResaleOrder {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      GiftForResaleOrder::Number(t) => t.td_name(),
      GiftForResaleOrder::Price(t) => t.td_name(),
      GiftForResaleOrder::PriceChangeDate(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      GiftForResaleOrder::Number(t) => t.extra(),
      GiftForResaleOrder::Price(t) => t.extra(),
      GiftForResaleOrder::PriceChangeDate(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftForResaleOrder {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, GiftForResaleOrder::_Default(_))
  }

  pub fn is_number(&self) -> bool {
    matches!(self, GiftForResaleOrder::Number(_))
  }
  pub fn is_price(&self) -> bool {
    matches!(self, GiftForResaleOrder::Price(_))
  }
  pub fn is_price_change_date(&self) -> bool {
    matches!(self, GiftForResaleOrder::PriceChangeDate(_))
  }

  pub fn on_number<F: FnOnce(&GiftForResaleOrderNumber)>(&self, fnc: F) -> &Self {
    if let GiftForResaleOrder::Number(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_price<F: FnOnce(&GiftForResaleOrderPrice)>(&self, fnc: F) -> &Self {
    if let GiftForResaleOrder::Price(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_price_change_date<F: FnOnce(&GiftForResaleOrderPriceChangeDate)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let GiftForResaleOrder::PriceChangeDate(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_number(&self) -> Option<&GiftForResaleOrderNumber> {
    if let GiftForResaleOrder::Number(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_price(&self) -> Option<&GiftForResaleOrderPrice> {
    if let GiftForResaleOrder::Price(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_price_change_date(&self) -> Option<&GiftForResaleOrderPriceChangeDate> {
    if let GiftForResaleOrder::PriceChangeDate(t) = self {
      return Some(t);
    }
    None
  }

  pub fn number<T: AsRef<GiftForResaleOrderNumber>>(t: T) -> Self {
    GiftForResaleOrder::Number(t.as_ref().clone())
  }

  pub fn price<T: AsRef<GiftForResaleOrderPrice>>(t: T) -> Self {
    GiftForResaleOrder::Price(t.as_ref().clone())
  }

  pub fn price_change_date<T: AsRef<GiftForResaleOrderPriceChangeDate>>(t: T) -> Self {
    GiftForResaleOrder::PriceChangeDate(t.as_ref().clone())
  }
}

impl AsRef<GiftForResaleOrder> for GiftForResaleOrder {
  fn as_ref(&self) -> &GiftForResaleOrder {
    self
  }
}

/// The gifts will be sorted by their number from the smallest to the largest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftForResaleOrderNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GiftForResaleOrderNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftForResaleOrderNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftForResaleOrder for GiftForResaleOrderNumber {}

impl GiftForResaleOrderNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftForResaleOrderNumberBuilder {
    let inner = GiftForResaleOrderNumber {
      td_name: "giftForResaleOrderNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGiftForResaleOrderNumberBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGiftForResaleOrderNumberBuilder {
  inner: GiftForResaleOrderNumber,
}

impl RTDGiftForResaleOrderNumberBuilder {
  pub fn build(&self) -> GiftForResaleOrderNumber {
    self.inner.clone()
  }
}

impl AsRef<GiftForResaleOrderNumber> for GiftForResaleOrderNumber {
  fn as_ref(&self) -> &GiftForResaleOrderNumber {
    self
  }
}

impl AsRef<GiftForResaleOrderNumber> for RTDGiftForResaleOrderNumberBuilder {
  fn as_ref(&self) -> &GiftForResaleOrderNumber {
    &self.inner
  }
}

/// The gifts will be sorted by their price from the lowest to the highest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftForResaleOrderPrice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GiftForResaleOrderPrice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftForResaleOrderPrice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftForResaleOrder for GiftForResaleOrderPrice {}

impl GiftForResaleOrderPrice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftForResaleOrderPriceBuilder {
    let inner = GiftForResaleOrderPrice {
      td_name: "giftForResaleOrderPrice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGiftForResaleOrderPriceBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGiftForResaleOrderPriceBuilder {
  inner: GiftForResaleOrderPrice,
}

impl RTDGiftForResaleOrderPriceBuilder {
  pub fn build(&self) -> GiftForResaleOrderPrice {
    self.inner.clone()
  }
}

impl AsRef<GiftForResaleOrderPrice> for GiftForResaleOrderPrice {
  fn as_ref(&self) -> &GiftForResaleOrderPrice {
    self
  }
}

impl AsRef<GiftForResaleOrderPrice> for RTDGiftForResaleOrderPriceBuilder {
  fn as_ref(&self) -> &GiftForResaleOrderPrice {
    &self.inner
  }
}

/// The gifts will be sorted by the last date when their price was changed from the newest to the oldest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftForResaleOrderPriceChangeDate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GiftForResaleOrderPriceChangeDate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftForResaleOrderPriceChangeDate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftForResaleOrder for GiftForResaleOrderPriceChangeDate {}

impl GiftForResaleOrderPriceChangeDate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftForResaleOrderPriceChangeDateBuilder {
    let inner = GiftForResaleOrderPriceChangeDate {
      td_name: "giftForResaleOrderPriceChangeDate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGiftForResaleOrderPriceChangeDateBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGiftForResaleOrderPriceChangeDateBuilder {
  inner: GiftForResaleOrderPriceChangeDate,
}

impl RTDGiftForResaleOrderPriceChangeDateBuilder {
  pub fn build(&self) -> GiftForResaleOrderPriceChangeDate {
    self.inner.clone()
  }
}

impl AsRef<GiftForResaleOrderPriceChangeDate> for GiftForResaleOrderPriceChangeDate {
  fn as_ref(&self) -> &GiftForResaleOrderPriceChangeDate {
    self
  }
}

impl AsRef<GiftForResaleOrderPriceChangeDate> for RTDGiftForResaleOrderPriceChangeDateBuilder {
  fn as_ref(&self) -> &GiftForResaleOrderPriceChangeDate {
    &self.inner
  }
}
