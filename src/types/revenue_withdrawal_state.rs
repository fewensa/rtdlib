use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes state of a revenue withdrawal
pub trait TDRevenueWithdrawalState: Debug + RObject {}

/// Describes state of a revenue withdrawal
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RevenueWithdrawalState {
  #[doc(hidden)]
  _Default(()),
  /// Withdrawal failed
  Failed(RevenueWithdrawalStateFailed),
  /// Withdrawal is pending
  Pending(RevenueWithdrawalStatePending),
  /// Withdrawal succeeded
  Succeeded(RevenueWithdrawalStateSucceeded),
}

impl Default for RevenueWithdrawalState {
  fn default() -> Self {
    RevenueWithdrawalState::_Default(())
  }
}

impl<'de> Deserialize<'de> for RevenueWithdrawalState {
  fn deserialize<D>(deserializer: D) -> Result<RevenueWithdrawalState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      RevenueWithdrawalState,
      (revenueWithdrawalStateFailed, Failed);
      (revenueWithdrawalStatePending, Pending);
      (revenueWithdrawalStateSucceeded, Succeeded);

    )(deserializer)
  }
}

impl RObject for RevenueWithdrawalState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      RevenueWithdrawalState::Failed(t) => t.td_name(),
      RevenueWithdrawalState::Pending(t) => t.td_name(),
      RevenueWithdrawalState::Succeeded(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      RevenueWithdrawalState::Failed(t) => t.extra(),
      RevenueWithdrawalState::Pending(t) => t.extra(),
      RevenueWithdrawalState::Succeeded(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl RevenueWithdrawalState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, RevenueWithdrawalState::_Default(_))
  }

  pub fn is_failed(&self) -> bool {
    matches!(self, RevenueWithdrawalState::Failed(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, RevenueWithdrawalState::Pending(_))
  }
  pub fn is_succeeded(&self) -> bool {
    matches!(self, RevenueWithdrawalState::Succeeded(_))
  }

  pub fn on_failed<F: FnOnce(&RevenueWithdrawalStateFailed)>(&self, fnc: F) -> &Self {
    if let RevenueWithdrawalState::Failed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&RevenueWithdrawalStatePending)>(&self, fnc: F) -> &Self {
    if let RevenueWithdrawalState::Pending(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_succeeded<F: FnOnce(&RevenueWithdrawalStateSucceeded)>(&self, fnc: F) -> &Self {
    if let RevenueWithdrawalState::Succeeded(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_failed(&self) -> Option<&RevenueWithdrawalStateFailed> {
    if let RevenueWithdrawalState::Failed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&RevenueWithdrawalStatePending> {
    if let RevenueWithdrawalState::Pending(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_succeeded(&self) -> Option<&RevenueWithdrawalStateSucceeded> {
    if let RevenueWithdrawalState::Succeeded(t) = self {
      return Some(t);
    }
    None
  }

  pub fn failed<T: AsRef<RevenueWithdrawalStateFailed>>(t: T) -> Self {
    RevenueWithdrawalState::Failed(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<RevenueWithdrawalStatePending>>(t: T) -> Self {
    RevenueWithdrawalState::Pending(t.as_ref().clone())
  }

  pub fn succeeded<T: AsRef<RevenueWithdrawalStateSucceeded>>(t: T) -> Self {
    RevenueWithdrawalState::Succeeded(t.as_ref().clone())
  }
}

impl AsRef<RevenueWithdrawalState> for RevenueWithdrawalState {
  fn as_ref(&self) -> &RevenueWithdrawalState {
    self
  }
}

/// Withdrawal failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevenueWithdrawalStateFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for RevenueWithdrawalStateFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "revenueWithdrawalStateFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRevenueWithdrawalState for RevenueWithdrawalStateFailed {}

impl RevenueWithdrawalStateFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRevenueWithdrawalStateFailedBuilder {
    let inner = RevenueWithdrawalStateFailed {
      td_name: "revenueWithdrawalStateFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDRevenueWithdrawalStateFailedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDRevenueWithdrawalStateFailedBuilder {
  inner: RevenueWithdrawalStateFailed,
}

impl RTDRevenueWithdrawalStateFailedBuilder {
  pub fn build(&self) -> RevenueWithdrawalStateFailed {
    self.inner.clone()
  }
}

impl AsRef<RevenueWithdrawalStateFailed> for RevenueWithdrawalStateFailed {
  fn as_ref(&self) -> &RevenueWithdrawalStateFailed {
    self
  }
}

impl AsRef<RevenueWithdrawalStateFailed> for RTDRevenueWithdrawalStateFailedBuilder {
  fn as_ref(&self) -> &RevenueWithdrawalStateFailed {
    &self.inner
  }
}

/// Withdrawal is pending
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevenueWithdrawalStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for RevenueWithdrawalStatePending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "revenueWithdrawalStatePending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRevenueWithdrawalState for RevenueWithdrawalStatePending {}

impl RevenueWithdrawalStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRevenueWithdrawalStatePendingBuilder {
    let inner = RevenueWithdrawalStatePending {
      td_name: "revenueWithdrawalStatePending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDRevenueWithdrawalStatePendingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDRevenueWithdrawalStatePendingBuilder {
  inner: RevenueWithdrawalStatePending,
}

impl RTDRevenueWithdrawalStatePendingBuilder {
  pub fn build(&self) -> RevenueWithdrawalStatePending {
    self.inner.clone()
  }
}

impl AsRef<RevenueWithdrawalStatePending> for RevenueWithdrawalStatePending {
  fn as_ref(&self) -> &RevenueWithdrawalStatePending {
    self
  }
}

impl AsRef<RevenueWithdrawalStatePending> for RTDRevenueWithdrawalStatePendingBuilder {
  fn as_ref(&self) -> &RevenueWithdrawalStatePending {
    &self.inner
  }
}

/// Withdrawal succeeded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RevenueWithdrawalStateSucceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the withdrawal was completed
  date: i64,
  /// The URL where the withdrawal transaction can be viewed
  url: String,
}

impl RObject for RevenueWithdrawalStateSucceeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "revenueWithdrawalStateSucceeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRevenueWithdrawalState for RevenueWithdrawalStateSucceeded {}

impl RevenueWithdrawalStateSucceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRevenueWithdrawalStateSucceededBuilder {
    let inner = RevenueWithdrawalStateSucceeded {
      td_name: "revenueWithdrawalStateSucceeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRevenueWithdrawalStateSucceededBuilder { inner }
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDRevenueWithdrawalStateSucceededBuilder {
  inner: RevenueWithdrawalStateSucceeded,
}

impl RTDRevenueWithdrawalStateSucceededBuilder {
  pub fn build(&self) -> RevenueWithdrawalStateSucceeded {
    self.inner.clone()
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<RevenueWithdrawalStateSucceeded> for RevenueWithdrawalStateSucceeded {
  fn as_ref(&self) -> &RevenueWithdrawalStateSucceeded {
    self
  }
}

impl AsRef<RevenueWithdrawalStateSucceeded> for RTDRevenueWithdrawalStateSucceededBuilder {
  fn as_ref(&self) -> &RevenueWithdrawalStateSucceeded {
    &self.inner
  }
}
