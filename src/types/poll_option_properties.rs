use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains properties of a poll option and describes actions that can be done with the option right now
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollOptionProperties {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the option can be deleted using deletePollOption
  can_be_deleted: bool,
  /// True, if the poll option can be replied in the same chat and forum topic using inputMessageReplyToMessage
  can_be_replied: bool,
  /// True, if the poll option can be replied in another chat or forum topic using inputMessageReplyToExternalMessage
  can_be_replied_in_another_chat: bool,
  /// True, if a link can be generated for the poll option using getMessageLink
  can_get_link: bool,
}

impl RObject for PollOptionProperties {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollOptionProperties"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PollOptionProperties {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollOptionPropertiesBuilder {
    let inner = PollOptionProperties {
      td_name: "pollOptionProperties".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollOptionPropertiesBuilder { inner }
  }

  pub fn can_be_deleted(&self) -> bool {
    self.can_be_deleted
  }

  pub fn can_be_replied(&self) -> bool {
    self.can_be_replied
  }

  pub fn can_be_replied_in_another_chat(&self) -> bool {
    self.can_be_replied_in_another_chat
  }

  pub fn can_get_link(&self) -> bool {
    self.can_get_link
  }
}

#[doc(hidden)]
pub struct RTDPollOptionPropertiesBuilder {
  inner: PollOptionProperties,
}

impl RTDPollOptionPropertiesBuilder {
  pub fn build(&self) -> PollOptionProperties {
    self.inner.clone()
  }

  pub fn can_be_deleted(&mut self, can_be_deleted: bool) -> &mut Self {
    self.inner.can_be_deleted = can_be_deleted;
    self
  }

  pub fn can_be_replied(&mut self, can_be_replied: bool) -> &mut Self {
    self.inner.can_be_replied = can_be_replied;
    self
  }

  pub fn can_be_replied_in_another_chat(
    &mut self,
    can_be_replied_in_another_chat: bool,
  ) -> &mut Self {
    self.inner.can_be_replied_in_another_chat = can_be_replied_in_another_chat;
    self
  }

  pub fn can_get_link(&mut self, can_get_link: bool) -> &mut Self {
    self.inner.can_get_link = can_get_link;
    self
  }
}

impl AsRef<PollOptionProperties> for PollOptionProperties {
  fn as_ref(&self) -> &PollOptionProperties {
    self
  }
}

impl AsRef<PollOptionProperties> for RTDPollOptionPropertiesBuilder {
  fn as_ref(&self) -> &PollOptionProperties {
    &self.inner
  }
}
