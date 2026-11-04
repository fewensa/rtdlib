use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of web browser
pub trait TDWebBrowserType: Debug + RObject {}

/// Describes the type of web browser
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum WebBrowserType {
  #[doc(hidden)]
  _Default(()),
  /// Returns a type of the web browser which must be used to open the link
  GetLinkWebBrowserType(GetLinkWebBrowserType),
  /// An external web browser
  External(WebBrowserTypeExternal),
  /// The in-app browser
  InApp(WebBrowserTypeInApp),
}

impl Default for WebBrowserType {
  fn default() -> Self {
    WebBrowserType::_Default(())
  }
}

impl<'de> Deserialize<'de> for WebBrowserType {
  fn deserialize<D>(deserializer: D) -> Result<WebBrowserType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      WebBrowserType,
      (getLinkWebBrowserType, GetLinkWebBrowserType);
      (webBrowserTypeExternal, External);
      (webBrowserTypeInApp, InApp);

    )(deserializer)
  }
}

impl RObject for WebBrowserType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      WebBrowserType::GetLinkWebBrowserType(t) => t.td_name(),
      WebBrowserType::External(t) => t.td_name(),
      WebBrowserType::InApp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      WebBrowserType::GetLinkWebBrowserType(t) => t.extra(),
      WebBrowserType::External(t) => t.extra(),
      WebBrowserType::InApp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebBrowserType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, WebBrowserType::_Default(_))
  }

  pub fn is_get_link_web_browser_type(&self) -> bool {
    matches!(self, WebBrowserType::GetLinkWebBrowserType(_))
  }
  pub fn is_external(&self) -> bool {
    matches!(self, WebBrowserType::External(_))
  }
  pub fn is_in_app(&self) -> bool {
    matches!(self, WebBrowserType::InApp(_))
  }

  pub fn on_get_link_web_browser_type<F: FnOnce(&GetLinkWebBrowserType)>(&self, fnc: F) -> &Self {
    if let WebBrowserType::GetLinkWebBrowserType(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_external<F: FnOnce(&WebBrowserTypeExternal)>(&self, fnc: F) -> &Self {
    if let WebBrowserType::External(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_in_app<F: FnOnce(&WebBrowserTypeInApp)>(&self, fnc: F) -> &Self {
    if let WebBrowserType::InApp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_get_link_web_browser_type(&self) -> Option<&GetLinkWebBrowserType> {
    if let WebBrowserType::GetLinkWebBrowserType(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_external(&self) -> Option<&WebBrowserTypeExternal> {
    if let WebBrowserType::External(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_in_app(&self) -> Option<&WebBrowserTypeInApp> {
    if let WebBrowserType::InApp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn get_link_web_browser_type<T: AsRef<GetLinkWebBrowserType>>(t: T) -> Self {
    WebBrowserType::GetLinkWebBrowserType(t.as_ref().clone())
  }

  pub fn external<T: AsRef<WebBrowserTypeExternal>>(t: T) -> Self {
    WebBrowserType::External(t.as_ref().clone())
  }

  pub fn in_app<T: AsRef<WebBrowserTypeInApp>>(t: T) -> Self {
    WebBrowserType::InApp(t.as_ref().clone())
  }
}

impl AsRef<WebBrowserType> for WebBrowserType {
  fn as_ref(&self) -> &WebBrowserType {
    self
  }
}

/// An external web browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebBrowserTypeExternal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebBrowserTypeExternal {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webBrowserTypeExternal"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebBrowserType for WebBrowserTypeExternal {}

impl WebBrowserTypeExternal {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebBrowserTypeExternalBuilder {
    let inner = WebBrowserTypeExternal {
      td_name: "webBrowserTypeExternal".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebBrowserTypeExternalBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebBrowserTypeExternalBuilder {
  inner: WebBrowserTypeExternal,
}

impl RTDWebBrowserTypeExternalBuilder {
  pub fn build(&self) -> WebBrowserTypeExternal {
    self.inner.clone()
  }
}

impl AsRef<WebBrowserTypeExternal> for WebBrowserTypeExternal {
  fn as_ref(&self) -> &WebBrowserTypeExternal {
    self
  }
}

impl AsRef<WebBrowserTypeExternal> for RTDWebBrowserTypeExternalBuilder {
  fn as_ref(&self) -> &WebBrowserTypeExternal {
    &self.inner
  }
}

/// The in-app browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebBrowserTypeInApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for WebBrowserTypeInApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webBrowserTypeInApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDWebBrowserType for WebBrowserTypeInApp {}

impl WebBrowserTypeInApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebBrowserTypeInAppBuilder {
    let inner = WebBrowserTypeInApp {
      td_name: "webBrowserTypeInApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDWebBrowserTypeInAppBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDWebBrowserTypeInAppBuilder {
  inner: WebBrowserTypeInApp,
}

impl RTDWebBrowserTypeInAppBuilder {
  pub fn build(&self) -> WebBrowserTypeInApp {
    self.inner.clone()
  }
}

impl AsRef<WebBrowserTypeInApp> for WebBrowserTypeInApp {
  fn as_ref(&self) -> &WebBrowserTypeInApp {
    self
  }
}

impl AsRef<WebBrowserTypeInApp> for RTDWebBrowserTypeInAppBuilder {
  fn as_ref(&self) -> &WebBrowserTypeInApp {
    &self.inner
  }
}
