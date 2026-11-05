use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of join of a chat by the current user
pub trait TDChatJoinResult: Debug + RObject {}

/// Describes result of join of a chat by the current user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatJoinResult {
  #[doc(hidden)]
  _Default(()),
  /// The join was declined by the guard bot
  Declined(ChatJoinResultDeclined),
  /// An approval from a guard bot through a Web App is required to join the chat
  GuardBotApprovalRequired(ChatJoinResultGuardBotApprovalRequired),
  /// The join request was sent and has to be approved by administrators of the chat
  RequestSent(ChatJoinResultRequestSent),
  /// The chat was joined successfully
  Success(ChatJoinResultSuccess),
  /// Adds the current user as a new member to a chat. Private and secret chats can't be joined using this method
  JoinChat(JoinChat),
  /// Uses an invite link to add the current user to the chat if possible
  JoinChatByInviteLink(JoinChatByInviteLink),
}

impl Default for ChatJoinResult {
  fn default() -> Self {
    ChatJoinResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatJoinResult {
  fn deserialize<D>(deserializer: D) -> Result<ChatJoinResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatJoinResult,
      (chatJoinResultDeclined, Declined);
      (chatJoinResultGuardBotApprovalRequired, GuardBotApprovalRequired);
      (chatJoinResultRequestSent, RequestSent);
      (chatJoinResultSuccess, Success);
      (joinChat, JoinChat);
      (joinChatByInviteLink, JoinChatByInviteLink);

    )(deserializer)
  }
}

