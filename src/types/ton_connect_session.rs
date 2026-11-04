use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a TON Connect session. See https://github.com/ton-blockchain/ton-connect/tree/main/spec for full TON Connect protocol documentation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectSession {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the session
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Hex-encoded public X25519 key of the dApp
  dapp_client_id: String,
  /// Hex-encoded public X25519 key of the current user's application; may be empty if none
  wallet_client_id: String,
  /// Point in time (Unix timestamp) when the session was created
  creation_date: i64,
  /// Manifest of the dApp
  manifest: TonConnectManifest,
  /// Nonce used for the session
  nonce: String,
  /// Session state
  state: TonConnectSessionState,
}

impl RObject for TonConnectSession {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectSession"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectSession {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectSessionBuilder {
    let inner = TonConnectSession {
      td_name: "tonConnectSession".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectSessionBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn dapp_client_id(&self) -> &String {
    &self.dapp_client_id
  }

  pub fn wallet_client_id(&self) -> &String {
    &self.wallet_client_id
  }

  pub fn creation_date(&self) -> i64 {
    self.creation_date
  }

  pub fn manifest(&self) -> &TonConnectManifest {
    &self.manifest
  }

  pub fn nonce(&self) -> &String {
    &self.nonce
  }

  pub fn state(&self) -> &TonConnectSessionState {
    &self.state
  }
}

#[doc(hidden)]
pub struct RTDTonConnectSessionBuilder {
  inner: TonConnectSession,
}

impl RTDTonConnectSessionBuilder {
  pub fn build(&self) -> TonConnectSession {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn dapp_client_id<T: AsRef<str>>(&mut self, dapp_client_id: T) -> &mut Self {
    self.inner.dapp_client_id = dapp_client_id.as_ref().to_string();
    self
  }

  pub fn wallet_client_id<T: AsRef<str>>(&mut self, wallet_client_id: T) -> &mut Self {
    self.inner.wallet_client_id = wallet_client_id.as_ref().to_string();
    self
  }

  pub fn creation_date(&mut self, creation_date: i64) -> &mut Self {
    self.inner.creation_date = creation_date;
    self
  }

  pub fn manifest<T: AsRef<TonConnectManifest>>(&mut self, manifest: T) -> &mut Self {
    self.inner.manifest = manifest.as_ref().clone();
    self
  }

  pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
    self.inner.nonce = nonce.as_ref().to_string();
    self
  }

  pub fn state<T: AsRef<TonConnectSessionState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }
}

impl AsRef<TonConnectSession> for TonConnectSession {
  fn as_ref(&self) -> &TonConnectSession {
    self
  }
}

impl AsRef<TonConnectSession> for RTDTonConnectSessionBuilder {
  fn as_ref(&self) -> &TonConnectSession {
    &self.inner
  }
}
