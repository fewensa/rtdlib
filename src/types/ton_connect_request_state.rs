use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes state of a request received from a dApp through TON Connect
pub trait TDTonConnectRequestState: Debug + RObject {}

/// Describes state of a request received from a dApp through TON Connect
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonConnectRequestState {
  #[doc(hidden)]
  _Default(()),
  /// The request was accepted
  Accepted(TonConnectRequestStateAccepted),
  /// The request must be accepted or rejected if it isn't expired yet
  Pending(TonConnectRequestStatePending),
  /// The request was rejected
  Rejected(TonConnectRequestStateRejected),
}

impl Default for TonConnectRequestState {
  fn default() -> Self {
    TonConnectRequestState::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonConnectRequestState {
  fn deserialize<D>(deserializer: D) -> Result<TonConnectRequestState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonConnectRequestState,
      (tonConnectRequestStateAccepted, Accepted);
      (tonConnectRequestStatePending, Pending);
      (tonConnectRequestStateRejected, Rejected);

    )(deserializer)
  }
}

impl RObject for TonConnectRequestState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonConnectRequestState::Accepted(t) => t.td_name(),
      TonConnectRequestState::Pending(t) => t.td_name(),
      TonConnectRequestState::Rejected(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonConnectRequestState::Accepted(t) => t.extra(),
      TonConnectRequestState::Pending(t) => t.extra(),
      TonConnectRequestState::Rejected(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectRequestState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonConnectRequestState::_Default(_))
  }

  pub fn is_accepted(&self) -> bool {
    matches!(self, TonConnectRequestState::Accepted(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, TonConnectRequestState::Pending(_))
  }
  pub fn is_rejected(&self) -> bool {
    matches!(self, TonConnectRequestState::Rejected(_))
  }

  pub fn on_accepted<F: FnOnce(&TonConnectRequestStateAccepted)>(&self, fnc: F) -> &Self {
    if let TonConnectRequestState::Accepted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&TonConnectRequestStatePending)>(&self, fnc: F) -> &Self {
    if let TonConnectRequestState::Pending(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_rejected<F: FnOnce(&TonConnectRequestStateRejected)>(&self, fnc: F) -> &Self {
    if let TonConnectRequestState::Rejected(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_accepted(&self) -> Option<&TonConnectRequestStateAccepted> {
    if let TonConnectRequestState::Accepted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&TonConnectRequestStatePending> {
    if let TonConnectRequestState::Pending(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_rejected(&self) -> Option<&TonConnectRequestStateRejected> {
    if let TonConnectRequestState::Rejected(t) = self {
      return Some(t);
    }
    None
  }

  pub fn accepted<T: AsRef<TonConnectRequestStateAccepted>>(t: T) -> Self {
    TonConnectRequestState::Accepted(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<TonConnectRequestStatePending>>(t: T) -> Self {
    TonConnectRequestState::Pending(t.as_ref().clone())
  }

  pub fn rejected<T: AsRef<TonConnectRequestStateRejected>>(t: T) -> Self {
    TonConnectRequestState::Rejected(t.as_ref().clone())
  }
}

impl AsRef<TonConnectRequestState> for TonConnectRequestState {
  fn as_ref(&self) -> &TonConnectRequestState {
    self
  }
}

/// The request was accepted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectRequestStateAccepted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectRequestStateAccepted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectRequestStateAccepted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectRequestState for TonConnectRequestStateAccepted {}

impl TonConnectRequestStateAccepted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectRequestStateAcceptedBuilder {
    let inner = TonConnectRequestStateAccepted {
      td_name: "tonConnectRequestStateAccepted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectRequestStateAcceptedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectRequestStateAcceptedBuilder {
  inner: TonConnectRequestStateAccepted,
}

impl RTDTonConnectRequestStateAcceptedBuilder {
  pub fn build(&self) -> TonConnectRequestStateAccepted {
    self.inner.clone()
  }
}

impl AsRef<TonConnectRequestStateAccepted> for TonConnectRequestStateAccepted {
  fn as_ref(&self) -> &TonConnectRequestStateAccepted {
    self
  }
}

impl AsRef<TonConnectRequestStateAccepted> for RTDTonConnectRequestStateAcceptedBuilder {
  fn as_ref(&self) -> &TonConnectRequestStateAccepted {
    &self.inner
  }
}

/// The request must be accepted or rejected if it isn't expired yet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectRequestStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the request will expire or has expired
  expiration_date: i64,
}

impl RObject for TonConnectRequestStatePending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectRequestStatePending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectRequestState for TonConnectRequestStatePending {}

impl TonConnectRequestStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectRequestStatePendingBuilder {
    let inner = TonConnectRequestStatePending {
      td_name: "tonConnectRequestStatePending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectRequestStatePendingBuilder { inner }
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }
}

#[doc(hidden)]
pub struct RTDTonConnectRequestStatePendingBuilder {
  inner: TonConnectRequestStatePending,
}

impl RTDTonConnectRequestStatePendingBuilder {
  pub fn build(&self) -> TonConnectRequestStatePending {
    self.inner.clone()
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }
}

impl AsRef<TonConnectRequestStatePending> for TonConnectRequestStatePending {
  fn as_ref(&self) -> &TonConnectRequestStatePending {
    self
  }
}

impl AsRef<TonConnectRequestStatePending> for RTDTonConnectRequestStatePendingBuilder {
  fn as_ref(&self) -> &TonConnectRequestStatePending {
    &self.inner
  }
}

/// The request was rejected
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectRequestStateRejected {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectRequestStateRejected {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectRequestStateRejected"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectRequestState for TonConnectRequestStateRejected {}

impl TonConnectRequestStateRejected {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectRequestStateRejectedBuilder {
    let inner = TonConnectRequestStateRejected {
      td_name: "tonConnectRequestStateRejected".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectRequestStateRejectedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectRequestStateRejectedBuilder {
  inner: TonConnectRequestStateRejected,
}

impl RTDTonConnectRequestStateRejectedBuilder {
  pub fn build(&self) -> TonConnectRequestStateRejected {
    self.inner.clone()
  }
}

impl AsRef<TonConnectRequestStateRejected> for TonConnectRequestStateRejected {
  fn as_ref(&self) -> &TonConnectRequestStateRejected {
    self
  }
}

impl AsRef<TonConnectRequestStateRejected> for RTDTonConnectRequestStateRejectedBuilder {
  fn as_ref(&self) -> &TonConnectRequestStateRejected {
    &self.inner
  }
}
