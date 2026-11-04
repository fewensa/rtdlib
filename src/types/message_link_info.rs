use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a link to a message or a forum topic in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the link is a public link for a message or a forum topic in a chat
  is_public: bool,
  /// If found, identifier of the chat to which the link points, 0 otherwise
  chat_id: i64,
  /// Identifier of the specific topic in which the message must be opened, or a topic to open if the message is missing; may be null if none
  topic_id: Option<MessageTopic>,
  /// If found, the linked message; may be null
  message: Option<Message>,
  /// Timestamp from which the video/audio/video note/voice note/story playing must start, in seconds; 0 if not specified. The media can be in the message content or in its link preview
  media_timestamp: i64,
  /// Identifier of the checklist task that is linked; 0 if none
  checklist_task_id: i64,
  /// Identifier of the poll option that is linked; empty if none
  poll_option_id: String,
  /// True, if the whole media album to which the message belongs is linked
  for_album: bool,
}

impl RObject for MessageLinkInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageLinkInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageLinkInfoBuilder {
    let inner = MessageLinkInfo {
      td_name: "messageLinkInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageLinkInfoBuilder { inner }
  }

  pub fn is_public(&self) -> bool {
    self.is_public
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn topic_id(&self) -> &Option<MessageTopic> {
    &self.topic_id
  }

  pub fn message(&self) -> &Option<Message> {
    &self.message
  }

  pub fn media_timestamp(&self) -> i64 {
    self.media_timestamp
  }

  pub fn checklist_task_id(&self) -> i64 {
    self.checklist_task_id
  }

  pub fn poll_option_id(&self) -> &String {
    &self.poll_option_id
  }

  pub fn for_album(&self) -> bool {
    self.for_album
  }
}

#[doc(hidden)]
pub struct RTDMessageLinkInfoBuilder {
  inner: MessageLinkInfo,
}

impl RTDMessageLinkInfoBuilder {
  pub fn build(&self) -> MessageLinkInfo {
    self.inner.clone()
  }

  pub fn is_public(&mut self, is_public: bool) -> &mut Self {
    self.inner.is_public = is_public;
    self
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn topic_id<T: AsRef<MessageTopic>>(&mut self, topic_id: T) -> &mut Self {
    self.inner.topic_id = Some(topic_id.as_ref().clone());
    self
  }

  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = Some(message.as_ref().clone());
    self
  }

  pub fn media_timestamp(&mut self, media_timestamp: i64) -> &mut Self {
    self.inner.media_timestamp = media_timestamp;
    self
  }

  pub fn checklist_task_id(&mut self, checklist_task_id: i64) -> &mut Self {
    self.inner.checklist_task_id = checklist_task_id;
    self
  }

  pub fn poll_option_id<T: AsRef<str>>(&mut self, poll_option_id: T) -> &mut Self {
    self.inner.poll_option_id = poll_option_id.as_ref().to_string();
    self
  }

  pub fn for_album(&mut self, for_album: bool) -> &mut Self {
    self.inner.for_album = for_album;
    self
  }
}

impl AsRef<MessageLinkInfo> for MessageLinkInfo {
  fn as_ref(&self) -> &MessageLinkInfo {
    self
  }
}

impl AsRef<MessageLinkInfo> for RTDMessageLinkInfoBuilder {
  fn as_ref(&self) -> &MessageLinkInfo {
    &self.inner
  }
}
