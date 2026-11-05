use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a ready to send inline message. Use sendInlineQueryResultMessage to send the message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreparedInlineMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the inline query to pass to sendInlineQueryResultMessage
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  inline_query_id: isize,
  /// Resulted inline message of the query
  result: InlineQueryResult,
  /// Types of the chats to which the message can be sent
  chat_types: TargetChatTypes,
}

impl RObject for PreparedInlineMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "preparedInlineMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PreparedInlineMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPreparedInlineMessageBuilder {
    let inner = PreparedInlineMessage {
      td_name: "preparedInlineMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPreparedInlineMessageBuilder { inner }
  }

  pub fn inline_query_id(&self) -> isize {
    self.inline_query_id
  }

  pub fn result(&self) -> &InlineQueryResult {
    &self.result
  }

  pub fn chat_types(&self) -> &TargetChatTypes {
    &self.chat_types
  }
}

#[doc(hidden)]
pub struct RTDPreparedInlineMessageBuilder {
  inner: PreparedInlineMessage,
}

impl RTDPreparedInlineMessageBuilder {
  pub fn build(&self) -> PreparedInlineMessage {
    self.inner.clone()
  }

  pub fn inline_query_id(&mut self, inline_query_id: isize) -> &mut Self {
    self.inner.inline_query_id = inline_query_id;
    self
  }

  pub fn result<T: AsRef<InlineQueryResult>>(&mut self, result: T) -> &mut Self {
    self.inner.result = result.as_ref().clone();
    self
  }

  pub fn chat_types<T: AsRef<TargetChatTypes>>(&mut self, chat_types: T) -> &mut Self {
    self.inner.chat_types = chat_types.as_ref().clone();
    self
  }
}

impl AsRef<PreparedInlineMessage> for PreparedInlineMessage {
  fn as_ref(&self) -> &PreparedInlineMessage {
    self
  }
}

impl AsRef<PreparedInlineMessage> for RTDPreparedInlineMessageBuilder {
  fn as_ref(&self) -> &PreparedInlineMessage {
    &self.inner
  }
}
