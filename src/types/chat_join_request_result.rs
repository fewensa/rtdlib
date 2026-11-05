use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of a chat join request
pub trait TDChatJoinRequestResult: Debug + RObject {}

/// Describes result of a chat join request
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatJoinRequestResult {
  #[doc(hidden)]
  _Default(()),
  /// The request was approved
  Approved(ChatJoinRequestResultApproved),
  /// The request was declined
  Declined(ChatJoinRequestResultDeclined),
  /// The request was postponed without a decision
  Queued(ChatJoinRequestResultQueued),
}

impl Default for ChatJoinRequestResult {
  fn default() -> Self {
    ChatJoinRequestResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatJoinRequestResult {
  fn deserialize<D>(deserializer: D) -> Result<ChatJoinRequestResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatJoinRequestResult,
      (chatJoinRequestResultApproved, Approved);
      (chatJoinRequestResultDeclined, Declined);
      (chatJoinRequestResultQueued, Queued);

    )(deserializer)
  }
}

impl RObject for ChatJoinRequestResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatJoinRequestResult::Approved(t) => t.td_name(),
      ChatJoinRequestResult::Declined(t) => t.td_name(),
      ChatJoinRequestResult::Queued(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatJoinRequestResult::Approved(t) => t.extra(),
      ChatJoinRequestResult::Declined(t) => t.extra(),
      ChatJoinRequestResult::Queued(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatJoinRequestResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatJoinRequestResult::_Default(_))
  }

  pub fn is_approved(&self) -> bool {
    matches!(self, ChatJoinRequestResult::Approved(_))
  }
  pub fn is_declined(&self) -> bool {
    matches!(self, ChatJoinRequestResult::Declined(_))
  }
  pub fn is_queued(&self) -> bool {
    matches!(self, ChatJoinRequestResult::Queued(_))
  }

  pub fn on_approved<F: FnOnce(&ChatJoinRequestResultApproved)>(&self, fnc: F) -> &Self {
    if let ChatJoinRequestResult::Approved(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_declined<F: FnOnce(&ChatJoinRequestResultDeclined)>(&self, fnc: F) -> &Self {
    if let ChatJoinRequestResult::Declined(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_queued<F: FnOnce(&ChatJoinRequestResultQueued)>(&self, fnc: F) -> &Self {
    if let ChatJoinRequestResult::Queued(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_approved(&self) -> Option<&ChatJoinRequestResultApproved> {
    if let ChatJoinRequestResult::Approved(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_declined(&self) -> Option<&ChatJoinRequestResultDeclined> {
    if let ChatJoinRequestResult::Declined(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_queued(&self) -> Option<&ChatJoinRequestResultQueued> {
    if let ChatJoinRequestResult::Queued(t) = self {
      return Some(t);
    }
    None
  }

  pub fn approved<T: AsRef<ChatJoinRequestResultApproved>>(t: T) -> Self {
    ChatJoinRequestResult::Approved(t.as_ref().clone())
  }

  pub fn declined<T: AsRef<ChatJoinRequestResultDeclined>>(t: T) -> Self {
    ChatJoinRequestResult::Declined(t.as_ref().clone())
  }

  pub fn queued<T: AsRef<ChatJoinRequestResultQueued>>(t: T) -> Self {
    ChatJoinRequestResult::Queued(t.as_ref().clone())
  }
}

impl AsRef<ChatJoinRequestResult> for ChatJoinRequestResult {
  fn as_ref(&self) -> &ChatJoinRequestResult {
    self
  }
}

/// The request was approved
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequestResultApproved {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatJoinRequestResultApproved {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatJoinRequestResultApproved"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatJoinRequestResult for ChatJoinRequestResultApproved {}

impl ChatJoinRequestResultApproved {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatJoinRequestResultApprovedBuilder {
    let inner = ChatJoinRequestResultApproved {
      td_name: "chatJoinRequestResultApproved".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatJoinRequestResultApprovedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatJoinRequestResultApprovedBuilder {
  inner: ChatJoinRequestResultApproved,
}

impl RTDChatJoinRequestResultApprovedBuilder {
  pub fn build(&self) -> ChatJoinRequestResultApproved {
    self.inner.clone()
  }
}

impl AsRef<ChatJoinRequestResultApproved> for ChatJoinRequestResultApproved {
  fn as_ref(&self) -> &ChatJoinRequestResultApproved {
    self
  }
}

impl AsRef<ChatJoinRequestResultApproved> for RTDChatJoinRequestResultApprovedBuilder {
  fn as_ref(&self) -> &ChatJoinRequestResultApproved {
    &self.inner
  }
}

/// The request was declined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequestResultDeclined {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatJoinRequestResultDeclined {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatJoinRequestResultDeclined"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatJoinRequestResult for ChatJoinRequestResultDeclined {}

impl ChatJoinRequestResultDeclined {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatJoinRequestResultDeclinedBuilder {
    let inner = ChatJoinRequestResultDeclined {
      td_name: "chatJoinRequestResultDeclined".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatJoinRequestResultDeclinedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatJoinRequestResultDeclinedBuilder {
  inner: ChatJoinRequestResultDeclined,
}

impl RTDChatJoinRequestResultDeclinedBuilder {
  pub fn build(&self) -> ChatJoinRequestResultDeclined {
    self.inner.clone()
  }
}

impl AsRef<ChatJoinRequestResultDeclined> for ChatJoinRequestResultDeclined {
  fn as_ref(&self) -> &ChatJoinRequestResultDeclined {
    self
  }
}

impl AsRef<ChatJoinRequestResultDeclined> for RTDChatJoinRequestResultDeclinedBuilder {
  fn as_ref(&self) -> &ChatJoinRequestResultDeclined {
    &self.inner
  }
}

/// The request was postponed without a decision
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequestResultQueued {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatJoinRequestResultQueued {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatJoinRequestResultQueued"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatJoinRequestResult for ChatJoinRequestResultQueued {}

impl ChatJoinRequestResultQueued {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatJoinRequestResultQueuedBuilder {
    let inner = ChatJoinRequestResultQueued {
      td_name: "chatJoinRequestResultQueued".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatJoinRequestResultQueuedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatJoinRequestResultQueuedBuilder {
  inner: ChatJoinRequestResultQueued,
}

impl RTDChatJoinRequestResultQueuedBuilder {
  pub fn build(&self) -> ChatJoinRequestResultQueued {
    self.inner.clone()
  }
}

impl AsRef<ChatJoinRequestResultQueued> for ChatJoinRequestResultQueued {
  fn as_ref(&self) -> &ChatJoinRequestResultQueued {
    self
  }
}

impl AsRef<ChatJoinRequestResultQueued> for RTDChatJoinRequestResultQueuedBuilder {
  fn as_ref(&self) -> &ChatJoinRequestResultQueued {
    &self.inner
  }
}
