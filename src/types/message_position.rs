
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a message in a specific position
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePosition {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// 0-based message position in the full list of suitable messages
  position: i64,
  /// Message identifier
  message_id: i64,
  /// Point in time (Unix timestamp) when the message was sent
  date: i64,
  
}

impl RObject for MessagePosition {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePosition" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessagePosition {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePositionBuilder {
    let mut inner = MessagePosition::default();
    inner.td_name = "messagePosition".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessagePositionBuilder { inner }
  }

  pub fn position(&self) -> i64 { self.position }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn date(&self) -> i64 { self.date }

}

#[doc(hidden)]
pub struct RTDMessagePositionBuilder {
  inner: MessagePosition
}

impl RTDMessagePositionBuilder {
  pub fn build(&self) -> MessagePosition { self.inner.clone() }

   
  pub fn position(&mut self, position: i64) -> &mut Self {
    self.inner.position = position;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

}

impl AsRef<MessagePosition> for MessagePosition {
  fn as_ref(&self) -> &MessagePosition { self }
}

impl AsRef<MessagePosition> for RTDMessagePositionBuilder {
  fn as_ref(&self) -> &MessagePosition { &self.inner }
}



