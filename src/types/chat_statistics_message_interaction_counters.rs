
use crate::types::*;
use crate::errors::*;




/// Contains statistics about interactions with a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsMessageInteractionCounters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Message identifier
  message_id: i64,
  /// Number of times the message was viewed
  view_count: i64,
  /// Number of times the message was forwarded
  forward_count: i64,
  
}

impl RObject for ChatStatisticsMessageInteractionCounters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsMessageInteractionCounters" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatisticsMessageInteractionCounters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsMessageInteractionCountersBuilder {
    let mut inner = ChatStatisticsMessageInteractionCounters::default();
    inner.td_name = "chatStatisticsMessageInteractionCounters".to_string();
    RTDChatStatisticsMessageInteractionCountersBuilder { inner }
  }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn view_count(&self) -> i64 { self.view_count }

  pub fn forward_count(&self) -> i64 { self.forward_count }

}

#[doc(hidden)]
pub struct RTDChatStatisticsMessageInteractionCountersBuilder {
  inner: ChatStatisticsMessageInteractionCounters
}

impl RTDChatStatisticsMessageInteractionCountersBuilder {
  pub fn build(&self) -> ChatStatisticsMessageInteractionCounters { self.inner.clone() }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn view_count(&mut self, view_count: i64) -> &mut Self {
    self.inner.view_count = view_count;
    self
  }

   
  pub fn forward_count(&mut self, forward_count: i64) -> &mut Self {
    self.inner.forward_count = forward_count;
    self
  }

}

impl AsRef<ChatStatisticsMessageInteractionCounters> for ChatStatisticsMessageInteractionCounters {
  fn as_ref(&self) -> &ChatStatisticsMessageInteractionCounters { self }
}

impl AsRef<ChatStatisticsMessageInteractionCounters> for RTDChatStatisticsMessageInteractionCountersBuilder {
  fn as_ref(&self) -> &ChatStatisticsMessageInteractionCounters { &self.inner }
}



