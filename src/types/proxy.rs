use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a proxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Proxy server domain or IP address
  server: String,
  /// Proxy server port
  port: i64,
  /// Type of the proxy
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: ProxyType,
}

impl RObject for Proxy {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "proxy"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Proxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProxyBuilder {
    let inner = Proxy {
      td_name: "proxy".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDProxyBuilder { inner }
  }

  pub fn server(&self) -> &String {
    &self.server
  }

  pub fn port(&self) -> i64 {
    self.port
  }

  pub fn type_(&self) -> &ProxyType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDProxyBuilder {
  inner: Proxy,
}

impl RTDProxyBuilder {
  pub fn build(&self) -> Proxy {
    self.inner.clone()
  }

  pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
    self.inner.server = server.as_ref().to_string();
    self
  }

  pub fn port(&mut self, port: i64) -> &mut Self {
    self.inner.port = port;
    self
  }

  pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<Proxy> for Proxy {
  fn as_ref(&self) -> &Proxy {
    self
  }
}

impl AsRef<Proxy> for RTDProxyBuilder {
  fn as_ref(&self) -> &Proxy {
    &self.inner
  }
}
