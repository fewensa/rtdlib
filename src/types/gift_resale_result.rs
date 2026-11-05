use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of sending a resold gift
pub trait TDGiftResaleResult: Debug + RObject {}

/// Describes result of sending a resold gift
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiftResaleResult {
  #[doc(hidden)]
  _Default(()),
  /// Operation was successfully completed
  Ok(GiftResaleResultOk),
  /// Operation has failed, because price has increased. If the price has decreased, then the buying will succeed anyway
  PriceIncreased(GiftResaleResultPriceIncreased),
  /// Sends an upgraded gift that is available for resale to another user or channel chat; gifts already owned by the current user must be transferred using transferGift and can't be passed to the method
  SendResoldGift(SendResoldGift),
}

impl Default for GiftResaleResult {
  fn default() -> Self {
    GiftResaleResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for GiftResaleResult {
  fn deserialize<D>(deserializer: D) -> Result<GiftResaleResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GiftResaleResult,
      (giftResaleResultOk, Ok);
      (giftResaleResultPriceIncreased, PriceIncreased);
      (sendResoldGift, SendResoldGift);

    )(deserializer)
  }
}

impl RObject for GiftResaleResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      GiftResaleResult::Ok(t) => t.td_name(),
      GiftResaleResult::PriceIncreased(t) => t.td_name(),
      GiftResaleResult::SendResoldGift(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      GiftResaleResult::Ok(t) => t.extra(),
      GiftResaleResult::PriceIncreased(t) => t.extra(),
      GiftResaleResult::SendResoldGift(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftResaleResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, GiftResaleResult::_Default(_))
  }

  pub fn is_ok(&self) -> bool {
    matches!(self, GiftResaleResult::Ok(_))
  }
  pub fn is_price_increased(&self) -> bool {
    matches!(self, GiftResaleResult::PriceIncreased(_))
  }
  pub fn is_send_resold_gift(&self) -> bool {
    matches!(self, GiftResaleResult::SendResoldGift(_))
  }

  pub fn on_ok<F: FnOnce(&GiftResaleResultOk)>(&self, fnc: F) -> &Self {
    if let GiftResaleResult::Ok(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_price_increased<F: FnOnce(&GiftResaleResultPriceIncreased)>(&self, fnc: F) -> &Self {
    if let GiftResaleResult::PriceIncreased(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_send_resold_gift<F: FnOnce(&SendResoldGift)>(&self, fnc: F) -> &Self {
    if let GiftResaleResult::SendResoldGift(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_ok(&self) -> Option<&GiftResaleResultOk> {
    if let GiftResaleResult::Ok(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_price_increased(&self) -> Option<&GiftResaleResultPriceIncreased> {
    if let GiftResaleResult::PriceIncreased(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_send_resold_gift(&self) -> Option<&SendResoldGift> {
    if let GiftResaleResult::SendResoldGift(t) = self {
      return Some(t);
    }
    None
  }

  pub fn ok<T: AsRef<GiftResaleResultOk>>(t: T) -> Self {
    GiftResaleResult::Ok(t.as_ref().clone())
  }

  pub fn price_increased<T: AsRef<GiftResaleResultPriceIncreased>>(t: T) -> Self {
    GiftResaleResult::PriceIncreased(t.as_ref().clone())
  }

  pub fn send_resold_gift<T: AsRef<SendResoldGift>>(t: T) -> Self {
    GiftResaleResult::SendResoldGift(t.as_ref().clone())
  }
}

impl AsRef<GiftResaleResult> for GiftResaleResult {
  fn as_ref(&self) -> &GiftResaleResult {
    self
  }
}

/// Operation was successfully completed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftResaleResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the received gift; only for the gifts sent to the current user
  received_gift_id: String,
}

impl RObject for GiftResaleResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftResaleResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftResaleResult for GiftResaleResultOk {}

impl GiftResaleResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftResaleResultOkBuilder {
    let inner = GiftResaleResultOk {
      td_name: "giftResaleResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftResaleResultOkBuilder { inner }
  }

  pub fn received_gift_id(&self) -> &String {
    &self.received_gift_id
  }
}

#[doc(hidden)]
pub struct RTDGiftResaleResultOkBuilder {
  inner: GiftResaleResultOk,
}

impl RTDGiftResaleResultOkBuilder {
  pub fn build(&self) -> GiftResaleResultOk {
    self.inner.clone()
  }

  pub fn received_gift_id<T: AsRef<str>>(&mut self, received_gift_id: T) -> &mut Self {
    self.inner.received_gift_id = received_gift_id.as_ref().to_string();
    self
  }
}

impl AsRef<GiftResaleResultOk> for GiftResaleResultOk {
  fn as_ref(&self) -> &GiftResaleResultOk {
    self
  }
}

impl AsRef<GiftResaleResultOk> for RTDGiftResaleResultOkBuilder {
  fn as_ref(&self) -> &GiftResaleResultOk {
    &self.inner
  }
}

/// Operation has failed, because price has increased. If the price has decreased, then the buying will succeed anyway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftResaleResultPriceIncreased {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New price for the gift
  price: GiftResalePrice,
}

impl RObject for GiftResaleResultPriceIncreased {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftResaleResultPriceIncreased"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftResaleResult for GiftResaleResultPriceIncreased {}

impl GiftResaleResultPriceIncreased {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftResaleResultPriceIncreasedBuilder {
    let inner = GiftResaleResultPriceIncreased {
      td_name: "giftResaleResultPriceIncreased".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftResaleResultPriceIncreasedBuilder { inner }
  }

  pub fn price(&self) -> &GiftResalePrice {
    &self.price
  }
}

#[doc(hidden)]
pub struct RTDGiftResaleResultPriceIncreasedBuilder {
  inner: GiftResaleResultPriceIncreased,
}

impl RTDGiftResaleResultPriceIncreasedBuilder {
  pub fn build(&self) -> GiftResaleResultPriceIncreased {
    self.inner.clone()
  }

  pub fn price<T: AsRef<GiftResalePrice>>(&mut self, price: T) -> &mut Self {
    self.inner.price = price.as_ref().clone();
    self
  }
}

impl AsRef<GiftResaleResultPriceIncreased> for GiftResaleResultPriceIncreased {
  fn as_ref(&self) -> &GiftResaleResultPriceIncreased {
    self
  }
}

impl AsRef<GiftResaleResultPriceIncreased> for RTDGiftResaleResultPriceIncreasedBuilder {
  fn as_ref(&self) -> &GiftResaleResultPriceIncreased {
    &self.inner
  }
}
