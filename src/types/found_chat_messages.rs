use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of messages found by a search in a given chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundChatMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of messages found; 1 if unknown
  total_count: i64,
  /// List of messages
  messages: Vec<Message>,
  /// The offset for the next request. If 0, there are no more results
  next_from_message_id: i64,
}

impl RObject for FoundChatMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundChatMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundChatMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundChatMessagesBuilder {
    let inner = FoundChatMessages {
      td_name: "foundChatMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundChatMessagesBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn messages(&self) -> &Vec<Message> {
    &self.messages
  }

  pub fn next_from_message_id(&self) -> i64 {
    self.next_from_message_id
  }
}

#[doc(hidden)]
pub struct RTDFoundChatMessagesBuilder {
  inner: FoundChatMessages,
}

impl RTDFoundChatMessagesBuilder {
  pub fn build(&self) -> FoundChatMessages {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn messages(&mut self, messages: Vec<Message>) -> &mut Self {
    self.inner.messages = messages;
    self
  }

  pub fn next_from_message_id(&mut self, next_from_message_id: i64) -> &mut Self {
    self.inner.next_from_message_id = next_from_message_id;
    self
  }
}

impl AsRef<FoundChatMessages> for FoundChatMessages {
  fn as_ref(&self) -> &FoundChatMessages {
    self
  }
}

impl AsRef<FoundChatMessages> for RTDFoundChatMessagesBuilder {
  fn as_ref(&self) -> &FoundChatMessages {
    &self.inner
  }
}
