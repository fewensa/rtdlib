use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a challenge for TON wallet ownership proof
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletOwnershipProofChallenge {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Payload of the challenge
  payload: String,
  /// Domain of the proof requester
  domain: String,
}

impl RObject for TonWalletOwnershipProofChallenge {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletOwnershipProofChallenge"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletOwnershipProofChallenge {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletOwnershipProofChallengeBuilder {
    let inner = TonWalletOwnershipProofChallenge {
      td_name: "tonWalletOwnershipProofChallenge".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletOwnershipProofChallengeBuilder { inner }
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }

  pub fn domain(&self) -> &String {
    &self.domain
  }
}

#[doc(hidden)]
pub struct RTDTonWalletOwnershipProofChallengeBuilder {
  inner: TonWalletOwnershipProofChallenge,
}

impl RTDTonWalletOwnershipProofChallengeBuilder {
  pub fn build(&self) -> TonWalletOwnershipProofChallenge {
    self.inner.clone()
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

  pub fn domain<T: AsRef<str>>(&mut self, domain: T) -> &mut Self {
    self.inner.domain = domain.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletOwnershipProofChallenge> for TonWalletOwnershipProofChallenge {
  fn as_ref(&self) -> &TonWalletOwnershipProofChallenge {
    self
  }
}

impl AsRef<TonWalletOwnershipProofChallenge> for RTDTonWalletOwnershipProofChallengeBuilder {
  fn as_ref(&self) -> &TonWalletOwnershipProofChallenge {
    &self.inner
  }
}
