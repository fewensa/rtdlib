use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of sponsored messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SponsoredMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of sponsored messages
  messages: Vec<SponsoredMessage>,
  /// The minimum number of messages between shown sponsored messages, or 0 if only one sponsored message must be shown after all ordinary messages
  messages_between: i64,
}

impl RObject for SponsoredMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sponsoredMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SponsoredMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSponsoredMessagesBuilder {
    let inner = SponsoredMessages {
      td_name: "sponsoredMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSponsoredMessagesBuilder { inner }
  }

  pub fn messages(&self) -> &Vec<SponsoredMessage> {
    &self.messages
  }

  pub fn messages_between(&self) -> i64 {
    self.messages_between
  }
}

#[doc(hidden)]
pub struct RTDSponsoredMessagesBuilder {
  inner: SponsoredMessages,
}

impl RTDSponsoredMessagesBuilder {
  pub fn build(&self) -> SponsoredMessages {
    self.inner.clone()
  }

  pub fn messages(&mut self, messages: Vec<SponsoredMessage>) -> &mut Self {
    self.inner.messages = messages;
    self
  }

  pub fn messages_between(&mut self, messages_between: i64) -> &mut Self {
    self.inner.messages_between = messages_between;
    self
  }
}

impl AsRef<SponsoredMessages> for SponsoredMessages {
  fn as_ref(&self) -> &SponsoredMessages {
    self
  }
}

impl AsRef<SponsoredMessages> for RTDSponsoredMessagesBuilder {
  fn as_ref(&self) -> &SponsoredMessages {
    &self.inner
  }
}
