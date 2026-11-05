use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes web browser settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebBrowserSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if links are opened in an external browser by default
  open_external_browser: bool,
  /// The list of websites which must always be opened in an external browser
  external_exceptions: Vec<WebDomainException>,
  /// The list of websites which must always be opened in the in-app browser
  in_app_exceptions: Vec<WebDomainException>,
  /// True, if a close button must be shown in the in-app browser; for Android app only
  display_close_button: bool,
}

impl RObject for WebBrowserSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webBrowserSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebBrowserSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebBrowserSettingsBuilder {
    let inner = WebBrowserSettings {
      td_name: "webBrowserSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebBrowserSettingsBuilder { inner }
  }

  pub fn open_external_browser(&self) -> bool {
    self.open_external_browser
  }

  pub fn external_exceptions(&self) -> &Vec<WebDomainException> {
    &self.external_exceptions
  }

  pub fn in_app_exceptions(&self) -> &Vec<WebDomainException> {
    &self.in_app_exceptions
  }

  pub fn display_close_button(&self) -> bool {
    self.display_close_button
  }
}

#[doc(hidden)]
pub struct RTDWebBrowserSettingsBuilder {
  inner: WebBrowserSettings,
}

impl RTDWebBrowserSettingsBuilder {
  pub fn build(&self) -> WebBrowserSettings {
    self.inner.clone()
  }

  pub fn open_external_browser(&mut self, open_external_browser: bool) -> &mut Self {
    self.inner.open_external_browser = open_external_browser;
    self
  }

  pub fn external_exceptions(&mut self, external_exceptions: Vec<WebDomainException>) -> &mut Self {
    self.inner.external_exceptions = external_exceptions;
    self
  }

  pub fn in_app_exceptions(&mut self, in_app_exceptions: Vec<WebDomainException>) -> &mut Self {
    self.inner.in_app_exceptions = in_app_exceptions;
    self
  }

  pub fn display_close_button(&mut self, display_close_button: bool) -> &mut Self {
    self.inner.display_close_button = display_close_button;
    self
  }
}

impl AsRef<WebBrowserSettings> for WebBrowserSettings {
  fn as_ref(&self) -> &WebBrowserSettings {
    self
  }
}

impl AsRef<WebBrowserSettings> for RTDWebBrowserSettingsBuilder {
  fn as_ref(&self) -> &WebBrowserSettings {
    &self.inner
  }
}
