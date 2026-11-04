use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about the time when a scheduled message will be sent
pub trait TDMessageSchedulingState: Debug + RObject {}

/// Contains information about the time when a scheduled message will be sent
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageSchedulingState {
  #[doc(hidden)]
  _Default(()),
  /// The message will be sent at the specified date
  SendAtDate(MessageSchedulingStateSendAtDate),
  /// The message will be sent when the other user is online. Applicable to private chats only and when the exact online status of the other user is known
  SendWhenOnline(MessageSchedulingStateSendWhenOnline),
  /// The message will be sent when the video in the message is converted and optimized; can be used only by the server
  SendWhenVideoProcessed(MessageSchedulingStateSendWhenVideoProcessed),
}

impl Default for MessageSchedulingState {
  fn default() -> Self {
    MessageSchedulingState::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageSchedulingState {
  fn deserialize<D>(deserializer: D) -> Result<MessageSchedulingState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageSchedulingState,
      (messageSchedulingStateSendAtDate, SendAtDate);
      (messageSchedulingStateSendWhenOnline, SendWhenOnline);
      (messageSchedulingStateSendWhenVideoProcessed, SendWhenVideoProcessed);

    )(deserializer)
  }
}

impl RObject for MessageSchedulingState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageSchedulingState::SendAtDate(t) => t.td_name(),
      MessageSchedulingState::SendWhenOnline(t) => t.td_name(),
      MessageSchedulingState::SendWhenVideoProcessed(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageSchedulingState::SendAtDate(t) => t.extra(),
      MessageSchedulingState::SendWhenOnline(t) => t.extra(),
      MessageSchedulingState::SendWhenVideoProcessed(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageSchedulingState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageSchedulingState::_Default(_))
  }

  pub fn is_send_at_date(&self) -> bool {
    matches!(self, MessageSchedulingState::SendAtDate(_))
  }
  pub fn is_send_when_online(&self) -> bool {
    matches!(self, MessageSchedulingState::SendWhenOnline(_))
  }
  pub fn is_send_when_video_processed(&self) -> bool {
    matches!(self, MessageSchedulingState::SendWhenVideoProcessed(_))
  }

  pub fn on_send_at_date<F: FnOnce(&MessageSchedulingStateSendAtDate)>(&self, fnc: F) -> &Self {
    if let MessageSchedulingState::SendAtDate(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_send_when_online<F: FnOnce(&MessageSchedulingStateSendWhenOnline)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let MessageSchedulingState::SendWhenOnline(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_send_when_video_processed<F: FnOnce(&MessageSchedulingStateSendWhenVideoProcessed)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let MessageSchedulingState::SendWhenVideoProcessed(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_send_at_date(&self) -> Option<&MessageSchedulingStateSendAtDate> {
    if let MessageSchedulingState::SendAtDate(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_send_when_online(&self) -> Option<&MessageSchedulingStateSendWhenOnline> {
    if let MessageSchedulingState::SendWhenOnline(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_send_when_video_processed(
    &self,
  ) -> Option<&MessageSchedulingStateSendWhenVideoProcessed> {
    if let MessageSchedulingState::SendWhenVideoProcessed(t) = self {
      return Some(t);
    }
    None
  }

  pub fn send_at_date<T: AsRef<MessageSchedulingStateSendAtDate>>(t: T) -> Self {
    MessageSchedulingState::SendAtDate(t.as_ref().clone())
  }

  pub fn send_when_online<T: AsRef<MessageSchedulingStateSendWhenOnline>>(t: T) -> Self {
    MessageSchedulingState::SendWhenOnline(t.as_ref().clone())
  }

  pub fn send_when_video_processed<T: AsRef<MessageSchedulingStateSendWhenVideoProcessed>>(
    t: T,
  ) -> Self {
    MessageSchedulingState::SendWhenVideoProcessed(t.as_ref().clone())
  }
}

impl AsRef<MessageSchedulingState> for MessageSchedulingState {
  fn as_ref(&self) -> &MessageSchedulingState {
    self
  }
}

/// The message will be sent at the specified date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSchedulingStateSendAtDate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the message will be sent. The date must be within 367 days in the future
  send_date: i64,
  /// Period after which the message will be sent again; in seconds; 0 if never; for Telegram Premium users only; may be non-zero only in sendMessage and forwardMessages with one message requests; must be one of 0, 86400, 7 * 86400, 14 * 86400, 30 * 86400, 91 * 86400, 182 * 86400, 365 * 86400, or additionally 60, or 300 in the Test DC
  repeat_period: i64,
}

impl RObject for MessageSchedulingStateSendAtDate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSchedulingStateSendAtDate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSchedulingState for MessageSchedulingStateSendAtDate {}

impl MessageSchedulingStateSendAtDate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSchedulingStateSendAtDateBuilder {
    let inner = MessageSchedulingStateSendAtDate {
      td_name: "messageSchedulingStateSendAtDate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageSchedulingStateSendAtDateBuilder { inner }
  }

  pub fn send_date(&self) -> i64 {
    self.send_date
  }

  pub fn repeat_period(&self) -> i64 {
    self.repeat_period
  }
}

#[doc(hidden)]
pub struct RTDMessageSchedulingStateSendAtDateBuilder {
  inner: MessageSchedulingStateSendAtDate,
}

impl RTDMessageSchedulingStateSendAtDateBuilder {
  pub fn build(&self) -> MessageSchedulingStateSendAtDate {
    self.inner.clone()
  }

  pub fn send_date(&mut self, send_date: i64) -> &mut Self {
    self.inner.send_date = send_date;
    self
  }

  pub fn repeat_period(&mut self, repeat_period: i64) -> &mut Self {
    self.inner.repeat_period = repeat_period;
    self
  }
}

impl AsRef<MessageSchedulingStateSendAtDate> for MessageSchedulingStateSendAtDate {
  fn as_ref(&self) -> &MessageSchedulingStateSendAtDate {
    self
  }
}

impl AsRef<MessageSchedulingStateSendAtDate> for RTDMessageSchedulingStateSendAtDateBuilder {
  fn as_ref(&self) -> &MessageSchedulingStateSendAtDate {
    &self.inner
  }
}

/// The message will be sent when the other user is online. Applicable to private chats only and when the exact online status of the other user is known
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSchedulingStateSendWhenOnline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSchedulingStateSendWhenOnline {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSchedulingStateSendWhenOnline"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSchedulingState for MessageSchedulingStateSendWhenOnline {}

impl MessageSchedulingStateSendWhenOnline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSchedulingStateSendWhenOnlineBuilder {
    let inner = MessageSchedulingStateSendWhenOnline {
      td_name: "messageSchedulingStateSendWhenOnline".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSchedulingStateSendWhenOnlineBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSchedulingStateSendWhenOnlineBuilder {
  inner: MessageSchedulingStateSendWhenOnline,
}

impl RTDMessageSchedulingStateSendWhenOnlineBuilder {
  pub fn build(&self) -> MessageSchedulingStateSendWhenOnline {
    self.inner.clone()
  }
}

impl AsRef<MessageSchedulingStateSendWhenOnline> for MessageSchedulingStateSendWhenOnline {
  fn as_ref(&self) -> &MessageSchedulingStateSendWhenOnline {
    self
  }
}

impl AsRef<MessageSchedulingStateSendWhenOnline>
  for RTDMessageSchedulingStateSendWhenOnlineBuilder
{
  fn as_ref(&self) -> &MessageSchedulingStateSendWhenOnline {
    &self.inner
  }
}

/// The message will be sent when the video in the message is converted and optimized; can be used only by the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSchedulingStateSendWhenVideoProcessed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate point in time (Unix timestamp) when the message is expected to be sent
  send_date: i64,
}

impl RObject for MessageSchedulingStateSendWhenVideoProcessed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSchedulingStateSendWhenVideoProcessed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSchedulingState for MessageSchedulingStateSendWhenVideoProcessed {}

impl MessageSchedulingStateSendWhenVideoProcessed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSchedulingStateSendWhenVideoProcessedBuilder {
    let inner = MessageSchedulingStateSendWhenVideoProcessed {
      td_name: "messageSchedulingStateSendWhenVideoProcessed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageSchedulingStateSendWhenVideoProcessedBuilder { inner }
  }

  pub fn send_date(&self) -> i64 {
    self.send_date
  }
}

#[doc(hidden)]
pub struct RTDMessageSchedulingStateSendWhenVideoProcessedBuilder {
  inner: MessageSchedulingStateSendWhenVideoProcessed,
}

impl RTDMessageSchedulingStateSendWhenVideoProcessedBuilder {
  pub fn build(&self) -> MessageSchedulingStateSendWhenVideoProcessed {
    self.inner.clone()
  }

  pub fn send_date(&mut self, send_date: i64) -> &mut Self {
    self.inner.send_date = send_date;
    self
  }
}

impl AsRef<MessageSchedulingStateSendWhenVideoProcessed>
  for MessageSchedulingStateSendWhenVideoProcessed
{
  fn as_ref(&self) -> &MessageSchedulingStateSendWhenVideoProcessed {
    self
  }
}

impl AsRef<MessageSchedulingStateSendWhenVideoProcessed>
  for RTDMessageSchedulingStateSendWhenVideoProcessedBuilder
{
  fn as_ref(&self) -> &MessageSchedulingStateSendWhenVideoProcessed {
    &self.inner
  }
}
