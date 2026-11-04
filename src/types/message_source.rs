use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes source of a message
pub trait TDMessageSource: Debug + RObject {}

/// Describes source of a message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageSource {
  #[doc(hidden)]
  _Default(()),
  /// The message is from a chat event log
  ChatEventLog(MessageSourceChatEventLog),
  /// The message is from a chat history
  ChatHistory(MessageSourceChatHistory),
  /// The message is from a chat list or a forum topic list
  ChatList(MessageSourceChatList),
  /// The message is from history of a topic in a channel direct messages chat administered by the current user
  DirectMessagesChatTopicHistory(MessageSourceDirectMessagesChatTopicHistory),
  /// The message is from history of a forum topic
  ForumTopicHistory(MessageSourceForumTopicHistory),
  /// The message is from chat, message thread or forum topic history preview
  HistoryPreview(MessageSourceHistoryPreview),
  /// The message is from history of a message thread
  MessageThreadHistory(MessageSourceMessageThreadHistory),
  /// The message is from a notification
  Notification(MessageSourceNotification),
  /// The message is from some other source
  Other(MessageSourceOther),
  /// The message was screenshotted; the source must be used only if the message content was visible during the screenshot
  Screenshot(MessageSourceScreenshot),
  /// The message is from search results, including file downloads, local file list, outgoing document messages, calendar
  Search(MessageSourceSearch),
}

impl Default for MessageSource {
  fn default() -> Self {
    MessageSource::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageSource {
  fn deserialize<D>(deserializer: D) -> Result<MessageSource, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageSource,
      (messageSourceChatEventLog, ChatEventLog);
      (messageSourceChatHistory, ChatHistory);
      (messageSourceChatList, ChatList);
      (messageSourceDirectMessagesChatTopicHistory, DirectMessagesChatTopicHistory);
      (messageSourceForumTopicHistory, ForumTopicHistory);
      (messageSourceHistoryPreview, HistoryPreview);
      (messageSourceMessageThreadHistory, MessageThreadHistory);
      (messageSourceNotification, Notification);
      (messageSourceOther, Other);
      (messageSourceScreenshot, Screenshot);
      (messageSourceSearch, Search);

    )(deserializer)
  }
}

impl RObject for MessageSource {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageSource::ChatEventLog(t) => t.td_name(),
      MessageSource::ChatHistory(t) => t.td_name(),
      MessageSource::ChatList(t) => t.td_name(),
      MessageSource::DirectMessagesChatTopicHistory(t) => t.td_name(),
      MessageSource::ForumTopicHistory(t) => t.td_name(),
      MessageSource::HistoryPreview(t) => t.td_name(),
      MessageSource::MessageThreadHistory(t) => t.td_name(),
      MessageSource::Notification(t) => t.td_name(),
      MessageSource::Other(t) => t.td_name(),
      MessageSource::Screenshot(t) => t.td_name(),
      MessageSource::Search(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageSource::ChatEventLog(t) => t.extra(),
      MessageSource::ChatHistory(t) => t.extra(),
      MessageSource::ChatList(t) => t.extra(),
      MessageSource::DirectMessagesChatTopicHistory(t) => t.extra(),
      MessageSource::ForumTopicHistory(t) => t.extra(),
      MessageSource::HistoryPreview(t) => t.extra(),
      MessageSource::MessageThreadHistory(t) => t.extra(),
      MessageSource::Notification(t) => t.extra(),
      MessageSource::Other(t) => t.extra(),
      MessageSource::Screenshot(t) => t.extra(),
      MessageSource::Search(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageSource::_Default(_))
  }

