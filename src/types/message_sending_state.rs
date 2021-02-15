
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about the sending state of the message
pub trait TDMessageSendingState: Debug + RObject {}

/// Contains information about the sending state of the message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageSendingState {
  #[doc(hidden)] _Default(()),
  /// The message failed to be sent
  Failed(MessageSendingStateFailed),
  /// The message is being sent now, but has not yet been delivered to the server
  Pending(MessageSendingStatePending),

}

impl Default for MessageSendingState {
  fn default() -> Self { MessageSendingState::_Default(()) }
}

impl<'de> Deserialize<'de> for MessageSendingState {
  fn deserialize<D>(deserializer: D) -> Result<MessageSendingState, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageSendingState,
      (messageSendingStateFailed, Failed);
      (messageSendingStatePending, Pending);

    )(deserializer)
  }
}

impl RObject for MessageSendingState {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      MessageSendingState::Failed(t) => t.td_name(),
      MessageSendingState::Pending(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      MessageSendingState::Failed(t) => t.extra(),
      MessageSendingState::Pending(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl MessageSendingState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let MessageSendingState::_Default(_) = self { true } else { false } }

  pub fn is_failed(&self) -> bool { if let MessageSendingState::Failed(_) = self { true } else { false } }
  pub fn is_pending(&self) -> bool { if let MessageSendingState::Pending(_) = self { true } else { false } }

  pub fn on_failed<F: FnOnce(&MessageSendingStateFailed)>(&self, fnc: F) -> &Self { if let MessageSendingState::Failed(t) = self { fnc(t) }; self }
  pub fn on_pending<F: FnOnce(&MessageSendingStatePending)>(&self, fnc: F) -> &Self { if let MessageSendingState::Pending(t) = self { fnc(t) }; self }

  pub fn as_failed(&self) -> Option<&MessageSendingStateFailed> { if let MessageSendingState::Failed(t) = self { return Some(t) } None }
  pub fn as_pending(&self) -> Option<&MessageSendingStatePending> { if let MessageSendingState::Pending(t) = self { return Some(t) } None }



  pub fn failed<T: AsRef<MessageSendingStateFailed>>(t: T) -> Self { MessageSendingState::Failed(t.as_ref().clone()) }

  pub fn pending<T: AsRef<MessageSendingStatePending>>(t: T) -> Self { MessageSendingState::Pending(t.as_ref().clone()) }

}

impl AsRef<MessageSendingState> for MessageSendingState {
  fn as_ref(&self) -> &MessageSendingState { self }
}







/// The message failed to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSendingStateFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for MessageSendingStateFailed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSendingStateFailed" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageSendingState for MessageSendingStateFailed {}



impl MessageSendingStateFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSendingStateFailedBuilder {
    let mut inner = MessageSendingStateFailed::default();
    inner.td_name = "messageSendingStateFailed".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageSendingStateFailedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageSendingStateFailedBuilder {
  inner: MessageSendingStateFailed
}

impl RTDMessageSendingStateFailedBuilder {
  pub fn build(&self) -> MessageSendingStateFailed { self.inner.clone() }

}

impl AsRef<MessageSendingStateFailed> for MessageSendingStateFailed {
  fn as_ref(&self) -> &MessageSendingStateFailed { self }
}

impl AsRef<MessageSendingStateFailed> for RTDMessageSendingStateFailedBuilder {
  fn as_ref(&self) -> &MessageSendingStateFailed { &self.inner }
}







/// The message is being sent now, but has not yet been delivered to the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSendingStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for MessageSendingStatePending {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSendingStatePending" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageSendingState for MessageSendingStatePending {}



impl MessageSendingStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSendingStatePendingBuilder {
    let mut inner = MessageSendingStatePending::default();
    inner.td_name = "messageSendingStatePending".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageSendingStatePendingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageSendingStatePendingBuilder {
  inner: MessageSendingStatePending
}

impl RTDMessageSendingStatePendingBuilder {
  pub fn build(&self) -> MessageSendingStatePending { self.inner.clone() }

}

impl AsRef<MessageSendingStatePending> for MessageSendingStatePending {
  fn as_ref(&self) -> &MessageSendingStatePending { self }
}

impl AsRef<MessageSendingStatePending> for RTDMessageSendingStatePendingBuilder {
  fn as_ref(&self) -> &MessageSendingStatePending { &self.inner }
}



