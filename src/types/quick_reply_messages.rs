use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of quick reply messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuickReplyMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of quick reply messages; messages may be null
  messages: Option<Vec<QuickReplyMessage>>,
}

impl RObject for QuickReplyMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "quickReplyMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl QuickReplyMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDQuickReplyMessagesBuilder {
    let inner = QuickReplyMessages {
      td_name: "quickReplyMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDQuickReplyMessagesBuilder { inner }
  }

  pub fn messages(&self) -> &Option<Vec<QuickReplyMessage>> {
    &self.messages
  }
}

#[doc(hidden)]
pub struct RTDQuickReplyMessagesBuilder {
  inner: QuickReplyMessages,
}

impl RTDQuickReplyMessagesBuilder {
  pub fn build(&self) -> QuickReplyMessages {
    self.inner.clone()
  }

  pub fn messages(&mut self, messages: Vec<QuickReplyMessage>) -> &mut Self {
    self.inner.messages = Some(messages);
    self
  }
}

impl AsRef<QuickReplyMessages> for QuickReplyMessages {
  fn as_ref(&self) -> &QuickReplyMessages {
    self
  }
}

impl AsRef<QuickReplyMessages> for RTDQuickReplyMessagesBuilder {
  fn as_ref(&self) -> &QuickReplyMessages {
    &self.inner
  }
}
