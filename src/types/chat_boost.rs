use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a boost applied to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the boost
  id: String,
  /// The number of identical boosts applied
  count: i64,
  /// Source of the boost
  source: ChatBoostSource,
  /// Point in time (Unix timestamp) when the chat was boosted
  start_date: i64,
  /// Point in time (Unix timestamp) when the boost will expire
  expiration_date: i64,
}

impl RObject for ChatBoost {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoost"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostBuilder {
    let inner = ChatBoost {
      td_name: "chatBoost".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn count(&self) -> i64 {
    self.count
  }

  pub fn source(&self) -> &ChatBoostSource {
    &self.source
  }

  pub fn start_date(&self) -> i64 {
    self.start_date
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }
}

#[doc(hidden)]
pub struct RTDChatBoostBuilder {
  inner: ChatBoost,
}

impl RTDChatBoostBuilder {
  pub fn build(&self) -> ChatBoost {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn count(&mut self, count: i64) -> &mut Self {
    self.inner.count = count;
    self
  }

  pub fn source<T: AsRef<ChatBoostSource>>(&mut self, source: T) -> &mut Self {
    self.inner.source = source.as_ref().clone();
    self
  }

  pub fn start_date(&mut self, start_date: i64) -> &mut Self {
    self.inner.start_date = start_date;
    self
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }
}

impl AsRef<ChatBoost> for ChatBoost {
  fn as_ref(&self) -> &ChatBoost {
    self
  }
}

impl AsRef<ChatBoost> for RTDChatBoostBuilder {
  fn as_ref(&self) -> &ChatBoost {
    &self.inner
  }
}
