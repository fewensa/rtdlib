use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Web App
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAppInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier for the Web App launch
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  launch_id: isize,
  /// The Web App URL to open in a web view
  url: WebAppUrl,
}

impl RObject for WebAppInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webAppInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebAppInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebAppInfoBuilder {
    let inner = WebAppInfo {
      td_name: "webAppInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebAppInfoBuilder { inner }
  }

  pub fn launch_id(&self) -> isize {
    self.launch_id
  }

  pub fn url(&self) -> &WebAppUrl {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDWebAppInfoBuilder {
  inner: WebAppInfo,
}

impl RTDWebAppInfoBuilder {
  pub fn build(&self) -> WebAppInfo {
    self.inner.clone()
  }

  pub fn launch_id(&mut self, launch_id: isize) -> &mut Self {
    self.inner.launch_id = launch_id;
    self
  }

  pub fn url<T: AsRef<WebAppUrl>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().clone();
    self
  }
}

impl AsRef<WebAppInfo> for WebAppInfo {
  fn as_ref(&self) -> &WebAppInfo {
    self
  }
}

impl AsRef<WebAppInfo> for RTDWebAppInfoBuilder {
  fn as_ref(&self) -> &WebAppInfo {
    &self.inner
  }
}
