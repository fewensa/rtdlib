use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of added proxy servers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddedProxies {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of proxy servers
  proxies: Vec<AddedProxy>,
}

impl RObject for AddedProxies {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "addedProxies"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AddedProxies {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAddedProxiesBuilder {
    let inner = AddedProxies {
      td_name: "addedProxies".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAddedProxiesBuilder { inner }
  }

  pub fn proxies(&self) -> &Vec<AddedProxy> {
    &self.proxies
  }
}

#[doc(hidden)]
pub struct RTDAddedProxiesBuilder {
  inner: AddedProxies,
}

impl RTDAddedProxiesBuilder {
  pub fn build(&self) -> AddedProxies {
    self.inner.clone()
  }

  pub fn proxies(&mut self, proxies: Vec<AddedProxy>) -> &mut Self {
    self.inner.proxies = proxies;
    self
  }
}

impl AsRef<AddedProxies> for AddedProxies {
  fn as_ref(&self) -> &AddedProxies {
    self
  }
}

impl AsRef<AddedProxies> for RTDAddedProxiesBuilder {
  fn as_ref(&self) -> &AddedProxies {
    &self.inner
  }
}
