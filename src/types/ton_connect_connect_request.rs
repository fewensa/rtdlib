use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a request to connect the user's TON wallet to a dApp using TON Connect
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectConnectRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the manifest of the dApp
  manifest_url: String,
  /// Items requested from the wallet
  items: Vec<TonConnectConnectItem>,
}

impl RObject for TonConnectConnectRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectConnectRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectConnectRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectConnectRequestBuilder {
    let inner = TonConnectConnectRequest {
      td_name: "tonConnectConnectRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectConnectRequestBuilder { inner }
  }

  pub fn manifest_url(&self) -> &String {
    &self.manifest_url
  }

  pub fn items(&self) -> &Vec<TonConnectConnectItem> {
    &self.items
  }
}

#[doc(hidden)]
pub struct RTDTonConnectConnectRequestBuilder {
  inner: TonConnectConnectRequest,
}

impl RTDTonConnectConnectRequestBuilder {
  pub fn build(&self) -> TonConnectConnectRequest {
    self.inner.clone()
  }

  pub fn manifest_url<T: AsRef<str>>(&mut self, manifest_url: T) -> &mut Self {
    self.inner.manifest_url = manifest_url.as_ref().to_string();
    self
  }

  pub fn items(&mut self, items: Vec<TonConnectConnectItem>) -> &mut Self {
    self.inner.items = items;
    self
  }
}

impl AsRef<TonConnectConnectRequest> for TonConnectConnectRequest {
  fn as_ref(&self) -> &TonConnectConnectRequest {
    self
  }
}

impl AsRef<TonConnectConnectRequest> for RTDTonConnectConnectRequestBuilder {
  fn as_ref(&self) -> &TonConnectConnectRequest {
    &self.inner
  }
}