  pub fn is_chat_event_log(&self) -> bool {
    matches!(self, MessageSource::ChatEventLog(_))
  }
  pub fn is_chat_history(&self) -> bool {
    matches!(self, MessageSource::ChatHistory(_))
  }
  pub fn is_chat_list(&self) -> bool {
    matches!(self, MessageSource::ChatList(_))
  }
  pub fn is_direct_messages_chat_topic_history(&self) -> bool {
    matches!(self, MessageSource::DirectMessagesChatTopicHistory(_))
  }
  pub fn is_forum_topic_history(&self) -> bool {
    matches!(self, MessageSource::ForumTopicHistory(_))
  }
  pub fn is_history_preview(&self) -> bool {
    matches!(self, MessageSource::HistoryPreview(_))
  }
  pub fn is_message_thread_history(&self) -> bool {
    matches!(self, MessageSource::MessageThreadHistory(_))
  }
  pub fn is_notification(&self) -> bool {
    matches!(self, MessageSource::Notification(_))
  }
  pub fn is_other(&self) -> bool {
    matches!(self, MessageSource::Other(_))
  }
  pub fn is_screenshot(&self) -> bool {
    matches!(self, MessageSource::Screenshot(_))
  }
  pub fn is_search(&self) -> bool {
    matches!(self, MessageSource::Search(_))
  }