impl RObject for ChatJoinResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatJoinResult::Declined(t) => t.td_name(),
      ChatJoinResult::GuardBotApprovalRequired(t) => t.td_name(),
      ChatJoinResult::RequestSent(t) => t.td_name(),
      ChatJoinResult::Success(t) => t.td_name(),
      ChatJoinResult::JoinChat(t) => t.td_name(),
      ChatJoinResult::JoinChatByInviteLink(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatJoinResult::Declined(t) => t.extra(),
      ChatJoinResult::GuardBotApprovalRequired(t) => t.extra(),
      ChatJoinResult::RequestSent(t) => t.extra(),
      ChatJoinResult::Success(t) => t.extra(),
      ChatJoinResult::JoinChat(t) => t.extra(),
      ChatJoinResult::JoinChatByInviteLink(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatJoinResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatJoinResult::_Default(_))
  }

  pub fn is_declined(&self) -> bool {
    matches!(self, ChatJoinResult::Declined(_))
  }
  pub fn is_guard_bot_approval_required(&self) -> bool {
    matches!(self, ChatJoinResult::GuardBotApprovalRequired(_))
  }
  pub fn is_request_sent(&self) -> bool {
    matches!(self, ChatJoinResult::RequestSent(_))
  }
  pub fn is_success(&self) -> bool {
    matches!(self, ChatJoinResult::Success(_))
  }
  pub fn is_join_chat(&self) -> bool {
    matches!(self, ChatJoinResult::JoinChat(_))
  }
  pub fn is_join_chat_by_invite_link(&self) -> bool {
    matches!(self, ChatJoinResult::JoinChatByInviteLink(_))
  }

  pub fn on_declined<F: FnOnce(&ChatJoinResultDeclined)>(&self, fnc: F) -> &Self {
    if let ChatJoinResult::Declined(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_guard_bot_approval_required<F: FnOnce(&ChatJoinResultGuardBotApprovalRequired)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ChatJoinResult::GuardBotApprovalRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_request_sent<F: FnOnce(&ChatJoinResultRequestSent)>(&self, fnc: F) -> &Self {
    if let ChatJoinResult::RequestSent(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_success<F: FnOnce(&ChatJoinResultSuccess)>(&self, fnc: F) -> &Self {
    if let ChatJoinResult::Success(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_join_chat<F: FnOnce(&JoinChat)>(&self, fnc: F) -> &Self {
    if let ChatJoinResult::JoinChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_join_chat_by_invite_link<F: FnOnce(&JoinChatByInviteLink)>(&self, fnc: F) -> &Self {
    if let ChatJoinResult::JoinChatByInviteLink(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_declined(&self) -> Option<&ChatJoinResultDeclined> {
    if let ChatJoinResult::Declined(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_guard_bot_approval_required(&self) -> Option<&ChatJoinResultGuardBotApprovalRequired> {
    if let ChatJoinResult::GuardBotApprovalRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_request_sent(&self) -> Option<&ChatJoinResultRequestSent> {
    if let ChatJoinResult::RequestSent(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_success(&self) -> Option<&ChatJoinResultSuccess> {
    if let ChatJoinResult::Success(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_join_chat(&self) -> Option<&JoinChat> {
    if let ChatJoinResult::JoinChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_join_chat_by_invite_link(&self) -> Option<&JoinChatByInviteLink> {
    if let ChatJoinResult::JoinChatByInviteLink(t) = self {
      return Some(t);
    }
    None
  }

  pub fn declined<T: AsRef<ChatJoinResultDeclined>>(t: T) -> Self {
    ChatJoinResult::Declined(t.as_ref().clone())
  }

  pub fn guard_bot_approval_required<T: AsRef<ChatJoinResultGuardBotApprovalRequired>>(
    t: T,
  ) -> Self {
    ChatJoinResult::GuardBotApprovalRequired(t.as_ref().clone())
  }

  pub fn request_sent<T: AsRef<ChatJoinResultRequestSent>>(t: T) -> Self {
    ChatJoinResult::RequestSent(t.as_ref().clone())
  }

  pub fn success<T: AsRef<ChatJoinResultSuccess>>(t: T) -> Self {
    ChatJoinResult::Success(t.as_ref().clone())
  }

  pub fn join_chat<T: AsRef<JoinChat>>(t: T) -> Self {
    ChatJoinResult::JoinChat(t.as_ref().clone())
  }

  pub fn join_chat_by_invite_link<T: AsRef<JoinChatByInviteLink>>(t: T) -> Self {
    ChatJoinResult::JoinChatByInviteLink(t.as_ref().clone())
  }
}

impl AsRef<ChatJoinResult> for ChatJoinResult {
  fn as_ref(&self) -> &ChatJoinResult {
    self
  }
}

/// The join was declined by the guard bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinResultDeclined {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatJoinResultDeclined {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatJoinResultDeclined"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatJoinResult for ChatJoinResultDeclined {}

impl ChatJoinResultDeclined {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatJoinResultDeclinedBuilder {
    let inner = ChatJoinResultDeclined {
      td_name: "chatJoinResultDeclined".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatJoinResultDeclinedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatJoinResultDeclinedBuilder {
  inner: ChatJoinResultDeclined,
}

impl RTDChatJoinResultDeclinedBuilder {
  pub fn build(&self) -> ChatJoinResultDeclined {
    self.inner.clone()
  }
}

impl AsRef<ChatJoinResultDeclined> for ChatJoinResultDeclined {
  fn as_ref(&self) -> &ChatJoinResultDeclined {
    self
  }
}

impl AsRef<ChatJoinResultDeclined> for RTDChatJoinResultDeclinedBuilder {
  fn as_ref(&self) -> &ChatJoinResultDeclined {
    &self.inner
  }
}

/// An approval from a guard bot through a Web App is required to join the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinResultGuardBotApprovalRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the guard bot
  bot_user_id: i64,
  /// Unique identifier of the join request, which will be used in getGuardBotWebAppUrl and updateChatJoinResult
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  query_id: isize,
}

impl RObject for ChatJoinResultGuardBotApprovalRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatJoinResultGuardBotApprovalRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatJoinResult for ChatJoinResultGuardBotApprovalRequired {}

impl ChatJoinResultGuardBotApprovalRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatJoinResultGuardBotApprovalRequiredBuilder {
    let inner = ChatJoinResultGuardBotApprovalRequired {
      td_name: "chatJoinResultGuardBotApprovalRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatJoinResultGuardBotApprovalRequiredBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn query_id(&self) -> isize {
    self.query_id
  }
}

#[doc(hidden)]
pub struct RTDChatJoinResultGuardBotApprovalRequiredBuilder {
  inner: ChatJoinResultGuardBotApprovalRequired,
}

impl RTDChatJoinResultGuardBotApprovalRequiredBuilder {
  pub fn build(&self) -> ChatJoinResultGuardBotApprovalRequired {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn query_id(&mut self, query_id: isize) -> &mut Self {
    self.inner.query_id = query_id;
    self
  }
}

impl AsRef<ChatJoinResultGuardBotApprovalRequired> for ChatJoinResultGuardBotApprovalRequired {
  fn as_ref(&self) -> &ChatJoinResultGuardBotApprovalRequired {
    self
  }
}

impl AsRef<ChatJoinResultGuardBotApprovalRequired>
  for RTDChatJoinResultGuardBotApprovalRequiredBuilder
{
  fn as_ref(&self) -> &ChatJoinResultGuardBotApprovalRequired {
    &self.inner
  }
}

/// The join request was sent and has to be approved by administrators of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinResultRequestSent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatJoinResultRequestSent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatJoinResultRequestSent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatJoinResult for ChatJoinResultRequestSent {}

impl ChatJoinResultRequestSent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatJoinResultRequestSentBuilder {
    let inner = ChatJoinResultRequestSent {
      td_name: "chatJoinResultRequestSent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatJoinResultRequestSentBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatJoinResultRequestSentBuilder {
  inner: ChatJoinResultRequestSent,
}

impl RTDChatJoinResultRequestSentBuilder {
  pub fn build(&self) -> ChatJoinResultRequestSent {
    self.inner.clone()
  }
}

impl AsRef<ChatJoinResultRequestSent> for ChatJoinResultRequestSent {
  fn as_ref(&self) -> &ChatJoinResultRequestSent {
    self
  }
}

impl AsRef<ChatJoinResultRequestSent> for RTDChatJoinResultRequestSentBuilder {
  fn as_ref(&self) -> &ChatJoinResultRequestSent {
    &self.inner
  }
}

/// The chat was joined successfully
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinResultSuccess {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat
  chat_id: i64,
}

impl RObject for ChatJoinResultSuccess {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatJoinResultSuccess"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatJoinResult for ChatJoinResultSuccess {}

impl ChatJoinResultSuccess {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatJoinResultSuccessBuilder {
    let inner = ChatJoinResultSuccess {
      td_name: "chatJoinResultSuccess".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatJoinResultSuccessBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDChatJoinResultSuccessBuilder {
  inner: ChatJoinResultSuccess,
}

impl RTDChatJoinResultSuccessBuilder {
  pub fn build(&self) -> ChatJoinResultSuccess {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<ChatJoinResultSuccess> for ChatJoinResultSuccess {
  fn as_ref(&self) -> &ChatJoinResultSuccess {
    self
  }
}

impl AsRef<ChatJoinResultSuccess> for RTDChatJoinResultSuccessBuilder {
  fn as_ref(&self) -> &ChatJoinResultSuccess {
    &self.inner
  }
}
