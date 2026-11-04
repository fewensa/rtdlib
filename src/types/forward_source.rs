use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about the last message from which a new message was forwarded last time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForwardSource {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to which the message that was forwarded belonged; may be 0 if unknown
  chat_id: i64,
  /// Identifier of the message; may be 0 if unknown
  message_id: i64,
  /// Identifier of the sender of the message; may be null if unknown or the new message was forwarded not to Saved Messages
  sender_id: Option<MessageSender>,
  /// Name of the sender of the message if the sender is hidden by their privacy settings
  sender_name: String,
  /// Point in time (Unix timestamp) when the message is sent; 0 if unknown
  date: i64,
  /// True, if the message that was forwarded is outgoing; always false if sender is unknown
  is_outgoing: bool,
}

impl RObject for ForwardSource {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "forwardSource"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ForwardSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDForwardSourceBuilder {
    let inner = ForwardSource {
      td_name: "forwardSource".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDForwardSourceBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn sender_id(&self) -> &Option<MessageSender> {
    &self.sender_id
  }

  pub fn sender_name(&self) -> &String {
    &self.sender_name
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn is_outgoing(&self) -> bool {
    self.is_outgoing
  }
}

#[doc(hidden)]
pub struct RTDForwardSourceBuilder {
  inner: ForwardSource,
}

impl RTDForwardSourceBuilder {
  pub fn build(&self) -> ForwardSource {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = Some(sender_id.as_ref().clone());
    self
  }

  pub fn sender_name<T: AsRef<str>>(&mut self, sender_name: T) -> &mut Self {
    self.inner.sender_name = sender_name.as_ref().to_string();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
    self.inner.is_outgoing = is_outgoing;
    self
  }
}

impl AsRef<ForwardSource> for ForwardSource {
  fn as_ref(&self) -> &ForwardSource {
    self
  }
}

impl AsRef<ForwardSource> for RTDForwardSourceBuilder {
  fn as_ref(&self) -> &ForwardSource {
    &self.inner
  }
}
