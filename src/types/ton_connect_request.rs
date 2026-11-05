use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a pending TON Connect request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonConnectRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the session
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  session_id: isize,
  /// Unique identifier of the service message in the chat 777000 with the request
  message_id: i64,
  /// Body of the request
  body: String,
  /// Point in time (Unix timestamp) when the request will expire; 0 if never
  expiration_date: i64,
  /// Topic of the request; may be empty if none
  topic: String,
  /// Identifier to trace the request; may be empty if none
  trace_id: String,
}

impl RObject for TonConnectRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonConnectRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonConnectRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonConnectRequestBuilder {
    let inner = TonConnectRequest {
      td_name: "tonConnectRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonConnectRequestBuilder { inner }
  }

  pub fn session_id(&self) -> isize {
    self.session_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn body(&self) -> &String {
    &self.body
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }

  pub fn topic(&self) -> &String {
    &self.topic
  }

  pub fn trace_id(&self) -> &String {
    &self.trace_id
  }
}

#[doc(hidden)]
pub struct RTDTonConnectRequestBuilder {
  inner: TonConnectRequest,
}

impl RTDTonConnectRequestBuilder {
  pub fn build(&self) -> TonConnectRequest {
    self.inner.clone()
  }

  pub fn session_id(&mut self, session_id: isize) -> &mut Self {
    self.inner.session_id = session_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn body<T: AsRef<str>>(&mut self, body: T) -> &mut Self {
    self.inner.body = body.as_ref().to_string();
    self
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }

  pub fn topic<T: AsRef<str>>(&mut self, topic: T) -> &mut Self {
    self.inner.topic = topic.as_ref().to_string();
    self
  }

  pub fn trace_id<T: AsRef<str>>(&mut self, trace_id: T) -> &mut Self {
    self.inner.trace_id = trace_id.as_ref().to_string();
    self
  }
}

impl AsRef<TonConnectRequest> for TonConnectRequest {
  fn as_ref(&self) -> &TonConnectRequest {
    self
  }
}

impl AsRef<TonConnectRequest> for RTDTonConnectRequestBuilder {
  fn as_ref(&self) -> &TonConnectRequest {
    &self.inner
  }
}
