use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of message senders, which can be used to send messages in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMessageSenders {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of available message senders
  senders: Vec<ChatMessageSender>,
}

impl RObject for ChatMessageSenders {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMessageSenders"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatMessageSenders {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMessageSendersBuilder {
    let inner = ChatMessageSenders {
      td_name: "chatMessageSenders".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatMessageSendersBuilder { inner }
  }

  pub fn senders(&self) -> &Vec<ChatMessageSender> {
    &self.senders
  }
}

#[doc(hidden)]
pub struct RTDChatMessageSendersBuilder {
  inner: ChatMessageSenders,
}

impl RTDChatMessageSendersBuilder {
  pub fn build(&self) -> ChatMessageSenders {
    self.inner.clone()
  }

  pub fn senders(&mut self, senders: Vec<ChatMessageSender>) -> &mut Self {
    self.inner.senders = senders;
    self
  }
}

impl AsRef<ChatMessageSenders> for ChatMessageSenders {
  fn as_ref(&self) -> &ChatMessageSenders {
    self
  }
}

impl AsRef<ChatMessageSenders> for RTDChatMessageSendersBuilder {
  fn as_ref(&self) -> &ChatMessageSenders {
    &self.inner
  }
}
