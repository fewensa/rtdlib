use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a non-joined group call that isn't bound to a chat
pub trait TDInputGroupCall: Debug + RObject {}

/// Describes a non-joined group call that isn't bound to a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputGroupCall {
  #[doc(hidden)]
  _Default(()),
  /// The group call is accessible through a link
  Link(InputGroupCallLink),
  /// The group call is accessible through a message of the type messageGroupCall
  Message(InputGroupCallMessage),
}

impl Default for InputGroupCall {
  fn default() -> Self {
    InputGroupCall::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputGroupCall {
  fn deserialize<D>(deserializer: D) -> Result<InputGroupCall, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputGroupCall,
      (inputGroupCallLink, Link);
      (inputGroupCallMessage, Message);

    )(deserializer)
  }
}

impl RObject for InputGroupCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputGroupCall::Link(t) => t.td_name(),
      InputGroupCall::Message(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputGroupCall::Link(t) => t.extra(),
      InputGroupCall::Message(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputGroupCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputGroupCall::_Default(_))
  }

  pub fn is_link(&self) -> bool {
    matches!(self, InputGroupCall::Link(_))
  }
  pub fn is_message(&self) -> bool {
    matches!(self, InputGroupCall::Message(_))
  }

  pub fn on_link<F: FnOnce(&InputGroupCallLink)>(&self, fnc: F) -> &Self {
    if let InputGroupCall::Link(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message<F: FnOnce(&InputGroupCallMessage)>(&self, fnc: F) -> &Self {
    if let InputGroupCall::Message(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_link(&self) -> Option<&InputGroupCallLink> {
    if let InputGroupCall::Link(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message(&self) -> Option<&InputGroupCallMessage> {
    if let InputGroupCall::Message(t) = self {
      return Some(t);
    }
    None
  }

  pub fn link<T: AsRef<InputGroupCallLink>>(t: T) -> Self {
    InputGroupCall::Link(t.as_ref().clone())
  }

  pub fn message<T: AsRef<InputGroupCallMessage>>(t: T) -> Self {
    InputGroupCall::Message(t.as_ref().clone())
  }
}

impl AsRef<InputGroupCall> for InputGroupCall {
  fn as_ref(&self) -> &InputGroupCall {
    self
  }
}

/// The group call is accessible through a link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputGroupCallLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The link for the group call
  link: String,
}

impl RObject for InputGroupCallLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputGroupCallLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputGroupCall for InputGroupCallLink {}

impl InputGroupCallLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputGroupCallLinkBuilder {
    let inner = InputGroupCallLink {
      td_name: "inputGroupCallLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputGroupCallLinkBuilder { inner }
  }

  pub fn link(&self) -> &String {
    &self.link
  }
}

#[doc(hidden)]
pub struct RTDInputGroupCallLinkBuilder {
  inner: InputGroupCallLink,
}

impl RTDInputGroupCallLinkBuilder {
  pub fn build(&self) -> InputGroupCallLink {
    self.inner.clone()
  }

  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }
}

impl AsRef<InputGroupCallLink> for InputGroupCallLink {
  fn as_ref(&self) -> &InputGroupCallLink {
    self
  }
}

impl AsRef<InputGroupCallLink> for RTDInputGroupCallLinkBuilder {
  fn as_ref(&self) -> &InputGroupCallLink {
    &self.inner
  }
}

/// The group call is accessible through a message of the type messageGroupCall
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputGroupCallMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat with the message
  chat_id: i64,
  /// Identifier of the message of the type messageGroupCall
  message_id: i64,
}

impl RObject for InputGroupCallMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputGroupCallMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputGroupCall for InputGroupCallMessage {}

impl InputGroupCallMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputGroupCallMessageBuilder {
    let inner = InputGroupCallMessage {
      td_name: "inputGroupCallMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputGroupCallMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDInputGroupCallMessageBuilder {
  inner: InputGroupCallMessage,
}

impl RTDInputGroupCallMessageBuilder {
  pub fn build(&self) -> InputGroupCallMessage {
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
}

impl AsRef<InputGroupCallMessage> for InputGroupCallMessage {
  fn as_ref(&self) -> &InputGroupCallMessage {
    self
  }
}

impl AsRef<InputGroupCallMessage> for RTDInputGroupCallMessageBuilder {
  fn as_ref(&self) -> &InputGroupCallMessage {
    &self.inner
  }
}
