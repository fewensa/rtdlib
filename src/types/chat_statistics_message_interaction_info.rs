
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains statistics about interactions with a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsMessageInteractionInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message identifier
  message_id: i64,
  /// Number of times the message was viewed
  view_count: i64,
  /// Number of times the message was forwarded
  forward_count: i64,
  
}

impl RObject for ChatStatisticsMessageInteractionInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsMessageInteractionInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatisticsMessageInteractionInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsMessageInteractionInfoBuilder {
    let mut inner = ChatStatisticsMessageInteractionInfo::default();
    inner.td_name = "chatStatisticsMessageInteractionInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatStatisticsMessageInteractionInfoBuilder { inner }
  }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn view_count(&self) -> i64 { self.view_count }

  pub fn forward_count(&self) -> i64 { self.forward_count }

}

#[doc(hidden)]
pub struct RTDChatStatisticsMessageInteractionInfoBuilder {
  inner: ChatStatisticsMessageInteractionInfo
}

impl RTDChatStatisticsMessageInteractionInfoBuilder {
  pub fn build(&self) -> ChatStatisticsMessageInteractionInfo { self.inner.clone() }

   
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

impl AsRef<ChatStatisticsMessageInteractionInfo> for ChatStatisticsMessageInteractionInfo {
  fn as_ref(&self) -> &ChatStatisticsMessageInteractionInfo { self }
}

impl AsRef<ChatStatisticsMessageInteractionInfo> for RTDChatStatisticsMessageInteractionInfoBuilder {
  fn as_ref(&self) -> &ChatStatisticsMessageInteractionInfo { &self.inner }
}



