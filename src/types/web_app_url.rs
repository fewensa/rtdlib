use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Web App URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAppUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The Web App URL to open in a web view
  url: String,
  /// True, if events from the Web App must be accepted only from the same origin as the URL
  require_same_origin: bool,
}

impl RObject for WebAppUrl {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webAppUrl"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebAppUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebAppUrlBuilder {
    let inner = WebAppUrl {
      td_name: "webAppUrl".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebAppUrlBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn require_same_origin(&self) -> bool {
    self.require_same_origin
  }
}

#[doc(hidden)]
pub struct RTDWebAppUrlBuilder {
  inner: WebAppUrl,
}

impl RTDWebAppUrlBuilder {
  pub fn build(&self) -> WebAppUrl {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn require_same_origin(&mut self, require_same_origin: bool) -> &mut Self {
    self.inner.require_same_origin = require_same_origin;
    self
  }
}

impl AsRef<WebAppUrl> for WebAppUrl {
  fn as_ref(&self) -> &WebAppUrl {
    self
  }
}

impl AsRef<WebAppUrl> for RTDWebAppUrlBuilder {
  fn as_ref(&self) -> &WebAppUrl {
    &self.inner
  }
}
