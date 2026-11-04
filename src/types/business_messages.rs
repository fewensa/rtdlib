use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of messages from a business account as received by a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of business messages
  messages: Vec<BusinessMessage>,
}

impl RObject for BusinessMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessMessagesBuilder {
    let inner = BusinessMessages {
      td_name: "businessMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessMessagesBuilder { inner }
  }

  pub fn messages(&self) -> &Vec<BusinessMessage> {
    &self.messages
  }
}

#[doc(hidden)]
pub struct RTDBusinessMessagesBuilder {
  inner: BusinessMessages,
}

impl RTDBusinessMessagesBuilder {
  pub fn build(&self) -> BusinessMessages {
    self.inner.clone()
  }

  pub fn messages(&mut self, messages: Vec<BusinessMessage>) -> &mut Self {
    self.inner.messages = messages;
    self
  }
}

impl AsRef<BusinessMessages> for BusinessMessages {
  fn as_ref(&self) -> &BusinessMessages {
    self
  }
}

impl AsRef<BusinessMessages> for RTDBusinessMessagesBuilder {
  fn as_ref(&self) -> &BusinessMessages {
    &self.inner
  }
}
