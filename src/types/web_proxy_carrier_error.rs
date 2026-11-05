use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the reason why a web proxy carrier failed
pub trait TDWebProxyCarrierError: Debug + RObject {}

/// Describes the reason why a web proxy carrier failed
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum WebProxyCarrierError {
  #[doc(hidden)]
  _Default(()),
  /// The page failed to load: a network, TLS, or HTTP error, or a forbidden navigation
  LoadFailed(WebProxyCarrierErrorLoadFailed),
  /// The application can't open the carrier now, for example, because it is in the background; TDLib will request it again after the next call to setNetworkType or after a delay
  Paused(WebProxyCarrierErrorPaused),
  /// The application refused to open a temporary carrier
  Refused(WebProxyCarrierErrorRefused),
  /// The web view or its renderer process was terminated, or passing data between the application and the page failed
  Terminated(WebProxyCarrierErrorTerminated),
  /// The application can't open the carrier, because the platform can't create hidden web views or define the bridge object; TDLib will not retry until the proxy is enabled again
  Unsupported(WebProxyCarrierErrorUnsupported),
}

impl Default for WebProxyCarrierError {
  fn default() -> Self {
    WebProxyCarrierError::_Default(())
  }
}

impl<'de> Deserialize<'de> for WebProxyCarrierError {
  fn deserialize<D>(deserializer: D) -> Result<WebProxyCarrierError, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      WebProxyCarrierError,
      (webProxyCarrierErrorLoadFailed, LoadFailed);
      (webProxyCarrierErrorPaused, Paused);
      (webProxyCarrierErrorRefused, Refused);
      (webProxyCarrierErrorTerminated, Terminated);
      (webProxyCarrierErrorUnsupported, Unsupported);

    )(deserializer)
  }
}

