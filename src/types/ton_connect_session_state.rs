use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes state of a TON Connect session
pub trait TDTonConnectSessionState: Debug + RObject {}

/// Describes state of a TON Connect session
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonConnectSessionState {
  #[doc(hidden)]
  _Default(()),
  /// The session is closed
  Closed(TonConnectSessionStateClosed),
  /// The session is being closed
  Closing(TonConnectSessionStateClosing),
  /// The session is pending
  Pending(TonConnectSessionStatePending),
  /// The session is ready
  Ready(TonConnectSessionStateReady),
}

impl Default for TonConnectSessionState {
  fn default() -> Self {
    TonConnectSessionState::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonConnectSessionState {
  fn deserialize<D>(deserializer: D) -> Result<TonConnectSessionState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonConnectSessionState,
      (tonConnectSessionStateClosed, Closed);
      (tonConnectSessionStateClosing, Closing);
      (tonConnectSessionStatePending, Pending);
      (tonConnectSessionStateReady, Ready);

    )(deserializer)
  }
}

impl RObject for TonConnectSessionState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonConnectSessionState::Closed(t) => t.td_name(),
      TonConnectSessionState::Closing(t) => t.td_name(),
      TonConnectSessionState::Pending(t) => t.td_name(),
      TonConnectSessionState::Ready(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonConnectSessionState::Closed(t) => t.extra(),
      TonConnectSessionState::Closing(t) => t.extra(),
      TonConnectSessionState::Pending(t) => t.extra(),
      TonConnectSessionState::Ready(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectSessionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonConnectSessionState::_Default(_))
  }

  pub fn is_closed(&self) -> bool {
    matches!(self, TonConnectSessionState::Closed(_))
  }
  pub fn is_closing(&self) -> bool {
    matches!(self, TonConnectSessionState::Closing(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, TonConnectSessionState::Pending(_))
  }
  pub fn is_ready(&self) -> bool {
    matches!(self, TonConnectSessionState::Ready(_))
  }

  pub fn on_closed<F: FnOnce(&TonConnectSessionStateClosed)>(&self, fnc: F) -> &Self {
    if let TonConnectSessionState::Closed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_closing<F: FnOnce(&TonConnectSessionStateClosing)>(&self, fnc: F) -> &Self {
    if let TonConnectSessionState::Closing(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&TonConnectSessionStatePending)>(&self, fnc: F) -> &Self {
    if let TonConnectSessionState::Pending(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ready<F: FnOnce(&TonConnectSessionStateReady)>(&self, fnc: F) -> &Self {
    if let TonConnectSessionState::Ready(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_closed(&self) -> Option<&TonConnectSessionStateClosed> {
    if let TonConnectSessionState::Closed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_closing(&self) -> Option<&TonConnectSessionStateClosing> {
    if let TonConnectSessionState::Closing(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&TonConnectSessionStatePending> {
    if let TonConnectSessionState::Pending(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ready(&self) -> Option<&TonConnectSessionStateReady> {
    if let TonConnectSessionState::Ready(t) = self {
      return Some(t);
    }
    None
  }

  pub fn closed<T: AsRef<TonConnectSessionStateClosed>>(t: T) -> Self {
    TonConnectSessionState::Closed(t.as_ref().clone())
  }

  pub fn closing<T: AsRef<TonConnectSessionStateClosing>>(t: T) -> Self {
    TonConnectSessionState::Closing(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<TonConnectSessionStatePending>>(t: T) -> Self {
    TonConnectSessionState::Pending(t.as_ref().clone())
  }

  pub fn ready<T: AsRef<TonConnectSessionStateReady>>(t: T) -> Self {
    TonConnectSessionState::Ready(t.as_ref().clone())
  }
}

impl AsRef<TonConnectSessionState> for TonConnectSessionState {
  fn as_ref(&self) -> &TonConnectSessionState {
    self
  }
}

/// The session is closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectSessionStateClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectSessionStateClosed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectSessionStateClosed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectSessionState for TonConnectSessionStateClosed {}

impl TonConnectSessionStateClosed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectSessionStateClosedBuilder {
    let inner = TonConnectSessionStateClosed {
      td_name: "tonConnectSessionStateClosed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectSessionStateClosedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectSessionStateClosedBuilder {
  inner: TonConnectSessionStateClosed,
}

impl RTDTonConnectSessionStateClosedBuilder {
  pub fn build(&self) -> TonConnectSessionStateClosed {
    self.inner.clone()
  }
}

impl AsRef<TonConnectSessionStateClosed> for TonConnectSessionStateClosed {
  fn as_ref(&self) -> &TonConnectSessionStateClosed {
    self
  }
}

impl AsRef<TonConnectSessionStateClosed> for RTDTonConnectSessionStateClosedBuilder {
  fn as_ref(&self) -> &TonConnectSessionStateClosed {
    &self.inner
  }
}

/// The session is being closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectSessionStateClosing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectSessionStateClosing {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectSessionStateClosing"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectSessionState for TonConnectSessionStateClosing {}

impl TonConnectSessionStateClosing {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectSessionStateClosingBuilder {
    let inner = TonConnectSessionStateClosing {
      td_name: "tonConnectSessionStateClosing".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectSessionStateClosingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectSessionStateClosingBuilder {
  inner: TonConnectSessionStateClosing,
}

impl RTDTonConnectSessionStateClosingBuilder {
  pub fn build(&self) -> TonConnectSessionStateClosing {
    self.inner.clone()
  }
}

impl AsRef<TonConnectSessionStateClosing> for TonConnectSessionStateClosing {
  fn as_ref(&self) -> &TonConnectSessionStateClosing {
    self
  }
}

impl AsRef<TonConnectSessionStateClosing> for RTDTonConnectSessionStateClosingBuilder {
  fn as_ref(&self) -> &TonConnectSessionStateClosing {
    &self.inner
  }
}

/// The session is pending
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectSessionStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectSessionStatePending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectSessionStatePending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectSessionState for TonConnectSessionStatePending {}

impl TonConnectSessionStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectSessionStatePendingBuilder {
    let inner = TonConnectSessionStatePending {
      td_name: "tonConnectSessionStatePending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectSessionStatePendingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectSessionStatePendingBuilder {
  inner: TonConnectSessionStatePending,
}

impl RTDTonConnectSessionStatePendingBuilder {
  pub fn build(&self) -> TonConnectSessionStatePending {
    self.inner.clone()
  }
}

impl AsRef<TonConnectSessionStatePending> for TonConnectSessionStatePending {
  fn as_ref(&self) -> &TonConnectSessionStatePending {
    self
  }
}

impl AsRef<TonConnectSessionStatePending> for RTDTonConnectSessionStatePendingBuilder {
  fn as_ref(&self) -> &TonConnectSessionStatePending {
    &self.inner
  }
}

/// The session is ready
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectSessionStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonConnectSessionStateReady {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectSessionStateReady"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonConnectSessionState for TonConnectSessionStateReady {}

impl TonConnectSessionStateReady {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectSessionStateReadyBuilder {
    let inner = TonConnectSessionStateReady {
      td_name: "tonConnectSessionStateReady".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonConnectSessionStateReadyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonConnectSessionStateReadyBuilder {
  inner: TonConnectSessionStateReady,
}

impl RTDTonConnectSessionStateReadyBuilder {
  pub fn build(&self) -> TonConnectSessionStateReady {
    self.inner.clone()
  }
}

impl AsRef<TonConnectSessionStateReady> for TonConnectSessionStateReady {
  fn as_ref(&self) -> &TonConnectSessionStateReady {
    self
  }
}

impl AsRef<TonConnectSessionStateReady> for RTDTonConnectSessionStateReadyBuilder {
  fn as_ref(&self) -> &TonConnectSessionStateReady {
    &self.inner
  }
}
