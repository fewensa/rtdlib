
use crate::types::*;
use crate::errors::*;




/// chatStatisticsMessageInteractionCounters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsMessageInteractionCounters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// message_id
  message_id: i64,
  /// views
  views: i64,
  /// forwards
  forwards: i64,
  
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

  pub fn views(&self) -> i64 { self.views }

  pub fn forwards(&self) -> i64 { self.forwards }

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

   
  pub fn views(&mut self, views: i64) -> &mut Self {
    self.inner.views = views;
    self
  }

   
  pub fn forwards(&mut self, forwards: i64) -> &mut Self {
    self.inner.forwards = forwards;
    self
  }

}

impl AsRef<ChatStatisticsMessageInteractionCounters> for ChatStatisticsMessageInteractionCounters {
  fn as_ref(&self) -> &ChatStatisticsMessageInteractionCounters { self }
}

impl AsRef<ChatStatisticsMessageInteractionCounters> for RTDChatStatisticsMessageInteractionCountersBuilder {
  fn as_ref(&self) -> &ChatStatisticsMessageInteractionCounters { &self.inner }
}