  pub fn on_chat_event_log<F: FnOnce(&MessageSourceChatEventLog)>(&self, fnc: F) -> &Self {
    if let MessageSource::ChatEventLog(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_history<F: FnOnce(&MessageSourceChatHistory)>(&self, fnc: F) -> &Self {
    if let MessageSource::ChatHistory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_list<F: FnOnce(&MessageSourceChatList)>(&self, fnc: F) -> &Self {
    if let MessageSource::ChatList(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_direct_messages_chat_topic_history<
    F: FnOnce(&MessageSourceDirectMessagesChatTopicHistory),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let MessageSource::DirectMessagesChatTopicHistory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_forum_topic_history<F: FnOnce(&MessageSourceForumTopicHistory)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let MessageSource::ForumTopicHistory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_history_preview<F: FnOnce(&MessageSourceHistoryPreview)>(&self, fnc: F) -> &Self {
    if let MessageSource::HistoryPreview(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_thread_history<F: FnOnce(&MessageSourceMessageThreadHistory)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let MessageSource::MessageThreadHistory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_notification<F: FnOnce(&MessageSourceNotification)>(&self, fnc: F) -> &Self {
    if let MessageSource::Notification(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_other<F: FnOnce(&MessageSourceOther)>(&self, fnc: F) -> &Self {
    if let MessageSource::Other(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_screenshot<F: FnOnce(&MessageSourceScreenshot)>(&self, fnc: F) -> &Self {
    if let MessageSource::Screenshot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_search<F: FnOnce(&MessageSourceSearch)>(&self, fnc: F) -> &Self {
    if let MessageSource::Search(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_chat_event_log(&self) -> Option<&MessageSourceChatEventLog> {
    if let MessageSource::ChatEventLog(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_history(&self) -> Option<&MessageSourceChatHistory> {
    if let MessageSource::ChatHistory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_list(&self) -> Option<&MessageSourceChatList> {
    if let MessageSource::ChatList(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_direct_messages_chat_topic_history(
    &self,
  ) -> Option<&MessageSourceDirectMessagesChatTopicHistory> {
    if let MessageSource::DirectMessagesChatTopicHistory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_forum_topic_history(&self) -> Option<&MessageSourceForumTopicHistory> {
    if let MessageSource::ForumTopicHistory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_history_preview(&self) -> Option<&MessageSourceHistoryPreview> {
    if let MessageSource::HistoryPreview(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_thread_history(&self) -> Option<&MessageSourceMessageThreadHistory> {
    if let MessageSource::MessageThreadHistory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_notification(&self) -> Option<&MessageSourceNotification> {
    if let MessageSource::Notification(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_other(&self) -> Option<&MessageSourceOther> {
    if let MessageSource::Other(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_screenshot(&self) -> Option<&MessageSourceScreenshot> {
    if let MessageSource::Screenshot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_search(&self) -> Option<&MessageSourceSearch> {
    if let MessageSource::Search(t) = self {
      return Some(t);
    }
    None
  }

  pub fn chat_event_log<T: AsRef<MessageSourceChatEventLog>>(t: T) -> Self {
    MessageSource::ChatEventLog(t.as_ref().clone())
  }

  pub fn chat_history<T: AsRef<MessageSourceChatHistory>>(t: T) -> Self {
    MessageSource::ChatHistory(t.as_ref().clone())
  }

  pub fn chat_list<T: AsRef<MessageSourceChatList>>(t: T) -> Self {
    MessageSource::ChatList(t.as_ref().clone())
  }

  pub fn direct_messages_chat_topic_history<
    T: AsRef<MessageSourceDirectMessagesChatTopicHistory>,
  >(
    t: T,
  ) -> Self {
    MessageSource::DirectMessagesChatTopicHistory(t.as_ref().clone())
  }

  pub fn forum_topic_history<T: AsRef<MessageSourceForumTopicHistory>>(t: T) -> Self {
    MessageSource::ForumTopicHistory(t.as_ref().clone())
  }

  pub fn history_preview<T: AsRef<MessageSourceHistoryPreview>>(t: T) -> Self {
    MessageSource::HistoryPreview(t.as_ref().clone())
  }

  pub fn message_thread_history<T: AsRef<MessageSourceMessageThreadHistory>>(t: T) -> Self {
    MessageSource::MessageThreadHistory(t.as_ref().clone())
  }

  pub fn notification<T: AsRef<MessageSourceNotification>>(t: T) -> Self {
    MessageSource::Notification(t.as_ref().clone())
  }

  pub fn other<T: AsRef<MessageSourceOther>>(t: T) -> Self {
    MessageSource::Other(t.as_ref().clone())
  }

  pub fn screenshot<T: AsRef<MessageSourceScreenshot>>(t: T) -> Self {
    MessageSource::Screenshot(t.as_ref().clone())
  }

  pub fn search<T: AsRef<MessageSourceSearch>>(t: T) -> Self {
    MessageSource::Search(t.as_ref().clone())
  }
}

impl AsRef<MessageSource> for MessageSource {
  fn as_ref(&self) -> &MessageSource {
    self
  }
}

/// The message is from a chat event log
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceChatEventLog {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceChatEventLog {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceChatEventLog"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceChatEventLog {}

impl MessageSourceChatEventLog {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceChatEventLogBuilder {
    let inner = MessageSourceChatEventLog {
      td_name: "messageSourceChatEventLog".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceChatEventLogBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceChatEventLogBuilder {
  inner: MessageSourceChatEventLog,
}

impl RTDMessageSourceChatEventLogBuilder {
  pub fn build(&self) -> MessageSourceChatEventLog {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceChatEventLog> for MessageSourceChatEventLog {
  fn as_ref(&self) -> &MessageSourceChatEventLog {
    self
  }
}

impl AsRef<MessageSourceChatEventLog> for RTDMessageSourceChatEventLogBuilder {
  fn as_ref(&self) -> &MessageSourceChatEventLog {
    &self.inner
  }
}

/// The message is from a chat history
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceChatHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceChatHistory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceChatHistory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceChatHistory {}

impl MessageSourceChatHistory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceChatHistoryBuilder {
    let inner = MessageSourceChatHistory {
      td_name: "messageSourceChatHistory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceChatHistoryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceChatHistoryBuilder {
  inner: MessageSourceChatHistory,
}

impl RTDMessageSourceChatHistoryBuilder {
  pub fn build(&self) -> MessageSourceChatHistory {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceChatHistory> for MessageSourceChatHistory {
  fn as_ref(&self) -> &MessageSourceChatHistory {
    self
  }
}

impl AsRef<MessageSourceChatHistory> for RTDMessageSourceChatHistoryBuilder {
  fn as_ref(&self) -> &MessageSourceChatHistory {
    &self.inner
  }
}

/// The message is from a chat list or a forum topic list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceChatList {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceChatList {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceChatList"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceChatList {}

impl MessageSourceChatList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceChatListBuilder {
    let inner = MessageSourceChatList {
      td_name: "messageSourceChatList".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceChatListBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceChatListBuilder {
  inner: MessageSourceChatList,
}

impl RTDMessageSourceChatListBuilder {
  pub fn build(&self) -> MessageSourceChatList {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceChatList> for MessageSourceChatList {
  fn as_ref(&self) -> &MessageSourceChatList {
    self
  }
}

impl AsRef<MessageSourceChatList> for RTDMessageSourceChatListBuilder {
  fn as_ref(&self) -> &MessageSourceChatList {
    &self.inner
  }
}

/// The message is from history of a topic in a channel direct messages chat administered by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceDirectMessagesChatTopicHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceDirectMessagesChatTopicHistory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceDirectMessagesChatTopicHistory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceDirectMessagesChatTopicHistory {}

impl MessageSourceDirectMessagesChatTopicHistory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceDirectMessagesChatTopicHistoryBuilder {
    let inner = MessageSourceDirectMessagesChatTopicHistory {
      td_name: "messageSourceDirectMessagesChatTopicHistory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceDirectMessagesChatTopicHistoryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceDirectMessagesChatTopicHistoryBuilder {
  inner: MessageSourceDirectMessagesChatTopicHistory,
}

impl RTDMessageSourceDirectMessagesChatTopicHistoryBuilder {
  pub fn build(&self) -> MessageSourceDirectMessagesChatTopicHistory {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceDirectMessagesChatTopicHistory>
  for MessageSourceDirectMessagesChatTopicHistory
{
  fn as_ref(&self) -> &MessageSourceDirectMessagesChatTopicHistory {
    self
  }
}

impl AsRef<MessageSourceDirectMessagesChatTopicHistory>
  for RTDMessageSourceDirectMessagesChatTopicHistoryBuilder
{
  fn as_ref(&self) -> &MessageSourceDirectMessagesChatTopicHistory {
    &self.inner
  }
}

/// The message is from history of a forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceForumTopicHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceForumTopicHistory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceForumTopicHistory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceForumTopicHistory {}

impl MessageSourceForumTopicHistory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceForumTopicHistoryBuilder {
    let inner = MessageSourceForumTopicHistory {
      td_name: "messageSourceForumTopicHistory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceForumTopicHistoryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceForumTopicHistoryBuilder {
  inner: MessageSourceForumTopicHistory,
}

impl RTDMessageSourceForumTopicHistoryBuilder {
  pub fn build(&self) -> MessageSourceForumTopicHistory {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceForumTopicHistory> for MessageSourceForumTopicHistory {
  fn as_ref(&self) -> &MessageSourceForumTopicHistory {
    self
  }
}

impl AsRef<MessageSourceForumTopicHistory> for RTDMessageSourceForumTopicHistoryBuilder {
  fn as_ref(&self) -> &MessageSourceForumTopicHistory {
    &self.inner
  }
}

/// The message is from chat, message thread or forum topic history preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceHistoryPreview {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceHistoryPreview {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceHistoryPreview"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceHistoryPreview {}

impl MessageSourceHistoryPreview {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceHistoryPreviewBuilder {
    let inner = MessageSourceHistoryPreview {
      td_name: "messageSourceHistoryPreview".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceHistoryPreviewBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceHistoryPreviewBuilder {
  inner: MessageSourceHistoryPreview,
}

impl RTDMessageSourceHistoryPreviewBuilder {
  pub fn build(&self) -> MessageSourceHistoryPreview {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceHistoryPreview> for MessageSourceHistoryPreview {
  fn as_ref(&self) -> &MessageSourceHistoryPreview {
    self
  }
}

impl AsRef<MessageSourceHistoryPreview> for RTDMessageSourceHistoryPreviewBuilder {
  fn as_ref(&self) -> &MessageSourceHistoryPreview {
    &self.inner
  }
}

/// The message is from history of a message thread
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceMessageThreadHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceMessageThreadHistory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceMessageThreadHistory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceMessageThreadHistory {}

impl MessageSourceMessageThreadHistory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceMessageThreadHistoryBuilder {
    let inner = MessageSourceMessageThreadHistory {
      td_name: "messageSourceMessageThreadHistory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceMessageThreadHistoryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceMessageThreadHistoryBuilder {
  inner: MessageSourceMessageThreadHistory,
}

impl RTDMessageSourceMessageThreadHistoryBuilder {
  pub fn build(&self) -> MessageSourceMessageThreadHistory {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceMessageThreadHistory> for MessageSourceMessageThreadHistory {
  fn as_ref(&self) -> &MessageSourceMessageThreadHistory {
    self
  }
}

impl AsRef<MessageSourceMessageThreadHistory> for RTDMessageSourceMessageThreadHistoryBuilder {
  fn as_ref(&self) -> &MessageSourceMessageThreadHistory {
    &self.inner
  }
}

/// The message is from a notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceNotification {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceNotification {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceNotification"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceNotification {}

impl MessageSourceNotification {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceNotificationBuilder {
    let inner = MessageSourceNotification {
      td_name: "messageSourceNotification".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceNotificationBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceNotificationBuilder {
  inner: MessageSourceNotification,
}

impl RTDMessageSourceNotificationBuilder {
  pub fn build(&self) -> MessageSourceNotification {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceNotification> for MessageSourceNotification {
  fn as_ref(&self) -> &MessageSourceNotification {
    self
  }
}

impl AsRef<MessageSourceNotification> for RTDMessageSourceNotificationBuilder {
  fn as_ref(&self) -> &MessageSourceNotification {
    &self.inner
  }
}

/// The message is from some other source
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceOther {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceOther {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceOther"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceOther {}

impl MessageSourceOther {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceOtherBuilder {
    let inner = MessageSourceOther {
      td_name: "messageSourceOther".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceOtherBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceOtherBuilder {
  inner: MessageSourceOther,
}

impl RTDMessageSourceOtherBuilder {
  pub fn build(&self) -> MessageSourceOther {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceOther> for MessageSourceOther {
  fn as_ref(&self) -> &MessageSourceOther {
    self
  }
}

impl AsRef<MessageSourceOther> for RTDMessageSourceOtherBuilder {
  fn as_ref(&self) -> &MessageSourceOther {
    &self.inner
  }
}

/// The message was screenshotted; the source must be used only if the message content was visible during the screenshot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceScreenshot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceScreenshot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceScreenshot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceScreenshot {}

impl MessageSourceScreenshot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceScreenshotBuilder {
    let inner = MessageSourceScreenshot {
      td_name: "messageSourceScreenshot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceScreenshotBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceScreenshotBuilder {
  inner: MessageSourceScreenshot,
}

impl RTDMessageSourceScreenshotBuilder {
  pub fn build(&self) -> MessageSourceScreenshot {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceScreenshot> for MessageSourceScreenshot {
  fn as_ref(&self) -> &MessageSourceScreenshot {
    self
  }
}

impl AsRef<MessageSourceScreenshot> for RTDMessageSourceScreenshotBuilder {
  fn as_ref(&self) -> &MessageSourceScreenshot {
    &self.inner
  }
}

/// The message is from search results, including file downloads, local file list, outgoing document messages, calendar
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceSearch {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSourceSearch {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSourceSearch"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSource for MessageSourceSearch {}

impl MessageSourceSearch {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSourceSearchBuilder {
    let inner = MessageSourceSearch {
      td_name: "messageSourceSearch".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSourceSearchBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSourceSearchBuilder {
  inner: MessageSourceSearch,
}

impl RTDMessageSourceSearchBuilder {
  pub fn build(&self) -> MessageSourceSearch {
    self.inner.clone()
  }
}

impl AsRef<MessageSourceSearch> for MessageSourceSearch {
  fn as_ref(&self) -> &MessageSourceSearch {
    self
  }
}

impl AsRef<MessageSourceSearch> for RTDMessageSourceSearchBuilder {
  fn as_ref(&self) -> &MessageSourceSearch {
    &self.inner
  }
}
