use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about the origin of a message
pub trait TDMessageOrigin: Debug + RObject {}

/// Contains information about the origin of a message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageOrigin {
  #[doc(hidden)]
  _Default(()),
  /// The message was originally a post in a channel
  Channel(MessageOriginChannel),
  /// The message was originally sent on behalf of a chat
  Chat(MessageOriginChat),
  /// The message was originally sent by a user who is hidden by their privacy settings
  HiddenUser(MessageOriginHiddenUser),
  /// The message was originally sent by a known user
  User(MessageOriginUser),
}

impl Default for MessageOrigin {
  fn default() -> Self {
    MessageOrigin::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageOrigin {
  fn deserialize<D>(deserializer: D) -> Result<MessageOrigin, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageOrigin,
      (messageOriginChannel, Channel);
      (messageOriginChat, Chat);
      (messageOriginHiddenUser, HiddenUser);
      (messageOriginUser, User);

    )(deserializer)
  }
}

impl RObject for MessageOrigin {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageOrigin::Channel(t) => t.td_name(),
      MessageOrigin::Chat(t) => t.td_name(),
      MessageOrigin::HiddenUser(t) => t.td_name(),
      MessageOrigin::User(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageOrigin::Channel(t) => t.extra(),
      MessageOrigin::Chat(t) => t.extra(),
      MessageOrigin::HiddenUser(t) => t.extra(),
      MessageOrigin::User(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageOrigin {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageOrigin::_Default(_))
  }

  pub fn is_channel(&self) -> bool {
    matches!(self, MessageOrigin::Channel(_))
  }
  pub fn is_chat(&self) -> bool {
    matches!(self, MessageOrigin::Chat(_))
  }
  pub fn is_hidden_user(&self) -> bool {
    matches!(self, MessageOrigin::HiddenUser(_))
  }
  pub fn is_user(&self) -> bool {
    matches!(self, MessageOrigin::User(_))
  }

  pub fn on_channel<F: FnOnce(&MessageOriginChannel)>(&self, fnc: F) -> &Self {
    if let MessageOrigin::Channel(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat<F: FnOnce(&MessageOriginChat)>(&self, fnc: F) -> &Self {
    if let MessageOrigin::Chat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_hidden_user<F: FnOnce(&MessageOriginHiddenUser)>(&self, fnc: F) -> &Self {
    if let MessageOrigin::HiddenUser(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user<F: FnOnce(&MessageOriginUser)>(&self, fnc: F) -> &Self {
    if let MessageOrigin::User(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_channel(&self) -> Option<&MessageOriginChannel> {
    if let MessageOrigin::Channel(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat(&self) -> Option<&MessageOriginChat> {
    if let MessageOrigin::Chat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_hidden_user(&self) -> Option<&MessageOriginHiddenUser> {
    if let MessageOrigin::HiddenUser(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user(&self) -> Option<&MessageOriginUser> {
    if let MessageOrigin::User(t) = self {
      return Some(t);
    }
    None
  }

  pub fn channel<T: AsRef<MessageOriginChannel>>(t: T) -> Self {
    MessageOrigin::Channel(t.as_ref().clone())
  }

  pub fn chat<T: AsRef<MessageOriginChat>>(t: T) -> Self {
    MessageOrigin::Chat(t.as_ref().clone())
  }

  pub fn hidden_user<T: AsRef<MessageOriginHiddenUser>>(t: T) -> Self {
    MessageOrigin::HiddenUser(t.as_ref().clone())
  }

  pub fn user<T: AsRef<MessageOriginUser>>(t: T) -> Self {
    MessageOrigin::User(t.as_ref().clone())
  }
}

impl AsRef<MessageOrigin> for MessageOrigin {
  fn as_ref(&self) -> &MessageOrigin {
    self
  }
}

/// The message was originally a post in a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageOriginChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel chat to which the message was originally sent
  chat_id: i64,
  /// Message identifier of the original message
  message_id: i64,
  /// Original post author signature
  author_signature: String,
}

impl RObject for MessageOriginChannel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageOriginChannel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageOrigin for MessageOriginChannel {}

impl MessageOriginChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageOriginChannelBuilder {
    let inner = MessageOriginChannel {
      td_name: "messageOriginChannel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageOriginChannelBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn author_signature(&self) -> &String {
    &self.author_signature
  }
}

#[doc(hidden)]
pub struct RTDMessageOriginChannelBuilder {
  inner: MessageOriginChannel,
}

impl RTDMessageOriginChannelBuilder {
  pub fn build(&self) -> MessageOriginChannel {
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

  pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
    self.inner.author_signature = author_signature.as_ref().to_string();
    self
  }
}

impl AsRef<MessageOriginChannel> for MessageOriginChannel {
  fn as_ref(&self) -> &MessageOriginChannel {
    self
  }
}

impl AsRef<MessageOriginChannel> for RTDMessageOriginChannelBuilder {
  fn as_ref(&self) -> &MessageOriginChannel {
    &self.inner
  }
}

/// The message was originally sent on behalf of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageOriginChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that originally sent the message
  sender_chat_id: i64,
  /// For messages originally sent by an anonymous chat administrator, original message author signature
  author_signature: String,
}

impl RObject for MessageOriginChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageOriginChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageOrigin for MessageOriginChat {}

impl MessageOriginChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageOriginChatBuilder {
    let inner = MessageOriginChat {
      td_name: "messageOriginChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageOriginChatBuilder { inner }
  }

  pub fn sender_chat_id(&self) -> i64 {
    self.sender_chat_id
  }

  pub fn author_signature(&self) -> &String {
    &self.author_signature
  }
}

#[doc(hidden)]
pub struct RTDMessageOriginChatBuilder {
  inner: MessageOriginChat,
}

impl RTDMessageOriginChatBuilder {
  pub fn build(&self) -> MessageOriginChat {
    self.inner.clone()
  }

  pub fn sender_chat_id(&mut self, sender_chat_id: i64) -> &mut Self {
    self.inner.sender_chat_id = sender_chat_id;
    self
  }

  pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
    self.inner.author_signature = author_signature.as_ref().to_string();
    self
  }
}

impl AsRef<MessageOriginChat> for MessageOriginChat {
  fn as_ref(&self) -> &MessageOriginChat {
    self
  }
}

impl AsRef<MessageOriginChat> for RTDMessageOriginChatBuilder {
  fn as_ref(&self) -> &MessageOriginChat {
    &self.inner
  }
}

/// The message was originally sent by a user who is hidden by their privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageOriginHiddenUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the sender
  sender_name: String,
}

impl RObject for MessageOriginHiddenUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageOriginHiddenUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageOrigin for MessageOriginHiddenUser {}

impl MessageOriginHiddenUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageOriginHiddenUserBuilder {
    let inner = MessageOriginHiddenUser {
      td_name: "messageOriginHiddenUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageOriginHiddenUserBuilder { inner }
  }

  pub fn sender_name(&self) -> &String {
    &self.sender_name
  }
}

#[doc(hidden)]
pub struct RTDMessageOriginHiddenUserBuilder {
  inner: MessageOriginHiddenUser,
}

impl RTDMessageOriginHiddenUserBuilder {
  pub fn build(&self) -> MessageOriginHiddenUser {
    self.inner.clone()
  }

  pub fn sender_name<T: AsRef<str>>(&mut self, sender_name: T) -> &mut Self {
    self.inner.sender_name = sender_name.as_ref().to_string();
    self
  }
}

impl AsRef<MessageOriginHiddenUser> for MessageOriginHiddenUser {
  fn as_ref(&self) -> &MessageOriginHiddenUser {
    self
  }
}

impl AsRef<MessageOriginHiddenUser> for RTDMessageOriginHiddenUserBuilder {
  fn as_ref(&self) -> &MessageOriginHiddenUser {
    &self.inner
  }
}

/// The message was originally sent by a known user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageOriginUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who originally sent the message
  sender_user_id: i64,
}

impl RObject for MessageOriginUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageOriginUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageOrigin for MessageOriginUser {}

impl MessageOriginUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageOriginUserBuilder {
    let inner = MessageOriginUser {
      td_name: "messageOriginUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageOriginUserBuilder { inner }
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }
}

#[doc(hidden)]
pub struct RTDMessageOriginUserBuilder {
  inner: MessageOriginUser,
}

impl RTDMessageOriginUserBuilder {
  pub fn build(&self) -> MessageOriginUser {
    self.inner.clone()
  }

  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }
}

impl AsRef<MessageOriginUser> for MessageOriginUser {
  fn as_ref(&self) -> &MessageOriginUser {
    self
  }
}

impl AsRef<MessageOriginUser> for RTDMessageOriginUserBuilder {
  fn as_ref(&self) -> &MessageOriginUser {
    &self.inner
  }
}
