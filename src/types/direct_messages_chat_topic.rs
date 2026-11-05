use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a topic in a channel direct messages chat administered by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirectMessagesChatTopic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat to which the topic belongs
  chat_id: i64,
  /// Unique topic identifier
  id: i64,
  /// Identifier of the user or chat that sends the messages to the topic
  sender_id: MessageSender,
  /// A parameter used to determine order of the topic in the topic list. Topics must be sorted by the order in descending order
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  order: isize,
  /// True, if the other party can send unpaid messages even if the chat has paid messages enabled
  can_send_unpaid_messages: bool,
  /// True, if the topic is marked as unread
  is_marked_as_unread: bool,
  /// Number of unread messages in the chat
  unread_count: i64,
  /// Identifier of the last read incoming message
  last_read_inbox_message_id: i64,
  /// Identifier of the last read outgoing message
  last_read_outbox_message_id: i64,
  /// Number of messages with unread reactions in the chat
  unread_reaction_count: i64,
  /// Last message in the topic; may be null if none or unknown
  last_message: Option<Message>,
  /// A draft of a message in the topic; may be null if none
  draft_message: Option<DraftMessage>,
}

impl RObject for DirectMessagesChatTopic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "directMessagesChatTopic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DirectMessagesChatTopic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDirectMessagesChatTopicBuilder {
    let inner = DirectMessagesChatTopic {
      td_name: "directMessagesChatTopic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDirectMessagesChatTopicBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn order(&self) -> isize {
    self.order
  }

  pub fn can_send_unpaid_messages(&self) -> bool {
    self.can_send_unpaid_messages
  }

  pub fn is_marked_as_unread(&self) -> bool {
    self.is_marked_as_unread
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

  pub fn unread_reaction_count(&self) -> i64 {
    self.unread_reaction_count
  }

  pub fn last_message(&self) -> &Option<Message> {
    &self.last_message
  }

  pub fn draft_message(&self) -> &Option<DraftMessage> {
    &self.draft_message
  }
}

#[doc(hidden)]
pub struct RTDDirectMessagesChatTopicBuilder {
  inner: DirectMessagesChatTopic,
}

impl RTDDirectMessagesChatTopicBuilder {
  pub fn build(&self) -> DirectMessagesChatTopic {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

  pub fn order(&mut self, order: isize) -> &mut Self {
    self.inner.order = order;
    self
  }

  pub fn can_send_unpaid_messages(&mut self, can_send_unpaid_messages: bool) -> &mut Self {
    self.inner.can_send_unpaid_messages = can_send_unpaid_messages;
    self
  }

  pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
    self.inner.is_marked_as_unread = is_marked_as_unread;
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

  pub fn unread_reaction_count(&mut self, unread_reaction_count: i64) -> &mut Self {
    self.inner.unread_reaction_count = unread_reaction_count;
    self
  }

  pub fn last_message<T: AsRef<Message>>(&mut self, last_message: T) -> &mut Self {
    self.inner.last_message = Some(last_message.as_ref().clone());
    self
  }

  pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
    self.inner.draft_message = Some(draft_message.as_ref().clone());
    self
  }
}

impl AsRef<DirectMessagesChatTopic> for DirectMessagesChatTopic {
  fn as_ref(&self) -> &DirectMessagesChatTopic {
    self
  }
}

impl AsRef<DirectMessagesChatTopic> for RTDDirectMessagesChatTopicBuilder {
  fn as_ref(&self) -> &DirectMessagesChatTopic {
    &self.inner
  }
}
