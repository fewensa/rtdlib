use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Options to be used when a Web App is opened
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebAppOpenParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Preferred Web App theme; pass null to use the default theme
  theme: ThemeParameters,
  /// Short name of the current application; 0-64 English letters, digits, and underscores
  application_name: String,
  /// The mode in which the Web App is opened; pass null to open in webAppOpenModeFullSize
  mode: WebAppOpenMode,
}

impl RObject for WebAppOpenParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webAppOpenParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebAppOpenParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebAppOpenParametersBuilder {
    let inner = WebAppOpenParameters {
      td_name: "webAppOpenParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebAppOpenParametersBuilder { inner }
  }

  pub fn theme(&self) -> &ThemeParameters {
    &self.theme
  }

  pub fn application_name(&self) -> &String {
    &self.application_name
  }

  pub fn mode(&self) -> &WebAppOpenMode {
    &self.mode
  }
}

#[doc(hidden)]
pub struct RTDWebAppOpenParametersBuilder {
  inner: WebAppOpenParameters,
}

impl RTDWebAppOpenParametersBuilder {
  pub fn build(&self) -> WebAppOpenParameters {
    self.inner.clone()
  }

  pub fn theme<T: AsRef<ThemeParameters>>(&mut self, theme: T) -> &mut Self {
    self.inner.theme = theme.as_ref().clone();
    self
  }

  pub fn application_name<T: AsRef<str>>(&mut self, application_name: T) -> &mut Self {
    self.inner.application_name = application_name.as_ref().to_string();
    self
  }

  pub fn mode<T: AsRef<WebAppOpenMode>>(&mut self, mode: T) -> &mut Self {
    self.inner.mode = mode.as_ref().clone();
    self
  }
}

impl AsRef<WebAppOpenParameters> for WebAppOpenParameters {
  fn as_ref(&self) -> &WebAppOpenParameters {
    self
  }
}

impl AsRef<WebAppOpenParameters> for RTDWebAppOpenParametersBuilder {
  fn as_ref(&self) -> &WebAppOpenParameters {
    &self.inner
  }
}
