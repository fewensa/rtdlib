use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of message viewers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageViewers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of message viewers
  viewers: Vec<MessageViewer>,
}

impl RObject for MessageViewers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageViewers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageViewers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageViewersBuilder {
    let inner = MessageViewers {
      td_name: "messageViewers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageViewersBuilder { inner }
  }

  pub fn viewers(&self) -> &Vec<MessageViewer> {
    &self.viewers
  }
}

#[doc(hidden)]
pub struct RTDMessageViewersBuilder {
  inner: MessageViewers,
}

impl RTDMessageViewersBuilder {
  pub fn build(&self) -> MessageViewers {
    self.inner.clone()
  }

  pub fn viewers(&mut self, viewers: Vec<MessageViewer>) -> &mut Self {
    self.inner.viewers = viewers;
    self
  }
}

impl AsRef<MessageViewers> for MessageViewers {
  fn as_ref(&self) -> &MessageViewers {
    self
  }
}

impl AsRef<MessageViewers> for RTDMessageViewersBuilder {
  fn as_ref(&self) -> &MessageViewers {
    &self.inner
  }
}
