use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the current call state
pub trait TDCallState: Debug + RObject {}

/// Describes the current call state
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallState {
  #[doc(hidden)]
  _Default(()),
  /// The call has ended successfully
  Discarded(CallStateDiscarded),
  /// The call has ended with an error
  Error(CallStateError),
  /// The call has been answered and encryption keys are being exchanged
  ExchangingKeys(CallStateExchangingKeys),
  /// The call is hanging up after discardCall has been called
  HangingUp(CallStateHangingUp),
  /// The call is pending, waiting to be accepted by a user
  Pending(CallStatePending),
  /// The call is ready to use
  Ready(CallStateReady),
}

impl Default for CallState {
  fn default() -> Self {
    CallState::_Default(())
  }
}

impl<'de> Deserialize<'de> for CallState {
  fn deserialize<D>(deserializer: D) -> Result<CallState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CallState,
      (callStateDiscarded, Discarded);
      (callStateError, Error);
      (callStateExchangingKeys, ExchangingKeys);
      (callStateHangingUp, HangingUp);
      (callStatePending, Pending);
      (callStateReady, Ready);

    )(deserializer)
  }
}

impl RObject for CallState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      CallState::Discarded(t) => t.td_name(),
      CallState::Error(t) => t.td_name(),
      CallState::ExchangingKeys(t) => t.td_name(),
      CallState::HangingUp(t) => t.td_name(),
      CallState::Pending(t) => t.td_name(),
      CallState::Ready(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      CallState::Discarded(t) => t.extra(),
      CallState::Error(t) => t.extra(),
      CallState::ExchangingKeys(t) => t.extra(),
      CallState::HangingUp(t) => t.extra(),
      CallState::Pending(t) => t.extra(),
      CallState::Ready(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CallState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, CallState::_Default(_))
  }

  pub fn is_discarded(&self) -> bool {
    matches!(self, CallState::Discarded(_))
  }
  pub fn is_error(&self) -> bool {
    matches!(self, CallState::Error(_))
  }
  pub fn is_exchanging_keys(&self) -> bool {
    matches!(self, CallState::ExchangingKeys(_))
  }
  pub fn is_hanging_up(&self) -> bool {
    matches!(self, CallState::HangingUp(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, CallState::Pending(_))
  }
  pub fn is_ready(&self) -> bool {
    matches!(self, CallState::Ready(_))
  }

  pub fn on_discarded<F: FnOnce(&CallStateDiscarded)>(&self, fnc: F) -> &Self {
    if let CallState::Discarded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_error<F: FnOnce(&CallStateError)>(&self, fnc: F) -> &Self {
    if let CallState::Error(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_exchanging_keys<F: FnOnce(&CallStateExchangingKeys)>(&self, fnc: F) -> &Self {
    if let CallState::ExchangingKeys(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_hanging_up<F: FnOnce(&CallStateHangingUp)>(&self, fnc: F) -> &Self {
    if let CallState::HangingUp(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&CallStatePending)>(&self, fnc: F) -> &Self {
    if let CallState::Pending(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ready<F: FnOnce(&CallStateReady)>(&self, fnc: F) -> &Self {
    if let CallState::Ready(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_discarded(&self) -> Option<&CallStateDiscarded> {
    if let CallState::Discarded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_error(&self) -> Option<&CallStateError> {
    if let CallState::Error(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_exchanging_keys(&self) -> Option<&CallStateExchangingKeys> {
    if let CallState::ExchangingKeys(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_hanging_up(&self) -> Option<&CallStateHangingUp> {
    if let CallState::HangingUp(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&CallStatePending> {
    if let CallState::Pending(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ready(&self) -> Option<&CallStateReady> {
    if let CallState::Ready(t) = self {
      return Some(t);
    }
    None
  }

  pub fn discarded<T: AsRef<CallStateDiscarded>>(t: T) -> Self {
    CallState::Discarded(t.as_ref().clone())
  }

  pub fn error<T: AsRef<CallStateError>>(t: T) -> Self {
    CallState::Error(t.as_ref().clone())
  }

  pub fn exchanging_keys<T: AsRef<CallStateExchangingKeys>>(t: T) -> Self {
    CallState::ExchangingKeys(t.as_ref().clone())
  }

  pub fn hanging_up<T: AsRef<CallStateHangingUp>>(t: T) -> Self {
    CallState::HangingUp(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<CallStatePending>>(t: T) -> Self {
    CallState::Pending(t.as_ref().clone())
  }

  pub fn ready<T: AsRef<CallStateReady>>(t: T) -> Self {
    CallState::Ready(t.as_ref().clone())
  }
}

impl AsRef<CallState> for CallState {
  fn as_ref(&self) -> &CallState {
    self
  }
}

/// The call has ended successfully
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateDiscarded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The reason why the call has ended
  reason: CallDiscardReason,
  /// True, if the call rating must be sent to the server
  need_rating: bool,
  /// True, if the call debug information must be sent to the server
  need_debug_information: bool,
  /// True, if the call log must be sent to the server
  need_log: bool,
}

impl RObject for CallStateDiscarded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "callStateDiscarded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCallState for CallStateDiscarded {}

impl CallStateDiscarded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCallStateDiscardedBuilder {
    let inner = CallStateDiscarded {
      td_name: "callStateDiscarded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCallStateDiscardedBuilder { inner }
  }

  pub fn reason(&self) -> &CallDiscardReason {
    &self.reason
  }

  pub fn need_rating(&self) -> bool {
    self.need_rating
  }

  pub fn need_debug_information(&self) -> bool {
    self.need_debug_information
  }

  pub fn need_log(&self) -> bool {
    self.need_log
  }
}

#[doc(hidden)]
pub struct RTDCallStateDiscardedBuilder {
  inner: CallStateDiscarded,
}

impl RTDCallStateDiscardedBuilder {
  pub fn build(&self) -> CallStateDiscarded {
    self.inner.clone()
  }

  pub fn reason<T: AsRef<CallDiscardReason>>(&mut self, reason: T) -> &mut Self {
    self.inner.reason = reason.as_ref().clone();
    self
  }

  pub fn need_rating(&mut self, need_rating: bool) -> &mut Self {
    self.inner.need_rating = need_rating;
    self
  }

  pub fn need_debug_information(&mut self, need_debug_information: bool) -> &mut Self {
    self.inner.need_debug_information = need_debug_information;
    self
  }

  pub fn need_log(&mut self, need_log: bool) -> &mut Self {
    self.inner.need_log = need_log;
    self
  }
}

impl AsRef<CallStateDiscarded> for CallStateDiscarded {
  fn as_ref(&self) -> &CallStateDiscarded {
    self
  }
}

impl AsRef<CallStateDiscarded> for RTDCallStateDiscardedBuilder {
  fn as_ref(&self) -> &CallStateDiscarded {
    &self.inner
  }
}

/// The call has ended with an error
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Error. An error with the code 4005000 will be returned if an outgoing call is missed because of an expired timeout
  error: Error,
}

impl RObject for CallStateError {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "callStateError"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCallState for CallStateError {}

impl CallStateError {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCallStateErrorBuilder {
    let inner = CallStateError {
      td_name: "callStateError".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCallStateErrorBuilder { inner }
  }

  pub fn error(&self) -> &Error {
    &self.error
  }
}

#[doc(hidden)]
pub struct RTDCallStateErrorBuilder {
  inner: CallStateError,
}

impl RTDCallStateErrorBuilder {
  pub fn build(&self) -> CallStateError {
    self.inner.clone()
  }

  pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
    self.inner.error = error.as_ref().clone();
    self
  }
}

impl AsRef<CallStateError> for CallStateError {
  fn as_ref(&self) -> &CallStateError {
    self
  }
}

impl AsRef<CallStateError> for RTDCallStateErrorBuilder {
  fn as_ref(&self) -> &CallStateError {
    &self.inner
  }
}

/// The call has been answered and encryption keys are being exchanged
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateExchangingKeys {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CallStateExchangingKeys {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "callStateExchangingKeys"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCallState for CallStateExchangingKeys {}

impl CallStateExchangingKeys {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCallStateExchangingKeysBuilder {
    let inner = CallStateExchangingKeys {
      td_name: "callStateExchangingKeys".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCallStateExchangingKeysBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCallStateExchangingKeysBuilder {
  inner: CallStateExchangingKeys,
}

impl RTDCallStateExchangingKeysBuilder {
  pub fn build(&self) -> CallStateExchangingKeys {
    self.inner.clone()
  }
}

impl AsRef<CallStateExchangingKeys> for CallStateExchangingKeys {
  fn as_ref(&self) -> &CallStateExchangingKeys {
    self
  }
}

impl AsRef<CallStateExchangingKeys> for RTDCallStateExchangingKeysBuilder {
  fn as_ref(&self) -> &CallStateExchangingKeys {
    &self.inner
  }
}

/// The call is hanging up after discardCall has been called
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateHangingUp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CallStateHangingUp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "callStateHangingUp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCallState for CallStateHangingUp {}

impl CallStateHangingUp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCallStateHangingUpBuilder {
    let inner = CallStateHangingUp {
      td_name: "callStateHangingUp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCallStateHangingUpBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCallStateHangingUpBuilder {
  inner: CallStateHangingUp,
}

impl RTDCallStateHangingUpBuilder {
  pub fn build(&self) -> CallStateHangingUp {
    self.inner.clone()
  }
}

impl AsRef<CallStateHangingUp> for CallStateHangingUp {
  fn as_ref(&self) -> &CallStateHangingUp {
    self
  }
}

impl AsRef<CallStateHangingUp> for RTDCallStateHangingUpBuilder {
  fn as_ref(&self) -> &CallStateHangingUp {
    &self.inner
  }
}

/// The call is pending, waiting to be accepted by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the call has already been created by the server
  is_created: bool,
  /// True, if the call has already been received by the other party
  is_received: bool,
}

impl RObject for CallStatePending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "callStatePending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCallState for CallStatePending {}

impl CallStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCallStatePendingBuilder {
    let inner = CallStatePending {
      td_name: "callStatePending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCallStatePendingBuilder { inner }
  }

  pub fn is_created(&self) -> bool {
    self.is_created
  }

  pub fn is_received(&self) -> bool {
    self.is_received
  }
}

#[doc(hidden)]
pub struct RTDCallStatePendingBuilder {
  inner: CallStatePending,
}

impl RTDCallStatePendingBuilder {
  pub fn build(&self) -> CallStatePending {
    self.inner.clone()
  }

  pub fn is_created(&mut self, is_created: bool) -> &mut Self {
    self.inner.is_created = is_created;
    self
  }

  pub fn is_received(&mut self, is_received: bool) -> &mut Self {
    self.inner.is_received = is_received;
    self
  }
}

impl AsRef<CallStatePending> for CallStatePending {
  fn as_ref(&self) -> &CallStatePending {
    self
  }
}

impl AsRef<CallStatePending> for RTDCallStatePendingBuilder {
  fn as_ref(&self) -> &CallStatePending {
    &self.inner
  }
}

/// The call is ready to use
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Call protocols supported by the other call participant
  protocol: CallProtocol,
  /// List of available call servers
  servers: Vec<CallServer>,
  /// A JSON-encoded call config
  config: String,
  /// Call encryption key
  encryption_key: String,
  /// Encryption key fingerprint represented as 4 emoji
  emojis: Vec<String>,
  /// True, if peer-to-peer connection is allowed by users privacy settings
  allow_p2p: bool,
  /// True, if the other party supports upgrading of the call to a group call
  is_group_call_supported: bool,
  /// Custom JSON-encoded call parameters to be passed to tgcalls
  custom_parameters: String,
}

impl RObject for CallStateReady {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "callStateReady"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCallState for CallStateReady {}

impl CallStateReady {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCallStateReadyBuilder {
    let inner = CallStateReady {
      td_name: "callStateReady".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCallStateReadyBuilder { inner }
  }

  pub fn protocol(&self) -> &CallProtocol {
    &self.protocol
  }

  pub fn servers(&self) -> &Vec<CallServer> {
    &self.servers
  }

  pub fn config(&self) -> &String {
    &self.config
  }

  pub fn encryption_key(&self) -> &String {
    &self.encryption_key
  }

  pub fn emojis(&self) -> &Vec<String> {
    &self.emojis
  }

  pub fn allow_p2p(&self) -> bool {
    self.allow_p2p
  }

  pub fn is_group_call_supported(&self) -> bool {
    self.is_group_call_supported
  }

  pub fn custom_parameters(&self) -> &String {
    &self.custom_parameters
  }
}

#[doc(hidden)]
pub struct RTDCallStateReadyBuilder {
  inner: CallStateReady,
}

impl RTDCallStateReadyBuilder {
  pub fn build(&self) -> CallStateReady {
    self.inner.clone()
  }

  pub fn protocol<T: AsRef<CallProtocol>>(&mut self, protocol: T) -> &mut Self {
    self.inner.protocol = protocol.as_ref().clone();
    self
  }

  pub fn servers(&mut self, servers: Vec<CallServer>) -> &mut Self {
    self.inner.servers = servers;
    self
  }

  pub fn config<T: AsRef<str>>(&mut self, config: T) -> &mut Self {
    self.inner.config = config.as_ref().to_string();
    self
  }

  pub fn encryption_key<T: AsRef<str>>(&mut self, encryption_key: T) -> &mut Self {
    self.inner.encryption_key = encryption_key.as_ref().to_string();
    self
  }

  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }

  pub fn allow_p2p(&mut self, allow_p2p: bool) -> &mut Self {
    self.inner.allow_p2p = allow_p2p;
    self
  }

  pub fn is_group_call_supported(&mut self, is_group_call_supported: bool) -> &mut Self {
    self.inner.is_group_call_supported = is_group_call_supported;
    self
  }

  pub fn custom_parameters<T: AsRef<str>>(&mut self, custom_parameters: T) -> &mut Self {
    self.inner.custom_parameters = custom_parameters.as_ref().to_string();
    self
  }
}

impl AsRef<CallStateReady> for CallStateReady {
  fn as_ref(&self) -> &CallStateReady {
    self
  }
}

impl AsRef<CallStateReady> for RTDCallStateReadyBuilder {
  fn as_ref(&self) -> &CallStateReady {
    &self.inner
  }
}
