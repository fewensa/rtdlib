use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of object, for which statistics are provided
pub trait TDChatStatisticsObjectType: Debug + RObject {}

/// Describes type of object, for which statistics are provided
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatStatisticsObjectType {
  #[doc(hidden)]
  _Default(()),
  /// Describes a message sent in the chat
  Message(ChatStatisticsObjectTypeMessage),
  /// Describes a story posted on behalf of the chat
  Story(ChatStatisticsObjectTypeStory),
}

impl Default for ChatStatisticsObjectType {
  fn default() -> Self {
    ChatStatisticsObjectType::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatStatisticsObjectType {
  fn deserialize<D>(deserializer: D) -> Result<ChatStatisticsObjectType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatStatisticsObjectType,
      (chatStatisticsObjectTypeMessage, Message);
      (chatStatisticsObjectTypeStory, Story);

    )(deserializer)
  }
}

impl RObject for ChatStatisticsObjectType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatStatisticsObjectType::Message(t) => t.td_name(),
      ChatStatisticsObjectType::Story(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatStatisticsObjectType::Message(t) => t.extra(),
      ChatStatisticsObjectType::Story(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatStatisticsObjectType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatStatisticsObjectType::_Default(_))
  }

  pub fn is_message(&self) -> bool {
    matches!(self, ChatStatisticsObjectType::Message(_))
  }
  pub fn is_story(&self) -> bool {
    matches!(self, ChatStatisticsObjectType::Story(_))
  }

  pub fn on_message<F: FnOnce(&ChatStatisticsObjectTypeMessage)>(&self, fnc: F) -> &Self {
    if let ChatStatisticsObjectType::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story<F: FnOnce(&ChatStatisticsObjectTypeStory)>(&self, fnc: F) -> &Self {
    if let ChatStatisticsObjectType::Story(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_message(&self) -> Option<&ChatStatisticsObjectTypeMessage> {
    if let ChatStatisticsObjectType::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story(&self) -> Option<&ChatStatisticsObjectTypeStory> {
    if let ChatStatisticsObjectType::Story(t) = self {
      return Some(t);
    }
    None
  }

  pub fn message<T: AsRef<ChatStatisticsObjectTypeMessage>>(t: T) -> Self {
    ChatStatisticsObjectType::Message(t.as_ref().clone())
  }

  pub fn story<T: AsRef<ChatStatisticsObjectTypeStory>>(t: T) -> Self {
    ChatStatisticsObjectType::Story(t.as_ref().clone())
  }
}

impl AsRef<ChatStatisticsObjectType> for ChatStatisticsObjectType {
  fn as_ref(&self) -> &ChatStatisticsObjectType {
    self
  }
}

/// Describes a message sent in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsObjectTypeMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message identifier
  message_id: i64,
}

impl RObject for ChatStatisticsObjectTypeMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatStatisticsObjectTypeMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatStatisticsObjectType for ChatStatisticsObjectTypeMessage {}

impl ChatStatisticsObjectTypeMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatStatisticsObjectTypeMessageBuilder {
    let inner = ChatStatisticsObjectTypeMessage {
      td_name: "chatStatisticsObjectTypeMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatStatisticsObjectTypeMessageBuilder { inner }
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDChatStatisticsObjectTypeMessageBuilder {
  inner: ChatStatisticsObjectTypeMessage,
}

impl RTDChatStatisticsObjectTypeMessageBuilder {
  pub fn build(&self) -> ChatStatisticsObjectTypeMessage {
    self.inner.clone()
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }
}

impl AsRef<ChatStatisticsObjectTypeMessage> for ChatStatisticsObjectTypeMessage {
  fn as_ref(&self) -> &ChatStatisticsObjectTypeMessage {
    self
  }
}

impl AsRef<ChatStatisticsObjectTypeMessage> for RTDChatStatisticsObjectTypeMessageBuilder {
  fn as_ref(&self) -> &ChatStatisticsObjectTypeMessage {
    &self.inner
  }
}

/// Describes a story posted on behalf of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsObjectTypeStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Story identifier
  story_id: i64,
}

impl RObject for ChatStatisticsObjectTypeStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatStatisticsObjectTypeStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatStatisticsObjectType for ChatStatisticsObjectTypeStory {}

impl ChatStatisticsObjectTypeStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatStatisticsObjectTypeStoryBuilder {
    let inner = ChatStatisticsObjectTypeStory {
      td_name: "chatStatisticsObjectTypeStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatStatisticsObjectTypeStoryBuilder { inner }
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDChatStatisticsObjectTypeStoryBuilder {
  inner: ChatStatisticsObjectTypeStory,
}

impl RTDChatStatisticsObjectTypeStoryBuilder {
  pub fn build(&self) -> ChatStatisticsObjectTypeStory {
    self.inner.clone()
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<ChatStatisticsObjectTypeStory> for ChatStatisticsObjectTypeStory {
  fn as_ref(&self) -> &ChatStatisticsObjectTypeStory {
    self
  }
}

impl AsRef<ChatStatisticsObjectTypeStory> for RTDChatStatisticsObjectTypeStoryBuilder {
  fn as_ref(&self) -> &ChatStatisticsObjectTypeStory {
    &self.inner
  }
}
