use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about the message or the story a message is replying to
pub trait TDMessageReplyTo: Debug + RObject {}

/// Contains information about the message or the story a message is replying to
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageReplyTo {
  #[doc(hidden)]
  _Default(()),
  /// Describes a message replied by a given message
  Message(MessageReplyToMessage),
  /// Describes a story replied by a given message
  Story(MessageReplyToStory),
}

impl Default for MessageReplyTo {
  fn default() -> Self {
    MessageReplyTo::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageReplyTo {
  fn deserialize<D>(deserializer: D) -> Result<MessageReplyTo, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageReplyTo,
      (messageReplyToMessage, Message);
      (messageReplyToStory, Story);

    )(deserializer)
  }
}

impl RObject for MessageReplyTo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageReplyTo::Message(t) => t.td_name(),
      MessageReplyTo::Story(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageReplyTo::Message(t) => t.extra(),
      MessageReplyTo::Story(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageReplyTo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageReplyTo::_Default(_))
  }

  pub fn is_message(&self) -> bool {
    matches!(self, MessageReplyTo::Message(_))
  }
  pub fn is_story(&self) -> bool {
    matches!(self, MessageReplyTo::Story(_))
  }

  pub fn on_message<F: FnOnce(&MessageReplyToMessage)>(&self, fnc: F) -> &Self {
    if let MessageReplyTo::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story<F: FnOnce(&MessageReplyToStory)>(&self, fnc: F) -> &Self {
    if let MessageReplyTo::Story(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_message(&self) -> Option<&MessageReplyToMessage> {
    if let MessageReplyTo::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story(&self) -> Option<&MessageReplyToStory> {
    if let MessageReplyTo::Story(t) = self {
      return Some(t);
    }
    None
  }

  pub fn message<T: AsRef<MessageReplyToMessage>>(t: T) -> Self {
    MessageReplyTo::Message(t.as_ref().clone())
  }

  pub fn story<T: AsRef<MessageReplyToStory>>(t: T) -> Self {
    MessageReplyTo::Story(t.as_ref().clone())
  }
}

impl AsRef<MessageReplyTo> for MessageReplyTo {
  fn as_ref(&self) -> &MessageReplyTo {
    self
  }
}

/// Describes a message replied by a given message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReplyToMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the chat to which the message belongs; may be 0 if the replied message is in unknown chat
  chat_id: i64,
  /// The identifier of the message; may be 0 if the replied message is in unknown chat
  message_id: i64,
  /// Chosen quote from the replied message; may be null if none
  quote: Option<TextQuote>,
  /// Identifier of the checklist task in the original message that was replied; 0 if none
  checklist_task_id: i64,
  /// Identifier of the poll option in the original message that was replied; empty if none
  poll_option_id: String,
  /// Information about origin of the message if the message was from another chat or topic; may be null for messages from the same chat
  origin: Option<MessageOrigin>,
  /// Point in time (Unix timestamp) when the message was sent if the message was from another chat or topic; 0 for messages from the same chat
  origin_send_date: i64,
  /// Media content of the message if the message was from another chat or topic; may be null for messages from the same chat and messages without media. Can be only one of the following types: messageAnimation, messageAudio, messageChecklist, messageContact, messageDice, messageDocument, messageGame, messageGiveaway, messageGiveawayWinners, messageInvoice, messageLocation, messagePaidMedia, messagePhoto, messagePoll, messageStakeDice, messageSticker, messageStory, messageText (for link preview), messageVenue, messageVideo, messageVideoNote, or messageVoiceNote
  content: Option<MessageContent>,
}

impl RObject for MessageReplyToMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReplyToMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageReplyTo for MessageReplyToMessage {}

impl MessageReplyToMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReplyToMessageBuilder {
    let inner = MessageReplyToMessage {
      td_name: "messageReplyToMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageReplyToMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn quote(&self) -> &Option<TextQuote> {
    &self.quote
  }

  pub fn checklist_task_id(&self) -> i64 {
    self.checklist_task_id
  }

  pub fn poll_option_id(&self) -> &String {
    &self.poll_option_id
  }

  pub fn origin(&self) -> &Option<MessageOrigin> {
    &self.origin
  }

  pub fn origin_send_date(&self) -> i64 {
    self.origin_send_date
  }

  pub fn content(&self) -> &Option<MessageContent> {
    &self.content
  }
}

#[doc(hidden)]
pub struct RTDMessageReplyToMessageBuilder {
  inner: MessageReplyToMessage,
}

impl RTDMessageReplyToMessageBuilder {
  pub fn build(&self) -> MessageReplyToMessage {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn quote<T: AsRef<TextQuote>>(&mut self, quote: T) -> &mut Self {
    self.inner.quote = Some(quote.as_ref().clone());
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

  pub fn origin<T: AsRef<MessageOrigin>>(&mut self, origin: T) -> &mut Self {
    self.inner.origin = Some(origin.as_ref().clone());
    self
  }

  pub fn origin_send_date(&mut self, origin_send_date: i64) -> &mut Self {
    self.inner.origin_send_date = origin_send_date;
    self
  }

  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = Some(content.as_ref().clone());
    self
  }
}

impl AsRef<MessageReplyToMessage> for MessageReplyToMessage {
  fn as_ref(&self) -> &MessageReplyToMessage {
    self
  }
}

impl AsRef<MessageReplyToMessage> for RTDMessageReplyToMessageBuilder {
  fn as_ref(&self) -> &MessageReplyToMessage {
    &self.inner
  }
}

/// Describes a story replied by a given message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReplyToStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the poster of the story
  story_poster_chat_id: i64,
  /// The identifier of the story
  story_id: i64,
}

impl RObject for MessageReplyToStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageReplyToStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageReplyTo for MessageReplyToStory {}

impl MessageReplyToStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageReplyToStoryBuilder {
    let inner = MessageReplyToStory {
      td_name: "messageReplyToStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageReplyToStoryBuilder { inner }
  }

  pub fn story_poster_chat_id(&self) -> i64 {
    self.story_poster_chat_id
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDMessageReplyToStoryBuilder {
  inner: MessageReplyToStory,
}

impl RTDMessageReplyToStoryBuilder {
  pub fn build(&self) -> MessageReplyToStory {
    self.inner.clone()
  }

  pub fn story_poster_chat_id(&mut self, story_poster_chat_id: i64) -> &mut Self {
    self.inner.story_poster_chat_id = story_poster_chat_id;
    self
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<MessageReplyToStory> for MessageReplyToStory {
  fn as_ref(&self) -> &MessageReplyToStory {
    self
  }
}

impl AsRef<MessageReplyToStory> for RTDMessageReplyToStoryBuilder {
  fn as_ref(&self) -> &MessageReplyToStory {
    &self.inner
  }
}