impl RObject for WebProxyCarrierError {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      WebProxyCarrierError::LoadFailed(t) => t.td_name(),
      WebProxyCarrierError::Paused(t) => t.td_name(),
      WebProxyCarrierError::Refused(t) => t.td_name(),
      WebProxyCarrierError::Terminated(t) => t.td_name(),
      WebProxyCarrierError::Unsupported(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      WebProxyCarrierError::LoadFailed(t) => t.extra(),
      WebProxyCarrierError::Paused(t) => t.extra(),
      WebProxyCarrierError::Refused(t) => t.extra(),
      WebProxyCarrierError::Terminated(t) => t.extra(),
      WebProxyCarrierError::Unsupported(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebProxyCarrierError {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, WebProxyCarrierError::_Default(_))
  }

  pub fn is_load_failed(&self) -> bool {
    matches!(self, WebProxyCarrierError::LoadFailed(_))
  }
  pub fn is_paused(&self) -> bool {
    matches!(self, WebProxyCarrierError::Paused(_))
  }
  pub fn is_refused(&self) -> bool {
    matches!(self, WebProxyCarrierError::Refused(_))
  }
  pub fn is_terminated(&self) -> bool {
    matches!(self, WebProxyCarrierError::Terminated(_))
  }
  pub fn is_unsupported(&self) -> bool {
    matches!(self, WebProxyCarrierError::Unsupported(_))
  }

  pub fn on_load_failed<F: FnOnce(&WebProxyCarrierErrorLoadFailed)>(&self, fnc: F) -> &Self {
    if let WebProxyCarrierError::LoadFailed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paused<F: FnOnce(&WebProxyCarrierErrorPaused)>(&self, fnc: F) -> &Self {
    if let WebProxyCarrierError::Paused(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_refused<F: FnOnce(&WebProxyCarrierErrorRefused)>(&self, fnc: F) -> &Self {
    if let WebProxyCarrierError::Refused(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_terminated<F: FnOnce(&WebProxyCarrierErrorTerminated)>(&self, fnc: F) -> &Self {
    if let WebProxyCarrierError::Terminated(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unsupported<F: FnOnce(&WebProxyCarrierErrorUnsupported)>(&self, fnc: F) -> &Self {
    if let WebProxyCarrierError::Unsupported(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_load_failed(&self) -> Option<&WebProxyCarrierErrorLoadFailed> {
    if let WebProxyCarrierError::LoadFailed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paused(&self) -> Option<&WebProxyCarrierErrorPaused> {
    if let WebProxyCarrierError::Paused(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_refused(&self) -> Option<&WebProxyCarrierErrorRefused> {
    if let WebProxyCarrierError::Refused(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_terminated(&self) -> Option<&WebProxyCarrierErrorTerminated> {
    if let WebProxyCarrierError::Terminated(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unsupported(&self) -> Option<&WebProxyCarrierErrorUnsupported> {
    if let WebProxyCarrierError::Unsupported(t) = self {
      return Some(t);
    }
    None
  }

  pub fn load_failed<T: AsRef<WebProxyCarrierErrorLoadFailed>>(t: T) -> Self {
    WebProxyCarrierError::LoadFailed(t.as_ref().clone())
  }

  pub fn paused<T: AsRef<WebProxyCarrierErrorPaused>>(t: T) -> Self {
    WebProxyCarrierError::Paused(t.as_ref().clone())
  }

  pub fn refused<T: AsRef<WebProxyCarrierErrorRefused>>(t: T) -> Self {
    WebProxyCarrierError::Refused(t.as_ref().clone())
  }

  pub fn terminated<T: AsRef<WebProxyCarrierErrorTerminated>>(t: T) -> Self {
    WebProxyCarrierError::Terminated(t.as_ref().clone())
  }

  pub fn unsupported<T: AsRef<WebProxyCarrierErrorUnsupported>>(t: T) -> Self {
    WebProxyCarrierError::Unsupported(t.as_ref().clone())
  }
}

impl AsRef<WebProxyCarrierError> for WebProxyCarrierError {
  fn as_ref(&self) -> &WebProxyCarrierError {
    self
  }
}

/// The page failed to load: a network, TLS, or HTTP error, or a forbidden navigation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebProxyCarrierErrorLoadFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The error to be logged; must not contain the URL
  message: String,
}

impl RObject for WebProxyCarrierErrorLoadFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webProxyCarrierErrorLoadFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebProxyCarrierError for WebProxyCarrierErrorLoadFailed {}

impl WebProxyCarrierErrorLoadFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebProxyCarrierErrorLoadFailedBuilder {
    let inner = WebProxyCarrierErrorLoadFailed {
      td_name: "webProxyCarrierErrorLoadFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebProxyCarrierErrorLoadFailedBuilder { inner }
  }

  pub fn message(&self) -> &String {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDWebProxyCarrierErrorLoadFailedBuilder {
  inner: WebProxyCarrierErrorLoadFailed,
}

impl RTDWebProxyCarrierErrorLoadFailedBuilder {
  pub fn build(&self) -> WebProxyCarrierErrorLoadFailed {
    self.inner.clone()
  }

  pub fn message<T: AsRef<str>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().to_string();
    self
  }
}

impl AsRef<WebProxyCarrierErrorLoadFailed> for WebProxyCarrierErrorLoadFailed {
  fn as_ref(&self) -> &WebProxyCarrierErrorLoadFailed {
    self
  }
}

impl AsRef<WebProxyCarrierErrorLoadFailed> for RTDWebProxyCarrierErrorLoadFailedBuilder {
  fn as_ref(&self) -> &WebProxyCarrierErrorLoadFailed {
    &self.inner
  }
}

/// The application can't open the carrier now, for example, because it is in the background; TDLib will request it again after the next call to setNetworkType or after a delay
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebProxyCarrierErrorPaused {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebProxyCarrierErrorPaused {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webProxyCarrierErrorPaused"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebProxyCarrierError for WebProxyCarrierErrorPaused {}

impl WebProxyCarrierErrorPaused {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebProxyCarrierErrorPausedBuilder {
    let inner = WebProxyCarrierErrorPaused {
      td_name: "webProxyCarrierErrorPaused".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebProxyCarrierErrorPausedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebProxyCarrierErrorPausedBuilder {
  inner: WebProxyCarrierErrorPaused,
}

impl RTDWebProxyCarrierErrorPausedBuilder {
  pub fn build(&self) -> WebProxyCarrierErrorPaused {
    self.inner.clone()
  }
}

impl AsRef<WebProxyCarrierErrorPaused> for WebProxyCarrierErrorPaused {
  fn as_ref(&self) -> &WebProxyCarrierErrorPaused {
    self
  }
}

impl AsRef<WebProxyCarrierErrorPaused> for RTDWebProxyCarrierErrorPausedBuilder {
  fn as_ref(&self) -> &WebProxyCarrierErrorPaused {
    &self.inner
  }
}

/// The application refused to open a temporary carrier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebProxyCarrierErrorRefused {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebProxyCarrierErrorRefused {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webProxyCarrierErrorRefused"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebProxyCarrierError for WebProxyCarrierErrorRefused {}

impl WebProxyCarrierErrorRefused {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebProxyCarrierErrorRefusedBuilder {
    let inner = WebProxyCarrierErrorRefused {
      td_name: "webProxyCarrierErrorRefused".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebProxyCarrierErrorRefusedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebProxyCarrierErrorRefusedBuilder {
  inner: WebProxyCarrierErrorRefused,
}

impl RTDWebProxyCarrierErrorRefusedBuilder {
  pub fn build(&self) -> WebProxyCarrierErrorRefused {
    self.inner.clone()
  }
}

impl AsRef<WebProxyCarrierErrorRefused> for WebProxyCarrierErrorRefused {
  fn as_ref(&self) -> &WebProxyCarrierErrorRefused {
    self
  }
}

impl AsRef<WebProxyCarrierErrorRefused> for RTDWebProxyCarrierErrorRefusedBuilder {
  fn as_ref(&self) -> &WebProxyCarrierErrorRefused {
    &self.inner
  }
}

/// The web view or its renderer process was terminated, or passing data between the application and the page failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebProxyCarrierErrorTerminated {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebProxyCarrierErrorTerminated {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webProxyCarrierErrorTerminated"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebProxyCarrierError for WebProxyCarrierErrorTerminated {}

impl WebProxyCarrierErrorTerminated {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebProxyCarrierErrorTerminatedBuilder {
    let inner = WebProxyCarrierErrorTerminated {
      td_name: "webProxyCarrierErrorTerminated".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebProxyCarrierErrorTerminatedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebProxyCarrierErrorTerminatedBuilder {
  inner: WebProxyCarrierErrorTerminated,
}

impl RTDWebProxyCarrierErrorTerminatedBuilder {
  pub fn build(&self) -> WebProxyCarrierErrorTerminated {
    self.inner.clone()
  }
}

impl AsRef<WebProxyCarrierErrorTerminated> for WebProxyCarrierErrorTerminated {
  fn as_ref(&self) -> &WebProxyCarrierErrorTerminated {
    self
  }
}

impl AsRef<WebProxyCarrierErrorTerminated> for RTDWebProxyCarrierErrorTerminatedBuilder {
  fn as_ref(&self) -> &WebProxyCarrierErrorTerminated {
    &self.inner
  }
}

/// The application can't open the carrier, because the platform can't create hidden web views or define the bridge object; TDLib will not retry until the proxy is enabled again
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebProxyCarrierErrorUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebProxyCarrierErrorUnsupported {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webProxyCarrierErrorUnsupported"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebProxyCarrierError for WebProxyCarrierErrorUnsupported {}

impl WebProxyCarrierErrorUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebProxyCarrierErrorUnsupportedBuilder {
    let inner = WebProxyCarrierErrorUnsupported {
      td_name: "webProxyCarrierErrorUnsupported".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebProxyCarrierErrorUnsupportedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebProxyCarrierErrorUnsupportedBuilder {
  inner: WebProxyCarrierErrorUnsupported,
}

impl RTDWebProxyCarrierErrorUnsupportedBuilder {
  pub fn build(&self) -> WebProxyCarrierErrorUnsupported {
    self.inner.clone()
  }
}

impl AsRef<WebProxyCarrierErrorUnsupported> for WebProxyCarrierErrorUnsupported {
  fn as_ref(&self) -> &WebProxyCarrierErrorUnsupported {
    self
  }
}

impl AsRef<WebProxyCarrierErrorUnsupported> for RTDWebProxyCarrierErrorUnsupportedBuilder {
  fn as_ref(&self) -> &WebProxyCarrierErrorUnsupported {
    &self.inner
  }
}
