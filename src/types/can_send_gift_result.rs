use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes whether a gift can be sent now by the current user
pub trait TDCanSendGiftResult: Debug + RObject {}

/// Describes whether a gift can be sent now by the current user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CanSendGiftResult {
  #[doc(hidden)]
  _Default(()),
  /// Checks whether a gift with next_send_date in the future can be sent already
  CanSendGift(CanSendGift),
  /// The gift can't be sent now by the current user
  Fail(CanSendGiftResultFail),
  /// The gift can be sent now by the current user
  Ok(CanSendGiftResultOk),
}

impl Default for CanSendGiftResult {
  fn default() -> Self {
    CanSendGiftResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for CanSendGiftResult {
  fn deserialize<D>(deserializer: D) -> Result<CanSendGiftResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CanSendGiftResult,
      (canSendGift, CanSendGift);
      (canSendGiftResultFail, Fail);
      (canSendGiftResultOk, Ok);

    )(deserializer)
  }
}

impl RObject for CanSendGiftResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      CanSendGiftResult::CanSendGift(t) => t.td_name(),
      CanSendGiftResult::Fail(t) => t.td_name(),
      CanSendGiftResult::Ok(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      CanSendGiftResult::CanSendGift(t) => t.extra(),
      CanSendGiftResult::Fail(t) => t.extra(),
      CanSendGiftResult::Ok(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CanSendGiftResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, CanSendGiftResult::_Default(_))
  }

  pub fn is_can_send_gift(&self) -> bool {
    matches!(self, CanSendGiftResult::CanSendGift(_))
  }
  pub fn is_fail(&self) -> bool {
    matches!(self, CanSendGiftResult::Fail(_))
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, CanSendGiftResult::Ok(_))
  }

  pub fn on_can_send_gift<F: FnOnce(&CanSendGift)>(&self, fnc: F) -> &Self {
    if let CanSendGiftResult::CanSendGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fail<F: FnOnce(&CanSendGiftResultFail)>(&self, fnc: F) -> &Self {
    if let CanSendGiftResult::Fail(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ok<F: FnOnce(&CanSendGiftResultOk)>(&self, fnc: F) -> &Self {
    if let CanSendGiftResult::Ok(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_can_send_gift(&self) -> Option<&CanSendGift> {
    if let CanSendGiftResult::CanSendGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fail(&self) -> Option<&CanSendGiftResultFail> {
    if let CanSendGiftResult::Fail(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ok(&self) -> Option<&CanSendGiftResultOk> {
    if let CanSendGiftResult::Ok(t) = self {
      return Some(t);
    }
    None
  }

  pub fn can_send_gift<T: AsRef<CanSendGift>>(t: T) -> Self {
    CanSendGiftResult::CanSendGift(t.as_ref().clone())
  }

  pub fn fail<T: AsRef<CanSendGiftResultFail>>(t: T) -> Self {
    CanSendGiftResult::Fail(t.as_ref().clone())
  }

  pub fn ok<T: AsRef<CanSendGiftResultOk>>(t: T) -> Self {
    CanSendGiftResult::Ok(t.as_ref().clone())
  }
}

impl AsRef<CanSendGiftResult> for CanSendGiftResult {
  fn as_ref(&self) -> &CanSendGiftResult {
    self
  }
}

/// The gift can't be sent now by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendGiftResultFail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Reason to be shown to the user
  reason: FormattedText,
}

impl RObject for CanSendGiftResultFail {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canSendGiftResultFail"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanSendGiftResult for CanSendGiftResultFail {}

impl CanSendGiftResultFail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanSendGiftResultFailBuilder {
    let inner = CanSendGiftResultFail {
      td_name: "canSendGiftResultFail".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCanSendGiftResultFailBuilder { inner }
  }

  pub fn reason(&self) -> &FormattedText {
    &self.reason
  }
}

#[doc(hidden)]
pub struct RTDCanSendGiftResultFailBuilder {
  inner: CanSendGiftResultFail,
}

impl RTDCanSendGiftResultFailBuilder {
  pub fn build(&self) -> CanSendGiftResultFail {
    self.inner.clone()
  }

  pub fn reason<T: AsRef<FormattedText>>(&mut self, reason: T) -> &mut Self {
    self.inner.reason = reason.as_ref().clone();
    self
  }
}

impl AsRef<CanSendGiftResultFail> for CanSendGiftResultFail {
  fn as_ref(&self) -> &CanSendGiftResultFail {
    self
  }
}

impl AsRef<CanSendGiftResultFail> for RTDCanSendGiftResultFailBuilder {
  fn as_ref(&self) -> &CanSendGiftResultFail {
    &self.inner
  }
}

/// The gift can be sent now by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendGiftResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CanSendGiftResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canSendGiftResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanSendGiftResult for CanSendGiftResultOk {}

impl CanSendGiftResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanSendGiftResultOkBuilder {
    let inner = CanSendGiftResultOk {
      td_name: "canSendGiftResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCanSendGiftResultOkBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCanSendGiftResultOkBuilder {
  inner: CanSendGiftResultOk,
}

impl RTDCanSendGiftResultOkBuilder {
  pub fn build(&self) -> CanSendGiftResultOk {
    self.inner.clone()
  }
}

impl AsRef<CanSendGiftResultOk> for CanSendGiftResultOk {
  fn as_ref(&self) -> &CanSendGiftResultOk {
    self
  }
}

impl AsRef<CanSendGiftResultOk> for RTDCanSendGiftResultOkBuilder {
  fn as_ref(&self) -> &CanSendGiftResultOk {
    &self.inner
  }
}
