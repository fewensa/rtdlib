
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a list of message senders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSenders {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total count of messages senders found
  total_count: i64,
  /// List of message senders
  senders: Vec<MessageSender>,
  
}

impl RObject for MessageSenders {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSenders" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessageSenders {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSendersBuilder {
    let mut inner = MessageSenders::default();
    inner.td_name = "messageSenders".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageSendersBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn senders(&self) -> &Vec<MessageSender> { &self.senders }

}

#[doc(hidden)]
pub struct RTDMessageSendersBuilder {
  inner: MessageSenders
}

impl RTDMessageSendersBuilder {
  pub fn build(&self) -> MessageSenders { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn senders(&mut self, senders: Vec<MessageSender>) -> &mut Self {
    self.inner.senders = senders;
    self
  }

}

impl AsRef<MessageSenders> for MessageSenders {
  fn as_ref(&self) -> &MessageSenders { self }
}

impl AsRef<MessageSenders> for RTDMessageSendersBuilder {
  fn as_ref(&self) -> &MessageSenders { &self.inner }
}



