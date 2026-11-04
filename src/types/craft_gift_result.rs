use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains result of gift crafting
pub trait TDCraftGiftResult: Debug + RObject {}

/// Contains result of gift crafting
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CraftGiftResult {
  #[doc(hidden)]
  _Default(()),
  /// Crafts a new gift from other gifts that will be permanently lost
  CraftGift(CraftGift),
  /// Crafting has failed
  Fail(CraftGiftResultFail),
  /// Crafting isn't possible because one of the gifts isn't suitable for crafting
  InvalidGift(CraftGiftResultInvalidGift),
  /// Crafting was successful
  Success(CraftGiftResultSuccess),
  /// Crafting isn't possible because one of the gifts can't be used for crafting yet
  TooEarly(CraftGiftResultTooEarly),
}

impl Default for CraftGiftResult {
  fn default() -> Self {
    CraftGiftResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for CraftGiftResult {
  fn deserialize<D>(deserializer: D) -> Result<CraftGiftResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CraftGiftResult,
      (craftGift, CraftGift);
      (craftGiftResultFail, Fail);
      (craftGiftResultInvalidGift, InvalidGift);
      (craftGiftResultSuccess, Success);
      (craftGiftResultTooEarly, TooEarly);

    )(deserializer)
  }
}

impl RObject for CraftGiftResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      CraftGiftResult::CraftGift(t) => t.td_name(),
      CraftGiftResult::Fail(t) => t.td_name(),
      CraftGiftResult::InvalidGift(t) => t.td_name(),
      CraftGiftResult::Success(t) => t.td_name(),
      CraftGiftResult::TooEarly(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      CraftGiftResult::CraftGift(t) => t.extra(),
      CraftGiftResult::Fail(t) => t.extra(),
      CraftGiftResult::InvalidGift(t) => t.extra(),
      CraftGiftResult::Success(t) => t.extra(),
      CraftGiftResult::TooEarly(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CraftGiftResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, CraftGiftResult::_Default(_))
  }

  pub fn is_craft_gift(&self) -> bool {
    matches!(self, CraftGiftResult::CraftGift(_))
  }
  pub fn is_fail(&self) -> bool {
    matches!(self, CraftGiftResult::Fail(_))
  }
  pub fn is_invalid_gift(&self) -> bool {
    matches!(self, CraftGiftResult::InvalidGift(_))
  }
  pub fn is_success(&self) -> bool {
    matches!(self, CraftGiftResult::Success(_))
  }
  pub fn is_too_early(&self) -> bool {
    matches!(self, CraftGiftResult::TooEarly(_))
  }

  pub fn on_craft_gift<F: FnOnce(&CraftGift)>(&self, fnc: F) -> &Self {
    if let CraftGiftResult::CraftGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fail<F: FnOnce(&CraftGiftResultFail)>(&self, fnc: F) -> &Self {
    if let CraftGiftResult::Fail(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_invalid_gift<F: FnOnce(&CraftGiftResultInvalidGift)>(&self, fnc: F) -> &Self {
    if let CraftGiftResult::InvalidGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_success<F: FnOnce(&CraftGiftResultSuccess)>(&self, fnc: F) -> &Self {
    if let CraftGiftResult::Success(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_too_early<F: FnOnce(&CraftGiftResultTooEarly)>(&self, fnc: F) -> &Self {
    if let CraftGiftResult::TooEarly(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_craft_gift(&self) -> Option<&CraftGift> {
    if let CraftGiftResult::CraftGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fail(&self) -> Option<&CraftGiftResultFail> {
    if let CraftGiftResult::Fail(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_invalid_gift(&self) -> Option<&CraftGiftResultInvalidGift> {
    if let CraftGiftResult::InvalidGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_success(&self) -> Option<&CraftGiftResultSuccess> {
    if let CraftGiftResult::Success(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_too_early(&self) -> Option<&CraftGiftResultTooEarly> {
    if let CraftGiftResult::TooEarly(t) = self {
      return Some(t);
    }
    None
  }

  pub fn craft_gift<T: AsRef<CraftGift>>(t: T) -> Self {
    CraftGiftResult::CraftGift(t.as_ref().clone())
  }

  pub fn fail<T: AsRef<CraftGiftResultFail>>(t: T) -> Self {
    CraftGiftResult::Fail(t.as_ref().clone())
  }

  pub fn invalid_gift<T: AsRef<CraftGiftResultInvalidGift>>(t: T) -> Self {
    CraftGiftResult::InvalidGift(t.as_ref().clone())
  }

  pub fn success<T: AsRef<CraftGiftResultSuccess>>(t: T) -> Self {
    CraftGiftResult::Success(t.as_ref().clone())
  }

  pub fn too_early<T: AsRef<CraftGiftResultTooEarly>>(t: T) -> Self {
    CraftGiftResult::TooEarly(t.as_ref().clone())
  }
}

impl AsRef<CraftGiftResult> for CraftGiftResult {
  fn as_ref(&self) -> &CraftGiftResult {
    self
  }
}

/// Crafting has failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CraftGiftResultFail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CraftGiftResultFail {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "craftGiftResultFail"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCraftGiftResult for CraftGiftResultFail {}

impl CraftGiftResultFail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCraftGiftResultFailBuilder {
    let inner = CraftGiftResultFail {
      td_name: "craftGiftResultFail".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCraftGiftResultFailBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCraftGiftResultFailBuilder {
  inner: CraftGiftResultFail,
}

impl RTDCraftGiftResultFailBuilder {
  pub fn build(&self) -> CraftGiftResultFail {
    self.inner.clone()
  }
}

impl AsRef<CraftGiftResultFail> for CraftGiftResultFail {
  fn as_ref(&self) -> &CraftGiftResultFail {
    self
  }
}

impl AsRef<CraftGiftResultFail> for RTDCraftGiftResultFailBuilder {
  fn as_ref(&self) -> &CraftGiftResultFail {
    &self.inner
  }
}

/// Crafting isn't possible because one of the gifts isn't suitable for crafting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CraftGiftResultInvalidGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CraftGiftResultInvalidGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "craftGiftResultInvalidGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCraftGiftResult for CraftGiftResultInvalidGift {}

impl CraftGiftResultInvalidGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCraftGiftResultInvalidGiftBuilder {
    let inner = CraftGiftResultInvalidGift {
      td_name: "craftGiftResultInvalidGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCraftGiftResultInvalidGiftBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCraftGiftResultInvalidGiftBuilder {
  inner: CraftGiftResultInvalidGift,
}

impl RTDCraftGiftResultInvalidGiftBuilder {
  pub fn build(&self) -> CraftGiftResultInvalidGift {
    self.inner.clone()
  }
}

impl AsRef<CraftGiftResultInvalidGift> for CraftGiftResultInvalidGift {
  fn as_ref(&self) -> &CraftGiftResultInvalidGift {
    self
  }
}

impl AsRef<CraftGiftResultInvalidGift> for RTDCraftGiftResultInvalidGiftBuilder {
  fn as_ref(&self) -> &CraftGiftResultInvalidGift {
    &self.inner
  }
}

/// Crafting was successful
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CraftGiftResultSuccess {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The created gift
  gift: UpgradedGift,
  /// Unique identifier of the received gift for the current user
  received_gift_id: String,
}

impl RObject for CraftGiftResultSuccess {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "craftGiftResultSuccess"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCraftGiftResult for CraftGiftResultSuccess {}

impl CraftGiftResultSuccess {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCraftGiftResultSuccessBuilder {
    let inner = CraftGiftResultSuccess {
      td_name: "craftGiftResultSuccess".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCraftGiftResultSuccessBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }

  pub fn received_gift_id(&self) -> &String {
    &self.received_gift_id
  }
}

#[doc(hidden)]
pub struct RTDCraftGiftResultSuccessBuilder {
  inner: CraftGiftResultSuccess,
}

impl RTDCraftGiftResultSuccessBuilder {
  pub fn build(&self) -> CraftGiftResultSuccess {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn received_gift_id<T: AsRef<str>>(&mut self, received_gift_id: T) -> &mut Self {
    self.inner.received_gift_id = received_gift_id.as_ref().to_string();
    self
  }
}

impl AsRef<CraftGiftResultSuccess> for CraftGiftResultSuccess {
  fn as_ref(&self) -> &CraftGiftResultSuccess {
    self
  }
}

impl AsRef<CraftGiftResultSuccess> for RTDCraftGiftResultSuccessBuilder {
  fn as_ref(&self) -> &CraftGiftResultSuccess {
    &self.inner
  }
}

/// Crafting isn't possible because one of the gifts can't be used for crafting yet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CraftGiftResultTooEarly {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Time left before the gift can be used for crafting
  retry_after: i64,
}

impl RObject for CraftGiftResultTooEarly {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "craftGiftResultTooEarly"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCraftGiftResult for CraftGiftResultTooEarly {}

impl CraftGiftResultTooEarly {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCraftGiftResultTooEarlyBuilder {
    let inner = CraftGiftResultTooEarly {
      td_name: "craftGiftResultTooEarly".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCraftGiftResultTooEarlyBuilder { inner }
  }

  pub fn retry_after(&self) -> i64 {
    self.retry_after
  }
}

#[doc(hidden)]
pub struct RTDCraftGiftResultTooEarlyBuilder {
  inner: CraftGiftResultTooEarly,
}

impl RTDCraftGiftResultTooEarlyBuilder {
  pub fn build(&self) -> CraftGiftResultTooEarly {
    self.inner.clone()
  }

  pub fn retry_after(&mut self, retry_after: i64) -> &mut Self {
    self.inner.retry_after = retry_after;
    self
  }
}

impl AsRef<CraftGiftResultTooEarly> for CraftGiftResultTooEarly {
  fn as_ref(&self) -> &CraftGiftResultTooEarly {
    self
  }
}

impl AsRef<CraftGiftResultTooEarly> for RTDCraftGiftResultTooEarlyBuilder {
  fn as_ref(&self) -> &CraftGiftResultTooEarly {
    &self.inner
  }
}
