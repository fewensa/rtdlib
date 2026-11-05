use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a TON Connect challenge
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectChallenge {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// 104-byte challenge
  challenge: String,
  /// Unique identifier of the first outgoing event
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  event_id: isize,
}

impl RObject for TonConnectChallenge {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectChallenge"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectChallenge {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectChallengeBuilder {
    let inner = TonConnectChallenge {
      td_name: "tonConnectChallenge".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectChallengeBuilder { inner }
  }

  pub fn challenge(&self) -> &String {
    &self.challenge
  }

  pub fn event_id(&self) -> isize {
    self.event_id
  }
}

#[doc(hidden)]
pub struct RTDTonConnectChallengeBuilder {
  inner: TonConnectChallenge,
}

impl RTDTonConnectChallengeBuilder {
  pub fn build(&self) -> TonConnectChallenge {
    self.inner.clone()
  }

  pub fn challenge<T: AsRef<str>>(&mut self, challenge: T) -> &mut Self {
    self.inner.challenge = challenge.as_ref().to_string();
    self
  }

  pub fn event_id(&mut self, event_id: isize) -> &mut Self {
    self.inner.event_id = event_id;
    self
  }
}

impl AsRef<TonConnectChallenge> for TonConnectChallenge {
  fn as_ref(&self) -> &TonConnectChallenge {
    self
  }
}

impl AsRef<TonConnectChallenge> for RTDTonConnectChallengeBuilder {
  fn as_ref(&self) -> &TonConnectChallenge {
    &self.inner
  }
}
