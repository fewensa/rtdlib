use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a message sent in a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique message identifier within the group call
  message_id: i64,
  /// Identifier of the sender of the message
  sender_id: MessageSender,
  /// Point in time (Unix timestamp) when the message was sent
  date: i64,
  /// Text of the message. If empty, then the message is a paid reaction in a live story
  text: FormattedText,
  /// The number of Telegram Stars that were paid to send the message; for live stories only
  paid_message_star_count: i64,
  /// True, if the message is sent by the owner of the call and must be treated as a message of the maximum level; for live stories only
  is_from_owner: bool,
  /// True, if the message can be deleted by the current user; for live stories only
  can_be_deleted: bool,
}

impl RObject for GroupCallMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCallMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GroupCallMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallMessageBuilder {
    let inner = GroupCallMessage {
      td_name: "groupCallMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGroupCallMessageBuilder { inner }
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn paid_message_star_count(&self) -> i64 {
    self.paid_message_star_count
  }

  pub fn is_from_owner(&self) -> bool {
    self.is_from_owner
  }

  pub fn can_be_deleted(&self) -> bool {
    self.can_be_deleted
  }
}

#[doc(hidden)]
pub struct RTDGroupCallMessageBuilder {
  inner: GroupCallMessage,
}

impl RTDGroupCallMessageBuilder {
  pub fn build(&self) -> GroupCallMessage {
    self.inner.clone()
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn paid_message_star_count(&mut self, paid_message_star_count: i64) -> &mut Self {
    self.inner.paid_message_star_count = paid_message_star_count;
    self
  }

  pub fn is_from_owner(&mut self, is_from_owner: bool) -> &mut Self {
    self.inner.is_from_owner = is_from_owner;
    self
  }

  pub fn can_be_deleted(&mut self, can_be_deleted: bool) -> &mut Self {
    self.inner.can_be_deleted = can_be_deleted;
    self
  }
}

impl AsRef<GroupCallMessage> for GroupCallMessage {
  fn as_ref(&self) -> &GroupCallMessage {
    self
  }
}

impl AsRef<GroupCallMessage> for RTDGroupCallMessageBuilder {
  fn as_ref(&self) -> &GroupCallMessage {
    &self.inner
  }
}
