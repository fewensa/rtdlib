use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents identifier of an event in a TON Connect session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectSessionEventId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The event identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  event_id: isize,
}

impl RObject for TonConnectSessionEventId {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectSessionEventId"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectSessionEventId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectSessionEventIdBuilder {
    let inner = TonConnectSessionEventId {
      td_name: "tonConnectSessionEventId".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectSessionEventIdBuilder { inner }
  }

  pub fn event_id(&self) -> isize {
    self.event_id
  }
}

#[doc(hidden)]
pub struct RTDTonConnectSessionEventIdBuilder {
  inner: TonConnectSessionEventId,
}

impl RTDTonConnectSessionEventIdBuilder {
  pub fn build(&self) -> TonConnectSessionEventId {
    self.inner.clone()
  }

  pub fn event_id(&mut self, event_id: isize) -> &mut Self {
    self.inner.event_id = event_id;
    self
  }
}

impl AsRef<TonConnectSessionEventId> for TonConnectSessionEventId {
  fn as_ref(&self) -> &TonConnectSessionEventId {
    self
  }
}

impl AsRef<TonConnectSessionEventId> for RTDTonConnectSessionEventIdBuilder {
  fn as_ref(&self) -> &TonConnectSessionEventId {
    &self.inner
  }
}
