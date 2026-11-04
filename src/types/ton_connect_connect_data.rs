use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains data required to give access to the TON wallet of the current user in acceptOauthRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectConnectData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the session
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  session_id: isize,
  /// Answer to the challenge received from setTonConnectSessionWalletClientId
  challenge_answer: String,
  /// Body of the result of the connection request as in sendTonConnectSessionConnectResult
  body: String,
  /// Proof of the TON wallet ownership for the payload oauthLinkInfo.ton_proof_payload
  proof: TonWalletOwnershipProof,
}

impl RObject for TonConnectConnectData {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectConnectData"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectConnectData {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectConnectDataBuilder {
    let inner = TonConnectConnectData {
      td_name: "tonConnectConnectData".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectConnectDataBuilder { inner }
  }

  pub fn session_id(&self) -> isize {
    self.session_id
  }

  pub fn challenge_answer(&self) -> &String {
    &self.challenge_answer
  }

  pub fn body(&self) -> &String {
    &self.body
  }

  pub fn proof(&self) -> &TonWalletOwnershipProof {
    &self.proof
  }
}

#[doc(hidden)]
pub struct RTDTonConnectConnectDataBuilder {
  inner: TonConnectConnectData,
}

impl RTDTonConnectConnectDataBuilder {
  pub fn build(&self) -> TonConnectConnectData {
    self.inner.clone()
  }

  pub fn session_id(&mut self, session_id: isize) -> &mut Self {
    self.inner.session_id = session_id;
    self
  }

  pub fn challenge_answer<T: AsRef<str>>(&mut self, challenge_answer: T) -> &mut Self {
    self.inner.challenge_answer = challenge_answer.as_ref().to_string();
    self
  }

  pub fn body<T: AsRef<str>>(&mut self, body: T) -> &mut Self {
    self.inner.body = body.as_ref().to_string();
    self
  }

  pub fn proof<T: AsRef<TonWalletOwnershipProof>>(&mut self, proof: T) -> &mut Self {
    self.inner.proof = proof.as_ref().clone();
    self
  }
}

impl AsRef<TonConnectConnectData> for TonConnectConnectData {
  fn as_ref(&self) -> &TonConnectConnectData {
    self
  }
}

impl AsRef<TonConnectConnectData> for RTDTonConnectConnectDataBuilder {
  fn as_ref(&self) -> &TonConnectConnectData {
    &self.inner
  }
}
