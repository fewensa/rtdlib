use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a proof of TON wallet ownership
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletOwnershipProof {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Public key of the wallet
  public_key: String,
  /// The current UNIX timestamp which was used to create the proof
  timestamp: i64,
  /// The signature proving the ownership
  signature: String,
}

impl RObject for TonWalletOwnershipProof {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletOwnershipProof"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletOwnershipProof {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletOwnershipProofBuilder {
    let inner = TonWalletOwnershipProof {
      td_name: "tonWalletOwnershipProof".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletOwnershipProofBuilder { inner }
  }

  pub fn public_key(&self) -> &String {
    &self.public_key
  }

  pub fn timestamp(&self) -> i64 {
    self.timestamp
  }

  pub fn signature(&self) -> &String {
    &self.signature
  }
}

#[doc(hidden)]
pub struct RTDTonWalletOwnershipProofBuilder {
  inner: TonWalletOwnershipProof,
}

impl RTDTonWalletOwnershipProofBuilder {
  pub fn build(&self) -> TonWalletOwnershipProof {
    self.inner.clone()
  }

  pub fn public_key<T: AsRef<str>>(&mut self, public_key: T) -> &mut Self {
    self.inner.public_key = public_key.as_ref().to_string();
    self
  }

  pub fn timestamp(&mut self, timestamp: i64) -> &mut Self {
    self.inner.timestamp = timestamp;
    self
  }

  pub fn signature<T: AsRef<str>>(&mut self, signature: T) -> &mut Self {
    self.inner.signature = signature.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletOwnershipProof> for TonWalletOwnershipProof {
  fn as_ref(&self) -> &TonWalletOwnershipProof {
    self
  }
}

impl AsRef<TonWalletOwnershipProof> for RTDTonWalletOwnershipProofBuilder {
  fn as_ref(&self) -> &TonWalletOwnershipProof {
    &self.inner
  }
}
