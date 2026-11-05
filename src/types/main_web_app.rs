use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about the main Web App of a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MainWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the Web App to open
  url: WebAppUrl,
  /// The mode in which the Web App must be opened
  mode: WebAppOpenMode,
}

impl RObject for MainWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "mainWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MainWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMainWebAppBuilder {
    let inner = MainWebApp {
      td_name: "mainWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMainWebAppBuilder { inner }
  }

  pub fn url(&self) -> &WebAppUrl {
    &self.url
  }

  pub fn mode(&self) -> &WebAppOpenMode {
    &self.mode
  }
}

#[doc(hidden)]
pub struct RTDMainWebAppBuilder {
  inner: MainWebApp,
}

impl RTDMainWebAppBuilder {
  pub fn build(&self) -> MainWebApp {
    self.inner.clone()
  }

  pub fn url<T: AsRef<WebAppUrl>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().clone();
    self
  }

  pub fn mode<T: AsRef<WebAppOpenMode>>(&mut self, mode: T) -> &mut Self {
    self.inner.mode = mode.as_ref().clone();
    self
  }
}

impl AsRef<MainWebApp> for MainWebApp {
  fn as_ref(&self) -> &MainWebApp {
    self
  }
}

impl AsRef<MainWebApp> for RTDMainWebAppBuilder {
  fn as_ref(&self) -> &MainWebApp {
    &self.inner
  }
}
