use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about the message or the story to be replied
pub trait TDInputMessageReplyTo: Debug + RObject {}

/// Contains information about the message or the story to be replied
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputMessageReplyTo {
  #[doc(hidden)]
  _Default(()),
  /// Describes an ephemeral message to be replied; for bots only
  EphemeralMessage(InputMessageReplyToEphemeralMessage),
  /// Describes a message to be replied that is from a different chat or a forum topic; not supported in secret chats
  ExternalMessage(InputMessageReplyToExternalMessage),
  /// Describes a message to be replied in the same chat and forum topic
  Message(InputMessageReplyToMessage),
  /// Describes a story to be replied
  Story(InputMessageReplyToStory),
}

impl Default for InputMessageReplyTo {
  fn default() -> Self {
    InputMessageReplyTo::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputMessageReplyTo {
  fn deserialize<D>(deserializer: D) -> Result<InputMessageReplyTo, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputMessageReplyTo,
      (inputMessageReplyToEphemeralMessage, EphemeralMessage);
      (inputMessageReplyToExternalMessage, ExternalMessage);
      (inputMessageReplyToMessage, Message);
      (inputMessageReplyToStory, Story);

    )(deserializer)
  }
}

impl RObject for InputMessageReplyTo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputMessageReplyTo::EphemeralMessage(t) => t.td_name(),
      InputMessageReplyTo::ExternalMessage(t) => t.td_name(),
      InputMessageReplyTo::Message(t) => t.td_name(),
      InputMessageReplyTo::Story(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputMessageReplyTo::EphemeralMessage(t) => t.extra(),
      InputMessageReplyTo::ExternalMessage(t) => t.extra(),
      InputMessageReplyTo::Message(t) => t.extra(),
      InputMessageReplyTo::Story(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputMessageReplyTo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputMessageReplyTo::_Default(_))
  }

  pub fn is_ephemeral_message(&self) -> bool {
    matches!(self, InputMessageReplyTo::EphemeralMessage(_))
  }
  pub fn is_external_message(&self) -> bool {
    matches!(self, InputMessageReplyTo::ExternalMessage(_))
  }
  pub fn is_message(&self) -> bool {
    matches!(self, InputMessageReplyTo::Message(_))
  }
  pub fn is_story(&self) -> bool {
    matches!(self, InputMessageReplyTo::Story(_))
  }

  pub fn on_ephemeral_message<F: FnOnce(&InputMessageReplyToEphemeralMessage)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InputMessageReplyTo::EphemeralMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_external_message<F: FnOnce(&InputMessageReplyToExternalMessage)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InputMessageReplyTo::ExternalMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message<F: FnOnce(&InputMessageReplyToMessage)>(&self, fnc: F) -> &Self {
    if let InputMessageReplyTo::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story<F: FnOnce(&InputMessageReplyToStory)>(&self, fnc: F) -> &Self {
    if let InputMessageReplyTo::Story(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_ephemeral_message(&self) -> Option<&InputMessageReplyToEphemeralMessage> {
    if let InputMessageReplyTo::EphemeralMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_external_message(&self) -> Option<&InputMessageReplyToExternalMessage> {
    if let InputMessageReplyTo::ExternalMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message(&self) -> Option<&InputMessageReplyToMessage> {
    if let InputMessageReplyTo::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story(&self) -> Option<&InputMessageReplyToStory> {
    if let InputMessageReplyTo::Story(t) = self {
      return Some(t);
    }
    None
  }

  pub fn ephemeral_message<T: AsRef<InputMessageReplyToEphemeralMessage>>(t: T) -> Self {
    InputMessageReplyTo::EphemeralMessage(t.as_ref().clone())
  }

  pub fn external_message<T: AsRef<InputMessageReplyToExternalMessage>>(t: T) -> Self {
    InputMessageReplyTo::ExternalMessage(t.as_ref().clone())
  }

  pub fn message<T: AsRef<InputMessageReplyToMessage>>(t: T) -> Self {
    InputMessageReplyTo::Message(t.as_ref().clone())
  }

  pub fn story<T: AsRef<InputMessageReplyToStory>>(t: T) -> Self {
    InputMessageReplyTo::Story(t.as_ref().clone())
  }
}

impl AsRef<InputMessageReplyTo> for InputMessageReplyTo {
  fn as_ref(&self) -> &InputMessageReplyTo {
    self
  }
}

/// Describes an ephemeral message to be replied; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageReplyToEphemeralMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the ephemeral message to be replied
  ephemeral_message_id: i64,
}

impl RObject for InputMessageReplyToEphemeralMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageReplyToEphemeralMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageReplyTo for InputMessageReplyToEphemeralMessage {}

impl InputMessageReplyToEphemeralMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageReplyToEphemeralMessageBuilder {
    let inner = InputMessageReplyToEphemeralMessage {
      td_name: "inputMessageReplyToEphemeralMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageReplyToEphemeralMessageBuilder { inner }
  }

  pub fn ephemeral_message_id(&self) -> i64 {
    self.ephemeral_message_id
  }
}

#[doc(hidden)]
pub struct RTDInputMessageReplyToEphemeralMessageBuilder {
  inner: InputMessageReplyToEphemeralMessage,
}

impl RTDInputMessageReplyToEphemeralMessageBuilder {
  pub fn build(&self) -> InputMessageReplyToEphemeralMessage {
    self.inner.clone()
  }

  pub fn ephemeral_message_id(&mut self, ephemeral_message_id: i64) -> &mut Self {
    self.inner.ephemeral_message_id = ephemeral_message_id;
    self
  }
}

impl AsRef<InputMessageReplyToEphemeralMessage> for InputMessageReplyToEphemeralMessage {
  fn as_ref(&self) -> &InputMessageReplyToEphemeralMessage {
    self
  }
}

impl AsRef<InputMessageReplyToEphemeralMessage> for RTDInputMessageReplyToEphemeralMessageBuilder {
  fn as_ref(&self) -> &InputMessageReplyToEphemeralMessage {
    &self.inner
  }
}

/// Describes a message to be replied that is from a different chat or a forum topic; not supported in secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageReplyToExternalMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the chat to which the message to be replied belongs
  chat_id: i64,
  /// The identifier of the message to be replied in the specified chat. A message can be replied in another chat or forum topic only if messageProperties.can_be_replied_in_another_chat
  message_id: i64,
  /// Quote from the message to be replied; pass null if none
  quote: InputTextQuote,
  /// Identifier of the checklist task in the message to be replied; pass 0 to reply to the whole message
  checklist_task_id: i64,
  /// Identifier of the poll option in the message to be replied; pass an empty string if none
  poll_option_id: String,
}

impl RObject for InputMessageReplyToExternalMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageReplyToExternalMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageReplyTo for InputMessageReplyToExternalMessage {}

impl InputMessageReplyToExternalMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageReplyToExternalMessageBuilder {
    let inner = InputMessageReplyToExternalMessage {
      td_name: "inputMessageReplyToExternalMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageReplyToExternalMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn quote(&self) -> &InputTextQuote {
    &self.quote
  }

  pub fn checklist_task_id(&self) -> i64 {
    self.checklist_task_id
  }

  pub fn poll_option_id(&self) -> &String {
    &self.poll_option_id
  }
}

#[doc(hidden)]
pub struct RTDInputMessageReplyToExternalMessageBuilder {
  inner: InputMessageReplyToExternalMessage,
}

impl RTDInputMessageReplyToExternalMessageBuilder {
  pub fn build(&self) -> InputMessageReplyToExternalMessage {
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

  pub fn quote<T: AsRef<InputTextQuote>>(&mut self, quote: T) -> &mut Self {
    self.inner.quote = quote.as_ref().clone();
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
}

impl AsRef<InputMessageReplyToExternalMessage> for InputMessageReplyToExternalMessage {
  fn as_ref(&self) -> &InputMessageReplyToExternalMessage {
    self
  }
}

impl AsRef<InputMessageReplyToExternalMessage> for RTDInputMessageReplyToExternalMessageBuilder {
  fn as_ref(&self) -> &InputMessageReplyToExternalMessage {
    &self.inner
  }
}

/// Describes a message to be replied in the same chat and forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageReplyToMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the message to be replied in the same chat and forum topic. A message can be replied in the same chat and forum topic only if messageProperties.can_be_replied
  message_id: i64,
  /// Quote from the message to be replied; pass null if none. Must always be null for replies in secret chats
  quote: InputTextQuote,
  /// Identifier of the checklist task in the message to be replied; pass 0 to reply to the whole message
  checklist_task_id: i64,
  /// Identifier of the poll option in the message to be replied; pass an empty string if none
  poll_option_id: String,
}

impl RObject for InputMessageReplyToMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageReplyToMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageReplyTo for InputMessageReplyToMessage {}

impl InputMessageReplyToMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageReplyToMessageBuilder {
    let inner = InputMessageReplyToMessage {
      td_name: "inputMessageReplyToMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageReplyToMessageBuilder { inner }
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn quote(&self) -> &InputTextQuote {
    &self.quote
  }

  pub fn checklist_task_id(&self) -> i64 {
    self.checklist_task_id
  }

  pub fn poll_option_id(&self) -> &String {
    &self.poll_option_id
  }
}

#[doc(hidden)]
pub struct RTDInputMessageReplyToMessageBuilder {
  inner: InputMessageReplyToMessage,
}

impl RTDInputMessageReplyToMessageBuilder {
  pub fn build(&self) -> InputMessageReplyToMessage {
    self.inner.clone()
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn quote<T: AsRef<InputTextQuote>>(&mut self, quote: T) -> &mut Self {
    self.inner.quote = quote.as_ref().clone();
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
}

impl AsRef<InputMessageReplyToMessage> for InputMessageReplyToMessage {
  fn as_ref(&self) -> &InputMessageReplyToMessage {
    self
  }
}

impl AsRef<InputMessageReplyToMessage> for RTDInputMessageReplyToMessageBuilder {
  fn as_ref(&self) -> &InputMessageReplyToMessage {
    &self.inner
  }
}

/// Describes a story to be replied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageReplyToStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the poster of the story. Currently, stories can be replied only in the chat that posted the story; channel stories can't be replied
  story_poster_chat_id: i64,
  /// The identifier of the story
  story_id: i64,
}

impl RObject for InputMessageReplyToStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageReplyToStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageReplyTo for InputMessageReplyToStory {}

impl InputMessageReplyToStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageReplyToStoryBuilder {
    let inner = InputMessageReplyToStory {
      td_name: "inputMessageReplyToStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageReplyToStoryBuilder { inner }
  }

  pub fn story_poster_chat_id(&self) -> i64 {
    self.story_poster_chat_id
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDInputMessageReplyToStoryBuilder {
  inner: InputMessageReplyToStory,
}

impl RTDInputMessageReplyToStoryBuilder {
  pub fn build(&self) -> InputMessageReplyToStory {
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

impl AsRef<InputMessageReplyToStory> for InputMessageReplyToStory {
  fn as_ref(&self) -> &InputMessageReplyToStory {
    self
  }
}

impl AsRef<InputMessageReplyToStory> for RTDInputMessageReplyToStoryBuilder {
  fn as_ref(&self) -> &InputMessageReplyToStory {
    &self.inner
  }
}
