use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a forwarded message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Origin of the forwarded message
  origin: MessageOrigin,
  /// Point in time (Unix timestamp) when the message was originally sent
  date: i64,
  /// For messages forwarded to the chat with the current user (Saved Messages), to the Replies bot chat, or to the channel's discussion group, information about the source message from which the message was forwarded last time; may be null for other forwards or if unknown
  source: Option<ForwardSource>,
  /// The type of public service announcement for the forwarded message
  public_service_announcement_type: String,
}

impl RObject for MessageForwardInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageForwardInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageForwardInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageForwardInfoBuilder {
    let inner = MessageForwardInfo {
      td_name: "messageForwardInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageForwardInfoBuilder { inner }
  }

  pub fn origin(&self) -> &MessageOrigin {
    &self.origin
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn source(&self) -> &Option<ForwardSource> {
    &self.source
  }

  pub fn public_service_announcement_type(&self) -> &String {
    &self.public_service_announcement_type
  }
}

#[doc(hidden)]
pub struct RTDMessageForwardInfoBuilder {
  inner: MessageForwardInfo,
}

impl RTDMessageForwardInfoBuilder {
  pub fn build(&self) -> MessageForwardInfo {
    self.inner.clone()
  }

  pub fn origin<T: AsRef<MessageOrigin>>(&mut self, origin: T) -> &mut Self {
    self.inner.origin = origin.as_ref().clone();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn source<T: AsRef<ForwardSource>>(&mut self, source: T) -> &mut Self {
    self.inner.source = Some(source.as_ref().clone());
    self
  }

  pub fn public_service_announcement_type<T: AsRef<str>>(
    &mut self,
    public_service_announcement_type: T,
  ) -> &mut Self {
    self.inner.public_service_announcement_type =
      public_service_announcement_type.as_ref().to_string();
    self
  }
}

impl AsRef<MessageForwardInfo> for MessageForwardInfo {
  fn as_ref(&self) -> &MessageForwardInfo {
    self
  }
}

impl AsRef<MessageForwardInfo> for RTDMessageForwardInfoBuilder {
  fn as_ref(&self) -> &MessageForwardInfo {
    &self.inner
  }
}
