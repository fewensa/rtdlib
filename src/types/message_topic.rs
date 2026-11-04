use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a topic of messages in a chat
pub trait TDMessageTopic: Debug + RObject {}

/// Describes a topic of messages in a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageTopic {
  #[doc(hidden)]
  _Default(()),
  /// A topic in a channel direct messages chat administered by the current user
  DirectMessages(MessageTopicDirectMessages),
  /// A topic in a forum supergroup chat or a chat with a bot
  Forum(MessageTopicForum),
  /// A topic in Saved Messages chat
  SavedMessages(MessageTopicSavedMessages),
  /// A topic in a non-forum supergroup chat
  Thread(MessageTopicThread),
}

impl Default for MessageTopic {
  fn default() -> Self {
    MessageTopic::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageTopic {
  fn deserialize<D>(deserializer: D) -> Result<MessageTopic, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageTopic,
      (messageTopicDirectMessages, DirectMessages);
      (messageTopicForum, Forum);
      (messageTopicSavedMessages, SavedMessages);
      (messageTopicThread, Thread);

    )(deserializer)
  }
}

impl RObject for MessageTopic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageTopic::DirectMessages(t) => t.td_name(),
      MessageTopic::Forum(t) => t.td_name(),
      MessageTopic::SavedMessages(t) => t.td_name(),
      MessageTopic::Thread(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageTopic::DirectMessages(t) => t.extra(),
      MessageTopic::Forum(t) => t.extra(),
      MessageTopic::SavedMessages(t) => t.extra(),
      MessageTopic::Thread(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageTopic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageTopic::_Default(_))
  }

  pub fn is_direct_messages(&self) -> bool {
    matches!(self, MessageTopic::DirectMessages(_))
  }
  pub fn is_forum(&self) -> bool {
    matches!(self, MessageTopic::Forum(_))
  }
  pub fn is_saved_messages(&self) -> bool {
    matches!(self, MessageTopic::SavedMessages(_))
  }
  pub fn is_thread(&self) -> bool {
    matches!(self, MessageTopic::Thread(_))
  }

  pub fn on_direct_messages<F: FnOnce(&MessageTopicDirectMessages)>(&self, fnc: F) -> &Self {
    if let MessageTopic::DirectMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_forum<F: FnOnce(&MessageTopicForum)>(&self, fnc: F) -> &Self {
    if let MessageTopic::Forum(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_messages<F: FnOnce(&MessageTopicSavedMessages)>(&self, fnc: F) -> &Self {
    if let MessageTopic::SavedMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_thread<F: FnOnce(&MessageTopicThread)>(&self, fnc: F) -> &Self {
    if let MessageTopic::Thread(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_direct_messages(&self) -> Option<&MessageTopicDirectMessages> {
    if let MessageTopic::DirectMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_forum(&self) -> Option<&MessageTopicForum> {
    if let MessageTopic::Forum(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_messages(&self) -> Option<&MessageTopicSavedMessages> {
    if let MessageTopic::SavedMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_thread(&self) -> Option<&MessageTopicThread> {
    if let MessageTopic::Thread(t) = self {
      return Some(t);
    }
    None
  }

  pub fn direct_messages<T: AsRef<MessageTopicDirectMessages>>(t: T) -> Self {
    MessageTopic::DirectMessages(t.as_ref().clone())
  }

  pub fn forum<T: AsRef<MessageTopicForum>>(t: T) -> Self {
    MessageTopic::Forum(t.as_ref().clone())
  }

  pub fn saved_messages<T: AsRef<MessageTopicSavedMessages>>(t: T) -> Self {
    MessageTopic::SavedMessages(t.as_ref().clone())
  }

  pub fn thread<T: AsRef<MessageTopicThread>>(t: T) -> Self {
    MessageTopic::Thread(t.as_ref().clone())
  }
}

impl AsRef<MessageTopic> for MessageTopic {
  fn as_ref(&self) -> &MessageTopic {
    self
  }
}

/// A topic in a channel direct messages chat administered by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageTopicDirectMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the topic
  direct_messages_chat_topic_id: i64,
}

impl RObject for MessageTopicDirectMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageTopicDirectMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageTopic for MessageTopicDirectMessages {}

impl MessageTopicDirectMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageTopicDirectMessagesBuilder {
    let inner = MessageTopicDirectMessages {
      td_name: "messageTopicDirectMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageTopicDirectMessagesBuilder { inner }
  }

  pub fn direct_messages_chat_topic_id(&self) -> i64 {
    self.direct_messages_chat_topic_id
  }
}

#[doc(hidden)]
pub struct RTDMessageTopicDirectMessagesBuilder {
  inner: MessageTopicDirectMessages,
}

impl RTDMessageTopicDirectMessagesBuilder {
  pub fn build(&self) -> MessageTopicDirectMessages {
    self.inner.clone()
  }

  pub fn direct_messages_chat_topic_id(&mut self, direct_messages_chat_topic_id: i64) -> &mut Self {
    self.inner.direct_messages_chat_topic_id = direct_messages_chat_topic_id;
    self
  }
}

impl AsRef<MessageTopicDirectMessages> for MessageTopicDirectMessages {
  fn as_ref(&self) -> &MessageTopicDirectMessages {
    self
  }
}

impl AsRef<MessageTopicDirectMessages> for RTDMessageTopicDirectMessagesBuilder {
  fn as_ref(&self) -> &MessageTopicDirectMessages {
    &self.inner
  }
}

/// A topic in a forum supergroup chat or a chat with a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageTopicForum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the forum topic
  forum_topic_id: i64,
}

impl RObject for MessageTopicForum {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageTopicForum"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageTopic for MessageTopicForum {}

impl MessageTopicForum {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageTopicForumBuilder {
    let inner = MessageTopicForum {
      td_name: "messageTopicForum".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageTopicForumBuilder { inner }
  }

  pub fn forum_topic_id(&self) -> i64 {
    self.forum_topic_id
  }
}

#[doc(hidden)]
pub struct RTDMessageTopicForumBuilder {
  inner: MessageTopicForum,
}

impl RTDMessageTopicForumBuilder {
  pub fn build(&self) -> MessageTopicForum {
    self.inner.clone()
  }

  pub fn forum_topic_id(&mut self, forum_topic_id: i64) -> &mut Self {
    self.inner.forum_topic_id = forum_topic_id;
    self
  }
}

impl AsRef<MessageTopicForum> for MessageTopicForum {
  fn as_ref(&self) -> &MessageTopicForum {
    self
  }
}

impl AsRef<MessageTopicForum> for RTDMessageTopicForumBuilder {
  fn as_ref(&self) -> &MessageTopicForum {
    &self.inner
  }
}

/// A topic in Saved Messages chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageTopicSavedMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the Saved Messages topic
  saved_messages_topic_id: i64,
}

impl RObject for MessageTopicSavedMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageTopicSavedMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageTopic for MessageTopicSavedMessages {}

impl MessageTopicSavedMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageTopicSavedMessagesBuilder {
    let inner = MessageTopicSavedMessages {
      td_name: "messageTopicSavedMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageTopicSavedMessagesBuilder { inner }
  }

  pub fn saved_messages_topic_id(&self) -> i64 {
    self.saved_messages_topic_id
  }
}

#[doc(hidden)]
pub struct RTDMessageTopicSavedMessagesBuilder {
  inner: MessageTopicSavedMessages,
}

impl RTDMessageTopicSavedMessagesBuilder {
  pub fn build(&self) -> MessageTopicSavedMessages {
    self.inner.clone()
  }

  pub fn saved_messages_topic_id(&mut self, saved_messages_topic_id: i64) -> &mut Self {
    self.inner.saved_messages_topic_id = saved_messages_topic_id;
    self
  }
}

impl AsRef<MessageTopicSavedMessages> for MessageTopicSavedMessages {
  fn as_ref(&self) -> &MessageTopicSavedMessages {
    self
  }
}

impl AsRef<MessageTopicSavedMessages> for RTDMessageTopicSavedMessagesBuilder {
  fn as_ref(&self) -> &MessageTopicSavedMessages {
    &self.inner
  }
}

/// A topic in a non-forum supergroup chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageTopicThread {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the message thread
  message_thread_id: i64,
}

impl RObject for MessageTopicThread {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageTopicThread"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageTopic for MessageTopicThread {}

impl MessageTopicThread {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageTopicThreadBuilder {
    let inner = MessageTopicThread {
      td_name: "messageTopicThread".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageTopicThreadBuilder { inner }
  }

  pub fn message_thread_id(&self) -> i64 {
    self.message_thread_id
  }
}

#[doc(hidden)]
pub struct RTDMessageTopicThreadBuilder {
  inner: MessageTopicThread,
}

impl RTDMessageTopicThreadBuilder {
  pub fn build(&self) -> MessageTopicThread {
    self.inner.clone()
  }

  pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
    self.inner.message_thread_id = message_thread_id;
    self
  }
}

impl AsRef<MessageTopicThread> for MessageTopicThread {
  fn as_ref(&self) -> &MessageTopicThread {
    self
  }
}

impl AsRef<MessageTopicThread> for RTDMessageTopicThreadBuilder {
  fn as_ref(&self) -> &MessageTopicThread {
    &self.inner
  }
}
