use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumTopic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Basic information about the topic
  info: ForumTopicInfo,
  /// Last message in the topic; may be null if unknown
  last_message: Option<Message>,
  /// A parameter used to determine order of the topic in the topic list. Topics must be sorted by the order in descending order
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  order: isize,
  /// True, if the topic is pinned in the topic list
  is_pinned: bool,
  /// Number of unread messages in the topic
  unread_count: i64,
  /// Identifier of the last read incoming message
  last_read_inbox_message_id: i64,
  /// Identifier of the last read outgoing message
  last_read_outbox_message_id: i64,
  /// Number of unread messages with a mention/reply in the topic
  unread_mention_count: i64,
  /// Number of messages with unread reactions in the topic
  unread_reaction_count: i64,
  /// Number of messages with unread poll votes in the topic
  unread_poll_vote_count: i64,
  /// Notification settings for the topic
  notification_settings: ChatNotificationSettings,
  /// A draft of a message in the topic; may be null if none
  draft_message: Option<DraftMessage>,
}

impl RObject for ForumTopic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "forumTopic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ForumTopic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDForumTopicBuilder {
    let inner = ForumTopic {
      td_name: "forumTopic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDForumTopicBuilder { inner }
  }

  pub fn info(&self) -> &ForumTopicInfo {
    &self.info
  }

  pub fn last_message(&self) -> &Option<Message> {
    &self.last_message
  }

  pub fn order(&self) -> isize {
    self.order
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }

  pub fn unread_count(&self) -> i64 {
    self.unread_count
  }

  pub fn last_read_inbox_message_id(&self) -> i64 {
    self.last_read_inbox_message_id
  }

  pub fn last_read_outbox_message_id(&self) -> i64 {
    self.last_read_outbox_message_id
  }

  pub fn unread_mention_count(&self) -> i64 {
    self.unread_mention_count
  }

  pub fn unread_reaction_count(&self) -> i64 {
    self.unread_reaction_count
  }

  pub fn unread_poll_vote_count(&self) -> i64 {
    self.unread_poll_vote_count
  }

  pub fn notification_settings(&self) -> &ChatNotificationSettings {
    &self.notification_settings
  }

  pub fn draft_message(&self) -> &Option<DraftMessage> {
    &self.draft_message
  }
}

#[doc(hidden)]
pub struct RTDForumTopicBuilder {
  inner: ForumTopic,
}

impl RTDForumTopicBuilder {
  pub fn build(&self) -> ForumTopic {
    self.inner.clone()
  }

  pub fn info<T: AsRef<ForumTopicInfo>>(&mut self, info: T) -> &mut Self {
    self.inner.info = info.as_ref().clone();
    self
  }

  pub fn last_message<T: AsRef<Message>>(&mut self, last_message: T) -> &mut Self {
    self.inner.last_message = Some(last_message.as_ref().clone());
    self
  }

  pub fn order(&mut self, order: isize) -> &mut Self {
    self.inner.order = order;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

  pub fn unread_count(&mut self, unread_count: i64) -> &mut Self {
    self.inner.unread_count = unread_count;
    self
  }

  pub fn last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self {
    self.inner.last_read_inbox_message_id = last_read_inbox_message_id;
    self
  }

  pub fn last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self {
    self.inner.last_read_outbox_message_id = last_read_outbox_message_id;
    self
  }

  pub fn unread_mention_count(&mut self, unread_mention_count: i64) -> &mut Self {
    self.inner.unread_mention_count = unread_mention_count;
    self
  }

  pub fn unread_reaction_count(&mut self, unread_reaction_count: i64) -> &mut Self {
    self.inner.unread_reaction_count = unread_reaction_count;
    self
  }

  pub fn unread_poll_vote_count(&mut self, unread_poll_vote_count: i64) -> &mut Self {
    self.inner.unread_poll_vote_count = unread_poll_vote_count;
    self
  }

  pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(
    &mut self,
    notification_settings: T,
  ) -> &mut Self {
    self.inner.notification_settings = notification_settings.as_ref().clone();
    self
  }

  pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
    self.inner.draft_message = Some(draft_message.as_ref().clone());
    self
  }
}

impl AsRef<ForumTopic> for ForumTopic {
  fn as_ref(&self) -> &ForumTopic {
    self
  }
}

impl AsRef<ForumTopic> for RTDForumTopicBuilder {
  fn as_ref(&self) -> &ForumTopic {
    &self.inner
  }
}
