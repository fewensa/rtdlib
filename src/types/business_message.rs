use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a message from a business account as received by a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The message
  message: Message,
  /// Message that is replied by the message in the same chat; may be null if none
  reply_to_message: Option<Message>,
}

impl RObject for BusinessMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessMessageBuilder {
    let inner = BusinessMessage {
      td_name: "businessMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessMessageBuilder { inner }
  }

  pub fn message(&self) -> &Message {
    &self.message
  }

  pub fn reply_to_message(&self) -> &Option<Message> {
    &self.reply_to_message
  }
}

#[doc(hidden)]
pub struct RTDBusinessMessageBuilder {
  inner: BusinessMessage,
}

impl RTDBusinessMessageBuilder {
  pub fn build(&self) -> BusinessMessage {
    self.inner.clone()
  }

  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

  pub fn reply_to_message<T: AsRef<Message>>(&mut self, reply_to_message: T) -> &mut Self {
    self.inner.reply_to_message = Some(reply_to_message.as_ref().clone());
    self
  }
}

impl AsRef<BusinessMessage> for BusinessMessage {
  fn as_ref(&self) -> &BusinessMessage {
    self
  }
}

impl AsRef<BusinessMessage> for RTDBusinessMessageBuilder {
  fn as_ref(&self) -> &BusinessMessage {
    &self.inner
  }
}
