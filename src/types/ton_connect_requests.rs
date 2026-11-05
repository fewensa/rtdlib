use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a session with its pending TON Connect requests
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectRequests {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The session
  session: TonConnectSession,
  /// The requests
  requests: Vec<TonConnectRequest>,
}

impl RObject for TonConnectRequests {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectRequests"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectRequests {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectRequestsBuilder {
    let inner = TonConnectRequests {
      td_name: "tonConnectRequests".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectRequestsBuilder { inner }
  }

  pub fn session(&self) -> &TonConnectSession {
    &self.session
  }

  pub fn requests(&self) -> &Vec<TonConnectRequest> {
    &self.requests
  }
}

#[doc(hidden)]
pub struct RTDTonConnectRequestsBuilder {
  inner: TonConnectRequests,
}

impl RTDTonConnectRequestsBuilder {
  pub fn build(&self) -> TonConnectRequests {
    self.inner.clone()
  }

  pub fn session<T: AsRef<TonConnectSession>>(&mut self, session: T) -> &mut Self {
    self.inner.session = session.as_ref().clone();
    self
  }

  pub fn requests(&mut self, requests: Vec<TonConnectRequest>) -> &mut Self {
    self.inner.requests = requests;
    self
  }
}

impl AsRef<TonConnectRequests> for TonConnectRequests {
  fn as_ref(&self) -> &TonConnectRequests {
    self
  }
}

impl AsRef<TonConnectRequests> for RTDTonConnectRequestsBuilder {
  fn as_ref(&self) -> &TonConnectRequests {
    &self.inner
  }
}
