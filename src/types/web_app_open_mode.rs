use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes mode in which a Web App is opened
pub trait TDWebAppOpenMode: Debug + RObject {}

/// Describes mode in which a Web App is opened
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum WebAppOpenMode {
  #[doc(hidden)]
  _Default(()),
  /// The Web App is opened in the compact mode
  Compact(WebAppOpenModeCompact),
  /// The Web App is opened in the full-screen mode
  FullScreen(WebAppOpenModeFullScreen),
  /// The Web App is opened in the full-size mode
  FullSize(WebAppOpenModeFullSize),
}

impl Default for WebAppOpenMode {
  fn default() -> Self {
    WebAppOpenMode::_Default(())
  }
}

impl<'de> Deserialize<'de> for WebAppOpenMode {
  fn deserialize<D>(deserializer: D) -> Result<WebAppOpenMode, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      WebAppOpenMode,
      (webAppOpenModeCompact, Compact);
      (webAppOpenModeFullScreen, FullScreen);
      (webAppOpenModeFullSize, FullSize);

    )(deserializer)
  }
}

impl RObject for WebAppOpenMode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      WebAppOpenMode::Compact(t) => t.td_name(),
      WebAppOpenMode::FullScreen(t) => t.td_name(),
      WebAppOpenMode::FullSize(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      WebAppOpenMode::Compact(t) => t.extra(),
      WebAppOpenMode::FullScreen(t) => t.extra(),
      WebAppOpenMode::FullSize(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebAppOpenMode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, WebAppOpenMode::_Default(_))
  }

  pub fn is_compact(&self) -> bool {
    matches!(self, WebAppOpenMode::Compact(_))
  }
  pub fn is_full_screen(&self) -> bool {
    matches!(self, WebAppOpenMode::FullScreen(_))
  }
  pub fn is_full_size(&self) -> bool {
    matches!(self, WebAppOpenMode::FullSize(_))
  }

  pub fn on_compact<F: FnOnce(&WebAppOpenModeCompact)>(&self, fnc: F) -> &Self {
    if let WebAppOpenMode::Compact(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_full_screen<F: FnOnce(&WebAppOpenModeFullScreen)>(&self, fnc: F) -> &Self {
    if let WebAppOpenMode::FullScreen(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_full_size<F: FnOnce(&WebAppOpenModeFullSize)>(&self, fnc: F) -> &Self {
    if let WebAppOpenMode::FullSize(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_compact(&self) -> Option<&WebAppOpenModeCompact> {
    if let WebAppOpenMode::Compact(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_full_screen(&self) -> Option<&WebAppOpenModeFullScreen> {
    if let WebAppOpenMode::FullScreen(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_full_size(&self) -> Option<&WebAppOpenModeFullSize> {
    if let WebAppOpenMode::FullSize(t) = self {
      return Some(t);
    }
    None
  }

  pub fn compact<T: AsRef<WebAppOpenModeCompact>>(t: T) -> Self {
    WebAppOpenMode::Compact(t.as_ref().clone())
  }

  pub fn full_screen<T: AsRef<WebAppOpenModeFullScreen>>(t: T) -> Self {
    WebAppOpenMode::FullScreen(t.as_ref().clone())
  }

  pub fn full_size<T: AsRef<WebAppOpenModeFullSize>>(t: T) -> Self {
    WebAppOpenMode::FullSize(t.as_ref().clone())
  }
}

impl AsRef<WebAppOpenMode> for WebAppOpenMode {
  fn as_ref(&self) -> &WebAppOpenMode {
    self
  }
}

/// The Web App is opened in the compact mode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAppOpenModeCompact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebAppOpenModeCompact {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webAppOpenModeCompact"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebAppOpenMode for WebAppOpenModeCompact {}

impl WebAppOpenModeCompact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebAppOpenModeCompactBuilder {
    let inner = WebAppOpenModeCompact {
      td_name: "webAppOpenModeCompact".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebAppOpenModeCompactBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebAppOpenModeCompactBuilder {
  inner: WebAppOpenModeCompact,
}

impl RTDWebAppOpenModeCompactBuilder {
  pub fn build(&self) -> WebAppOpenModeCompact {
    self.inner.clone()
  }
}

impl AsRef<WebAppOpenModeCompact> for WebAppOpenModeCompact {
  fn as_ref(&self) -> &WebAppOpenModeCompact {
    self
  }
}

impl AsRef<WebAppOpenModeCompact> for RTDWebAppOpenModeCompactBuilder {
  fn as_ref(&self) -> &WebAppOpenModeCompact {
    &self.inner
  }
}

/// The Web App is opened in the full-screen mode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAppOpenModeFullScreen {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebAppOpenModeFullScreen {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webAppOpenModeFullScreen"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebAppOpenMode for WebAppOpenModeFullScreen {}

impl WebAppOpenModeFullScreen {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebAppOpenModeFullScreenBuilder {
    let inner = WebAppOpenModeFullScreen {
      td_name: "webAppOpenModeFullScreen".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebAppOpenModeFullScreenBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebAppOpenModeFullScreenBuilder {
  inner: WebAppOpenModeFullScreen,
}

impl RTDWebAppOpenModeFullScreenBuilder {
  pub fn build(&self) -> WebAppOpenModeFullScreen {
    self.inner.clone()
  }
}

impl AsRef<WebAppOpenModeFullScreen> for WebAppOpenModeFullScreen {
  fn as_ref(&self) -> &WebAppOpenModeFullScreen {
    self
  }
}

impl AsRef<WebAppOpenModeFullScreen> for RTDWebAppOpenModeFullScreenBuilder {
  fn as_ref(&self) -> &WebAppOpenModeFullScreen {
    &self.inner
  }
}

/// The Web App is opened in the full-size mode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAppOpenModeFullSize {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebAppOpenModeFullSize {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webAppOpenModeFullSize"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebAppOpenMode for WebAppOpenModeFullSize {}

impl WebAppOpenModeFullSize {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebAppOpenModeFullSizeBuilder {
    let inner = WebAppOpenModeFullSize {
      td_name: "webAppOpenModeFullSize".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebAppOpenModeFullSizeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebAppOpenModeFullSizeBuilder {
  inner: WebAppOpenModeFullSize,
}

impl RTDWebAppOpenModeFullSizeBuilder {
  pub fn build(&self) -> WebAppOpenModeFullSize {
    self.inner.clone()
  }
}

impl AsRef<WebAppOpenModeFullSize> for WebAppOpenModeFullSize {
  fn as_ref(&self) -> &WebAppOpenModeFullSize {
    self
  }
}

impl AsRef<WebAppOpenModeFullSize> for RTDWebAppOpenModeFullSizeBuilder {
  fn as_ref(&self) -> &WebAppOpenModeFullSize {
    &self.inner
  }
}
