use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a web proxy carrier: an HTTPS relay page, which carries traffic of a web proxy and must be loaded by the application as the top-level document of a hidden web view. Before loading the page, the application must arrange for an object named carrier.bridge_object_name to be defined in the top-level document whenever its origin is exactly carrier.origin, and in no other document or frame. The object must be read-only and must have two members: a method postMessage(value), which passes an ArrayBuffer or a typed array to processWebProxyCarrierMessage as webProxyCarrierMessageBinary, passes a string as webProxyCarrierMessageText, and rejects any other value; and a property onmessage, which the page sets to a function. The application delivers data to the page by calling that function with an object {data: value}, where value is an ArrayBuffer for binary data and a string for text. Each carrier needs its own web view
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebProxyCarrier {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the carrier
  id: i64,
  /// Identifier of the added proxy that uses the carrier; 0 if the carrier is used by pingProxy or testProxy
  proxy_id: i64,
  /// The HTTPS URL to load. The URL contains the capability for the proxy and a fragment, which must be kept; the URL must never be logged or shown to the user
  url: String,
  /// Origin of the relay page, for example, "https://proxy.example.com". The web view must not be allowed to navigate to another origin
  origin: String,
  /// The only path prefix on carrier.origin from which the page may load resources or to which it may send requests, for example, "/" or "/relay/"
  path_prefix: String,
  /// Name of the object to define in the top-level document; currently, always "TelegramWebProxy"
  bridge_object_name: String,
  /// True, if the carrier is needed only to test the proxy and will be closed after the test. The application may refuse to open such carriers, for example, to avoid creating a web view for every saved proxy
  is_temporary: bool,
}

impl RObject for WebProxyCarrier {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webProxyCarrier"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebProxyCarrier {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebProxyCarrierBuilder {
    let inner = WebProxyCarrier {
      td_name: "webProxyCarrier".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebProxyCarrierBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn proxy_id(&self) -> i64 {
    self.proxy_id
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn origin(&self) -> &String {
    &self.origin
  }

  pub fn path_prefix(&self) -> &String {
    &self.path_prefix
  }

  pub fn bridge_object_name(&self) -> &String {
    &self.bridge_object_name
  }

  pub fn is_temporary(&self) -> bool {
    self.is_temporary
  }
}

#[doc(hidden)]
pub struct RTDWebProxyCarrierBuilder {
  inner: WebProxyCarrier,
}

impl RTDWebProxyCarrierBuilder {
  pub fn build(&self) -> WebProxyCarrier {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn proxy_id(&mut self, proxy_id: i64) -> &mut Self {
    self.inner.proxy_id = proxy_id;
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn origin<T: AsRef<str>>(&mut self, origin: T) -> &mut Self {
    self.inner.origin = origin.as_ref().to_string();
    self
  }

  pub fn path_prefix<T: AsRef<str>>(&mut self, path_prefix: T) -> &mut Self {
    self.inner.path_prefix = path_prefix.as_ref().to_string();
    self
  }

  pub fn bridge_object_name<T: AsRef<str>>(&mut self, bridge_object_name: T) -> &mut Self {
    self.inner.bridge_object_name = bridge_object_name.as_ref().to_string();
    self
  }

  pub fn is_temporary(&mut self, is_temporary: bool) -> &mut Self {
    self.inner.is_temporary = is_temporary;
    self
  }
}

impl AsRef<WebProxyCarrier> for WebProxyCarrier {
  fn as_ref(&self) -> &WebProxyCarrier {
    self
  }
}

impl AsRef<WebProxyCarrier> for RTDWebProxyCarrierBuilder {
  fn as_ref(&self) -> &WebProxyCarrier {
    &self.inner
  }
}
