use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of on-ramp providers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnRampProviders {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of providers
  providers: Vec<OnRampProvider>,
}

impl RObject for OnRampProviders {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "onRampProviders"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl OnRampProviders {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOnRampProvidersBuilder {
    let inner = OnRampProviders {
      td_name: "onRampProviders".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOnRampProvidersBuilder { inner }
  }

  pub fn providers(&self) -> &Vec<OnRampProvider> {
    &self.providers
  }
}

#[doc(hidden)]
pub struct RTDOnRampProvidersBuilder {
  inner: OnRampProviders,
}

impl RTDOnRampProvidersBuilder {
  pub fn build(&self) -> OnRampProviders {
    self.inner.clone()
  }

  pub fn providers(&mut self, providers: Vec<OnRampProvider>) -> &mut Self {
    self.inner.providers = providers;
    self
  }
}

impl AsRef<OnRampProviders> for OnRampProviders {
  fn as_ref(&self) -> &OnRampProviders {
    self
  }
}

impl AsRef<OnRampProviders> for RTDOnRampProvidersBuilder {
  fn as_ref(&self) -> &OnRampProviders {
    &self.inner
  }
}
