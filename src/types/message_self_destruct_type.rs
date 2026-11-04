use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes when a message will be self-destructed
pub trait TDMessageSelfDestructType: Debug + RObject {}

/// Describes when a message will be self-destructed
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageSelfDestructType {
  #[doc(hidden)]
  _Default(()),
  /// The message can be opened only once and will be self-destructed once closed
  Immediately(MessageSelfDestructTypeImmediately),
  /// The message will be self-destructed in the specified time after its content was opened
  Timer(MessageSelfDestructTypeTimer),
}

impl Default for MessageSelfDestructType {
  fn default() -> Self {
    MessageSelfDestructType::_Default(())
  }
}

impl<'de> Deserialize<'de> for MessageSelfDestructType {
  fn deserialize<D>(deserializer: D) -> Result<MessageSelfDestructType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageSelfDestructType,
      (messageSelfDestructTypeImmediately, Immediately);
      (messageSelfDestructTypeTimer, Timer);

    )(deserializer)
  }
}

impl RObject for MessageSelfDestructType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      MessageSelfDestructType::Immediately(t) => t.td_name(),
      MessageSelfDestructType::Timer(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      MessageSelfDestructType::Immediately(t) => t.extra(),
      MessageSelfDestructType::Timer(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageSelfDestructType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, MessageSelfDestructType::_Default(_))
  }

  pub fn is_immediately(&self) -> bool {
    matches!(self, MessageSelfDestructType::Immediately(_))
  }
  pub fn is_timer(&self) -> bool {
    matches!(self, MessageSelfDestructType::Timer(_))
  }

  pub fn on_immediately<F: FnOnce(&MessageSelfDestructTypeImmediately)>(&self, fnc: F) -> &Self {
    if let MessageSelfDestructType::Immediately(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_timer<F: FnOnce(&MessageSelfDestructTypeTimer)>(&self, fnc: F) -> &Self {
    if let MessageSelfDestructType::Timer(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_immediately(&self) -> Option<&MessageSelfDestructTypeImmediately> {
    if let MessageSelfDestructType::Immediately(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_timer(&self) -> Option<&MessageSelfDestructTypeTimer> {
    if let MessageSelfDestructType::Timer(t) = self {
      return Some(t);
    }
    None
  }

  pub fn immediately<T: AsRef<MessageSelfDestructTypeImmediately>>(t: T) -> Self {
    MessageSelfDestructType::Immediately(t.as_ref().clone())
  }

  pub fn timer<T: AsRef<MessageSelfDestructTypeTimer>>(t: T) -> Self {
    MessageSelfDestructType::Timer(t.as_ref().clone())
  }
}

impl AsRef<MessageSelfDestructType> for MessageSelfDestructType {
  fn as_ref(&self) -> &MessageSelfDestructType {
    self
  }
}

/// The message can be opened only once and will be self-destructed once closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSelfDestructTypeImmediately {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for MessageSelfDestructTypeImmediately {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSelfDestructTypeImmediately"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSelfDestructType for MessageSelfDestructTypeImmediately {}

impl MessageSelfDestructTypeImmediately {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSelfDestructTypeImmediatelyBuilder {
    let inner = MessageSelfDestructTypeImmediately {
      td_name: "messageSelfDestructTypeImmediately".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDMessageSelfDestructTypeImmediatelyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDMessageSelfDestructTypeImmediatelyBuilder {
  inner: MessageSelfDestructTypeImmediately,
}

impl RTDMessageSelfDestructTypeImmediatelyBuilder {
  pub fn build(&self) -> MessageSelfDestructTypeImmediately {
    self.inner.clone()
  }
}

impl AsRef<MessageSelfDestructTypeImmediately> for MessageSelfDestructTypeImmediately {
  fn as_ref(&self) -> &MessageSelfDestructTypeImmediately {
    self
  }
}

impl AsRef<MessageSelfDestructTypeImmediately> for RTDMessageSelfDestructTypeImmediatelyBuilder {
  fn as_ref(&self) -> &MessageSelfDestructTypeImmediately {
    &self.inner
  }
}

/// The message will be self-destructed in the specified time after its content was opened
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSelfDestructTypeTimer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The message's self-destruct time, in seconds; must be between 0 and 60 in private chats
  self_destruct_time: i64,
}

impl RObject for MessageSelfDestructTypeTimer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageSelfDestructTypeTimer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDMessageSelfDestructType for MessageSelfDestructTypeTimer {}

impl MessageSelfDestructTypeTimer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageSelfDestructTypeTimerBuilder {
    let inner = MessageSelfDestructTypeTimer {
      td_name: "messageSelfDestructTypeTimer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageSelfDestructTypeTimerBuilder { inner }
  }

  pub fn self_destruct_time(&self) -> i64 {
    self.self_destruct_time
  }
}

#[doc(hidden)]
pub struct RTDMessageSelfDestructTypeTimerBuilder {
  inner: MessageSelfDestructTypeTimer,
}

impl RTDMessageSelfDestructTypeTimerBuilder {
  pub fn build(&self) -> MessageSelfDestructTypeTimer {
    self.inner.clone()
  }

  pub fn self_destruct_time(&mut self, self_destruct_time: i64) -> &mut Self {
    self.inner.self_destruct_time = self_destruct_time;
    self
  }
}

impl AsRef<MessageSelfDestructTypeTimer> for MessageSelfDestructTypeTimer {
  fn as_ref(&self) -> &MessageSelfDestructTypeTimer {
    self
  }
}

impl AsRef<MessageSelfDestructTypeTimer> for RTDMessageSelfDestructTypeTimerBuilder {
  fn as_ref(&self) -> &MessageSelfDestructTypeTimer {
    &self.inner
  }
}
