use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a call
pub trait TDInputCall: Debug + RObject {}

/// Describes a call
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputCall {
  #[doc(hidden)]
  _Default(()),
  /// A just ended call
  Discarded(InputCallDiscarded),
  /// A call from a message of the type messageCall with non-zero messageCall.unique_id
  FromMessage(InputCallFromMessage),
}

impl Default for InputCall {
  fn default() -> Self {
    InputCall::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputCall {
  fn deserialize<D>(deserializer: D) -> Result<InputCall, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputCall,
      (inputCallDiscarded, Discarded);
      (inputCallFromMessage, FromMessage);

    )(deserializer)
  }
}

impl RObject for InputCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputCall::Discarded(t) => t.td_name(),
      InputCall::FromMessage(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputCall::Discarded(t) => t.extra(),
      InputCall::FromMessage(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputCall::_Default(_))
  }

  pub fn is_discarded(&self) -> bool {
    matches!(self, InputCall::Discarded(_))
  }
  pub fn is_from_message(&self) -> bool {
    matches!(self, InputCall::FromMessage(_))
  }

  pub fn on_discarded<F: FnOnce(&InputCallDiscarded)>(&self, fnc: F) -> &Self {
    if let InputCall::Discarded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_from_message<F: FnOnce(&InputCallFromMessage)>(&self, fnc: F) -> &Self {
    if let InputCall::FromMessage(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_discarded(&self) -> Option<&InputCallDiscarded> {
    if let InputCall::Discarded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_from_message(&self) -> Option<&InputCallFromMessage> {
    if let InputCall::FromMessage(t) = self {
      return Some(t);
    }
    None
  }

  pub fn discarded<T: AsRef<InputCallDiscarded>>(t: T) -> Self {
    InputCall::Discarded(t.as_ref().clone())
  }

  pub fn from_message<T: AsRef<InputCallFromMessage>>(t: T) -> Self {
    InputCall::FromMessage(t.as_ref().clone())
  }
}

impl AsRef<InputCall> for InputCall {
  fn as_ref(&self) -> &InputCall {
    self
  }
}

/// A just ended call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCallDiscarded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the call
  call_id: i64,
}

impl RObject for InputCallDiscarded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputCallDiscarded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputCall for InputCallDiscarded {}

impl InputCallDiscarded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputCallDiscardedBuilder {
    let inner = InputCallDiscarded {
      td_name: "inputCallDiscarded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputCallDiscardedBuilder { inner }
  }

  pub fn call_id(&self) -> i64 {
    self.call_id
  }
}

#[doc(hidden)]
pub struct RTDInputCallDiscardedBuilder {
  inner: InputCallDiscarded,
}

impl RTDInputCallDiscardedBuilder {
  pub fn build(&self) -> InputCallDiscarded {
    self.inner.clone()
  }

  pub fn call_id(&mut self, call_id: i64) -> &mut Self {
    self.inner.call_id = call_id;
    self
  }
}

impl AsRef<InputCallDiscarded> for InputCallDiscarded {
  fn as_ref(&self) -> &InputCallDiscarded {
    self
  }
}

impl AsRef<InputCallDiscarded> for RTDInputCallDiscardedBuilder {
  fn as_ref(&self) -> &InputCallDiscarded {
    &self.inner
  }
}

/// A call from a message of the type messageCall with non-zero messageCall.unique_id
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCallFromMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the message
  chat_id: i64,
  /// Message identifier
  message_id: i64,
}

impl RObject for InputCallFromMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputCallFromMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputCall for InputCallFromMessage {}

impl InputCallFromMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputCallFromMessageBuilder {
    let inner = InputCallFromMessage {
      td_name: "inputCallFromMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputCallFromMessageBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDInputCallFromMessageBuilder {
  inner: InputCallFromMessage,
}

impl RTDInputCallFromMessageBuilder {
  pub fn build(&self) -> InputCallFromMessage {
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

impl AsRef<InputCallFromMessage> for InputCallFromMessage {
  fn as_ref(&self) -> &InputCallFromMessage {
    self
  }
}

impl AsRef<InputCallFromMessage> for RTDInputCallFromMessageBuilder {
  fn as_ref(&self) -> &InputCallFromMessage {
    &self.inner
  }
}
