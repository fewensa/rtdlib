use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains statistics about interactions with a message sent in the chat or a story posted on behalf of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsInteractionInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the object
  object_type: ChatStatisticsObjectType,
  /// Number of times the object was viewed
  view_count: i64,
  /// Number of times the object was forwarded
  forward_count: i64,
  /// Number of times reactions were added to the object
  reaction_count: i64,
}

impl RObject for ChatStatisticsInteractionInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatStatisticsInteractionInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatStatisticsInteractionInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatStatisticsInteractionInfoBuilder {
    let inner = ChatStatisticsInteractionInfo {
      td_name: "chatStatisticsInteractionInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatStatisticsInteractionInfoBuilder { inner }
  }

  pub fn object_type(&self) -> &ChatStatisticsObjectType {
    &self.object_type
  }

  pub fn view_count(&self) -> i64 {
    self.view_count
  }

  pub fn forward_count(&self) -> i64 {
    self.forward_count
  }

  pub fn reaction_count(&self) -> i64 {
    self.reaction_count
  }
}

#[doc(hidden)]
pub struct RTDChatStatisticsInteractionInfoBuilder {
  inner: ChatStatisticsInteractionInfo,
}

impl RTDChatStatisticsInteractionInfoBuilder {
  pub fn build(&self) -> ChatStatisticsInteractionInfo {
    self.inner.clone()
  }

  pub fn object_type<T: AsRef<ChatStatisticsObjectType>>(&mut self, object_type: T) -> &mut Self {
    self.inner.object_type = object_type.as_ref().clone();
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

  pub fn reaction_count(&mut self, reaction_count: i64) -> &mut Self {
    self.inner.reaction_count = reaction_count;
    self
  }
}

impl AsRef<ChatStatisticsInteractionInfo> for ChatStatisticsInteractionInfo {
  fn as_ref(&self) -> &ChatStatisticsInteractionInfo {
    self
  }
}

impl AsRef<ChatStatisticsInteractionInfo> for RTDChatStatisticsInteractionInfoBuilder {
  fn as_ref(&self) -> &ChatStatisticsInteractionInfo {
    &self.inner
  }
}
