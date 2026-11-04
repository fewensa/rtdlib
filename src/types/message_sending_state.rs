use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about the sending state of the message
pub trait TDMessageSendingState: Debug + RObject {}

/// Contains information about the sending state of the message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageSendingState {
  #[doc(hidden)]
  _Default(()),
  /// The message failed to be sent
  Failed(MessageSendingStateFailed),
  /// The message is being sent now, but has not yet been delivered to the server
  Pending(MessageSendingStatePending),
}

impl Default for MessageSendingState {
  fn default() -> Self {
    MessageSendingState::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageSendingState {
  fn deserialize<D>(deserializer: D) -> Result<MessageSendingState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageSendingState,
      (messageSendingStateFailed, Failed);
      (messageSendingStatePending, Pending);

    )(deserializer)
  }
}

impl RObject for MessageSendingState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageSendingState::Failed(t) => t.td_name(),
      MessageSendingState::Pending(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageSendingState::Failed(t) => t.extra(),
      MessageSendingState::Pending(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageSendingState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageSendingState::_Default(_))
  }

  pub fn is_failed(&self) -> bool {
    matches!(self, MessageSendingState::Failed(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, MessageSendingState::Pending(_))
  }

  pub fn on_failed<F: FnOnce(&MessageSendingStateFailed)>(&self, fnc: F) -> &Self {
    if let MessageSendingState::Failed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&MessageSendingStatePending)>(&self, fnc: F) -> &Self {
    if let MessageSendingState::Pending(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_failed(&self) -> Option<&MessageSendingStateFailed> {
    if let MessageSendingState::Failed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&MessageSendingStatePending> {
    if let MessageSendingState::Pending(t) = self {
      return Some(t);
    }
    None
  }

  pub fn failed<T: AsRef<MessageSendingStateFailed>>(t: T) -> Self {
    MessageSendingState::Failed(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<MessageSendingStatePending>>(t: T) -> Self {
    MessageSendingState::Pending(t.as_ref().clone())
  }
}

impl AsRef<MessageSendingState> for MessageSendingState {
  fn as_ref(&self) -> &MessageSendingState {
    self
  }
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
  /// The cause of the message sending failure
  error: Error,
  /// True, if the message can be re-sent using resendMessages or readdQuickReplyShortcutMessages
  can_retry: bool,
  /// True, if the message can be re-sent only on behalf of a different sender
  need_another_sender: bool,
  /// True, if the message can be re-sent only if another quote is chosen in the message that is replied by the given message
  need_another_reply_quote: bool,
  /// True, if the message can be re-sent only if the message to be replied is removed. This will be done automatically by resendMessages
  need_drop_reply: bool,
  /// The number of Telegram Stars that must be paid to send the message; 0 if the current amount is correct
  required_paid_message_star_count: i64,
  /// Time left before the message can be re-sent, in seconds. No update is sent when this field changes
  retry_after: f32,
}

impl RObject for MessageSendingStateFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSendingStateFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSendingState for MessageSendingStateFailed {}

impl MessageSendingStateFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSendingStateFailedBuilder {
    let inner = MessageSendingStateFailed {
      td_name: "messageSendingStateFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageSendingStateFailedBuilder { inner }
  }

  pub fn error(&self) -> &Error {
    &self.error
  }

  pub fn can_retry(&self) -> bool {
    self.can_retry
  }

  pub fn need_another_sender(&self) -> bool {
    self.need_another_sender
  }

  pub fn need_another_reply_quote(&self) -> bool {
    self.need_another_reply_quote
  }

  pub fn need_drop_reply(&self) -> bool {
    self.need_drop_reply
  }

  pub fn required_paid_message_star_count(&self) -> i64 {
    self.required_paid_message_star_count
  }

  pub fn retry_after(&self) -> f32 {
    self.retry_after
  }
}

#[doc(hidden)]
pub struct RTDMessageSendingStateFailedBuilder {
  inner: MessageSendingStateFailed,
}

impl RTDMessageSendingStateFailedBuilder {
  pub fn build(&self) -> MessageSendingStateFailed {
    self.inner.clone()
  }

  pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
    self.inner.error = error.as_ref().clone();
    self
  }

  pub fn can_retry(&mut self, can_retry: bool) -> &mut Self {
    self.inner.can_retry = can_retry;
    self
  }

  pub fn need_another_sender(&mut self, need_another_sender: bool) -> &mut Self {
    self.inner.need_another_sender = need_another_sender;
    self
  }

  pub fn need_another_reply_quote(&mut self, need_another_reply_quote: bool) -> &mut Self {
    self.inner.need_another_reply_quote = need_another_reply_quote;
    self
  }

  pub fn need_drop_reply(&mut self, need_drop_reply: bool) -> &mut Self {
    self.inner.need_drop_reply = need_drop_reply;
    self
  }

  pub fn required_paid_message_star_count(
    &mut self,
    required_paid_message_star_count: i64,
  ) -> &mut Self {
    self.inner.required_paid_message_star_count = required_paid_message_star_count;
    self
  }

  pub fn retry_after(&mut self, retry_after: f32) -> &mut Self {
    self.inner.retry_after = retry_after;
    self
  }
}

impl AsRef<MessageSendingStateFailed> for MessageSendingStateFailed {
  fn as_ref(&self) -> &MessageSendingStateFailed {
    self
  }
}

impl AsRef<MessageSendingStateFailed> for RTDMessageSendingStateFailedBuilder {
  fn as_ref(&self) -> &MessageSendingStateFailed {
    &self.inner
  }
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
  /// Non-persistent message sending identifier, specified by the application
  sending_id: i64,
}

impl RObject for MessageSendingStatePending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSendingStatePending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSendingState for MessageSendingStatePending {}

impl MessageSendingStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSendingStatePendingBuilder {
    let inner = MessageSendingStatePending {
      td_name: "messageSendingStatePending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageSendingStatePendingBuilder { inner }
  }

  pub fn sending_id(&self) -> i64 {
    self.sending_id
  }
}

#[doc(hidden)]
pub struct RTDMessageSendingStatePendingBuilder {
  inner: MessageSendingStatePending,
}

impl RTDMessageSendingStatePendingBuilder {
  pub fn build(&self) -> MessageSendingStatePending {
    self.inner.clone()
  }

  pub fn sending_id(&mut self, sending_id: i64) -> &mut Self {
    self.inner.sending_id = sending_id;
    self
  }
}

impl AsRef<MessageSendingStatePending> for MessageSendingStatePending {
  fn as_ref(&self) -> &MessageSendingStatePending {
    self
  }
}

impl AsRef<MessageSendingStatePending> for RTDMessageSendingStatePendingBuilder {
  fn as_ref(&self) -> &MessageSendingStatePending {
    &self.inner
  }
}
