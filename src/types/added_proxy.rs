use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a proxy server added to the list of proxies
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddedProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the proxy
  id: i64,
  /// Point in time (Unix timestamp) when the proxy was last used; 0 if never
  last_used_date: i64,
  /// True, if the proxy is enabled now
  is_enabled: bool,
  /// Comment for the proxy added by the user
  comment: String,
  /// The proxy
  proxy: Proxy,
}

impl RObject for AddedProxy {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "addedProxy"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AddedProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAddedProxyBuilder {
    let inner = AddedProxy {
      td_name: "addedProxy".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAddedProxyBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn last_used_date(&self) -> i64 {
    self.last_used_date
  }

  pub fn is_enabled(&self) -> bool {
    self.is_enabled
  }

  pub fn comment(&self) -> &String {
    &self.comment
  }

  pub fn proxy(&self) -> &Proxy {
    &self.proxy
  }
}

#[doc(hidden)]
pub struct RTDAddedProxyBuilder {
  inner: AddedProxy,
}

impl RTDAddedProxyBuilder {
  pub fn build(&self) -> AddedProxy {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn last_used_date(&mut self, last_used_date: i64) -> &mut Self {
    self.inner.last_used_date = last_used_date;
    self
  }

  pub fn is_enabled(&mut self, is_enabled: bool) -> &mut Self {
    self.inner.is_enabled = is_enabled;
    self
  }

  pub fn comment<T: AsRef<str>>(&mut self, comment: T) -> &mut Self {
    self.inner.comment = comment.as_ref().to_string();
    self
  }

  pub fn proxy<T: AsRef<Proxy>>(&mut self, proxy: T) -> &mut Self {
    self.inner.proxy = proxy.as_ref().clone();
    self
  }
}

impl AsRef<AddedProxy> for AddedProxy {
  fn as_ref(&self) -> &AddedProxy {
    self
  }
}

impl AsRef<AddedProxy> for RTDAddedProxyBuilder {
  fn as_ref(&self) -> &AddedProxy {
    &self.inner
  }
}
