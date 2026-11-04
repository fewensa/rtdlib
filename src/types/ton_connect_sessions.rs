use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of TON Connect sessions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectSessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of sessions
  sessions: Vec<TonConnectSession>,
}

impl RObject for TonConnectSessions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectSessions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectSessions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectSessionsBuilder {
    let inner = TonConnectSessions {
      td_name: "tonConnectSessions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectSessionsBuilder { inner }
  }

  pub fn sessions(&self) -> &Vec<TonConnectSession> {
    &self.sessions
  }
}

#[doc(hidden)]
pub struct RTDTonConnectSessionsBuilder {
  inner: TonConnectSessions,
}

impl RTDTonConnectSessionsBuilder {
  pub fn build(&self) -> TonConnectSessions {
    self.inner.clone()
  }

  pub fn sessions(&mut self, sessions: Vec<TonConnectSession>) -> &mut Self {
    self.inner.sessions = sessions;
    self
  }
}

impl AsRef<TonConnectSessions> for TonConnectSessions {
  fn as_ref(&self) -> &TonConnectSessions {
    self
  }
}

impl AsRef<TonConnectSessions> for RTDTonConnectSessionsBuilder {
  fn as_ref(&self) -> &TonConnectSessions {
    &self.inner
  }
}
