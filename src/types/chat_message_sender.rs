use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a message sender, which can be used to send messages in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMessageSender {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The message sender
  sender: MessageSender,
  /// True, if Telegram Premium is needed to use the message sender
  needs_premium: bool,
}

impl RObject for ChatMessageSender {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMessageSender"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatMessageSender {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMessageSenderBuilder {
    let inner = ChatMessageSender {
      td_name: "chatMessageSender".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatMessageSenderBuilder { inner }
  }

  pub fn sender(&self) -> &MessageSender {
    &self.sender
  }

  pub fn needs_premium(&self) -> bool {
    self.needs_premium
  }
}

#[doc(hidden)]
pub struct RTDChatMessageSenderBuilder {
  inner: ChatMessageSender,
}

impl RTDChatMessageSenderBuilder {
  pub fn build(&self) -> ChatMessageSender {
    self.inner.clone()
  }

  pub fn sender<T: AsRef<MessageSender>>(&mut self, sender: T) -> &mut Self {
    self.inner.sender = sender.as_ref().clone();
    self
  }

  pub fn needs_premium(&mut self, needs_premium: bool) -> &mut Self {
    self.inner.needs_premium = needs_premium;
    self
  }
}

impl AsRef<ChatMessageSender> for ChatMessageSender {
  fn as_ref(&self) -> &ChatMessageSender {
    self
  }
}

impl AsRef<ChatMessageSender> for RTDChatMessageSenderBuilder {
  fn as_ref(&self) -> &ChatMessageSender {
    &self.inner
  }
}
