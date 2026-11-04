use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents an inline message that can be sent via the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreparedInlineMessageId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier for the message
  id: String,
  /// Point in time (Unix timestamp) when the message can't be used anymore
  expiration_date: i64,
}

impl RObject for PreparedInlineMessageId {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "preparedInlineMessageId"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PreparedInlineMessageId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPreparedInlineMessageIdBuilder {
    let inner = PreparedInlineMessageId {
      td_name: "preparedInlineMessageId".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPreparedInlineMessageIdBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }
}

#[doc(hidden)]
pub struct RTDPreparedInlineMessageIdBuilder {
  inner: PreparedInlineMessageId,
}

impl RTDPreparedInlineMessageIdBuilder {
  pub fn build(&self) -> PreparedInlineMessageId {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }
}

impl AsRef<PreparedInlineMessageId> for PreparedInlineMessageId {
  fn as_ref(&self) -> &PreparedInlineMessageId {
    self
  }
}

impl AsRef<PreparedInlineMessageId> for RTDPreparedInlineMessageIdBuilder {
  fn as_ref(&self) -> &PreparedInlineMessageId {
    &self.inner
  }
}
