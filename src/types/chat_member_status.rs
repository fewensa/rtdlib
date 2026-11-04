use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Provides information about the status of a member in a chat
pub trait TDChatMemberStatus: Debug + RObject {}

/// Provides information about the status of a member in a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatMemberStatus {
  #[doc(hidden)]
  _Default(()),
  /// The user is a member of the chat and has some additional privileges. In basic groups, administrators have all applicable rights. In supergroups and channels, any subset of the rights can be chosen for an administrator
  Administrator(ChatMemberStatusAdministrator),
  /// The user or the chat was banned (and hence is not a member of the chat). Implies the user can't return to the chat, view messages, or be used as a participant identifier to join a video chat of the chat
  Banned(ChatMemberStatusBanned),
  /// The user is the owner of the chat and has all the administrator privileges
  Creator(ChatMemberStatusCreator),
  /// The user or the chat is not a chat member
  Left(ChatMemberStatusLeft),
  /// The user is a member of the chat, without any additional privileges or restrictions
  Member(ChatMemberStatusMember),
  /// The user is under certain restrictions in the chat. Not supported in basic groups and channels
  Restricted(ChatMemberStatusRestricted),
}

impl Default for ChatMemberStatus {
  fn default() -> Self {
    ChatMemberStatus::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatMemberStatus {
  fn deserialize<D>(deserializer: D) -> Result<ChatMemberStatus, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatMemberStatus,
      (chatMemberStatusAdministrator, Administrator);
      (chatMemberStatusBanned, Banned);
      (chatMemberStatusCreator, Creator);
      (chatMemberStatusLeft, Left);
      (chatMemberStatusMember, Member);
      (chatMemberStatusRestricted, Restricted);

    )(deserializer)
  }
}

impl RObject for ChatMemberStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatMemberStatus::Administrator(t) => t.td_name(),
      ChatMemberStatus::Banned(t) => t.td_name(),
      ChatMemberStatus::Creator(t) => t.td_name(),
      ChatMemberStatus::Left(t) => t.td_name(),
      ChatMemberStatus::Member(t) => t.td_name(),
      ChatMemberStatus::Restricted(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatMemberStatus::Administrator(t) => t.extra(),
      ChatMemberStatus::Banned(t) => t.extra(),
      ChatMemberStatus::Creator(t) => t.extra(),
      ChatMemberStatus::Left(t) => t.extra(),
      ChatMemberStatus::Member(t) => t.extra(),
      ChatMemberStatus::Restricted(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatMemberStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatMemberStatus::_Default(_))
  }

  pub fn is_administrator(&self) -> bool {
    matches!(self, ChatMemberStatus::Administrator(_))
  }
  pub fn is_banned(&self) -> bool {
    matches!(self, ChatMemberStatus::Banned(_))
  }
  pub fn is_creator(&self) -> bool {
    matches!(self, ChatMemberStatus::Creator(_))
  }
  pub fn is_left(&self) -> bool {
    matches!(self, ChatMemberStatus::Left(_))
  }
  pub fn is_member(&self) -> bool {
    matches!(self, ChatMemberStatus::Member(_))
  }
  pub fn is_restricted(&self) -> bool {
    matches!(self, ChatMemberStatus::Restricted(_))
  }

  pub fn on_administrator<F: FnOnce(&ChatMemberStatusAdministrator)>(&self, fnc: F) -> &Self {
    if let ChatMemberStatus::Administrator(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_banned<F: FnOnce(&ChatMemberStatusBanned)>(&self, fnc: F) -> &Self {
    if let ChatMemberStatus::Banned(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_creator<F: FnOnce(&ChatMemberStatusCreator)>(&self, fnc: F) -> &Self {
    if let ChatMemberStatus::Creator(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_left<F: FnOnce(&ChatMemberStatusLeft)>(&self, fnc: F) -> &Self {
    if let ChatMemberStatus::Left(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_member<F: FnOnce(&ChatMemberStatusMember)>(&self, fnc: F) -> &Self {
    if let ChatMemberStatus::Member(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restricted<F: FnOnce(&ChatMemberStatusRestricted)>(&self, fnc: F) -> &Self {
    if let ChatMemberStatus::Restricted(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_administrator(&self) -> Option<&ChatMemberStatusAdministrator> {
    if let ChatMemberStatus::Administrator(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_banned(&self) -> Option<&ChatMemberStatusBanned> {
    if let ChatMemberStatus::Banned(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_creator(&self) -> Option<&ChatMemberStatusCreator> {
    if let ChatMemberStatus::Creator(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_left(&self) -> Option<&ChatMemberStatusLeft> {
    if let ChatMemberStatus::Left(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_member(&self) -> Option<&ChatMemberStatusMember> {
    if let ChatMemberStatus::Member(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restricted(&self) -> Option<&ChatMemberStatusRestricted> {
    if let ChatMemberStatus::Restricted(t) = self {
      return Some(t);
    }
    None
  }

  pub fn administrator<T: AsRef<ChatMemberStatusAdministrator>>(t: T) -> Self {
    ChatMemberStatus::Administrator(t.as_ref().clone())
  }

  pub fn banned<T: AsRef<ChatMemberStatusBanned>>(t: T) -> Self {
    ChatMemberStatus::Banned(t.as_ref().clone())
  }

  pub fn creator<T: AsRef<ChatMemberStatusCreator>>(t: T) -> Self {
    ChatMemberStatus::Creator(t.as_ref().clone())
  }

  pub fn left<T: AsRef<ChatMemberStatusLeft>>(t: T) -> Self {
    ChatMemberStatus::Left(t.as_ref().clone())
  }

  pub fn member<T: AsRef<ChatMemberStatusMember>>(t: T) -> Self {
    ChatMemberStatus::Member(t.as_ref().clone())
  }

  pub fn restricted<T: AsRef<ChatMemberStatusRestricted>>(t: T) -> Self {
    ChatMemberStatus::Restricted(t.as_ref().clone())
  }
}

impl AsRef<ChatMemberStatus> for ChatMemberStatus {
  fn as_ref(&self) -> &ChatMemberStatus {
    self
  }
}

/// The user is a member of the chat and has some additional privileges. In basic groups, administrators have all applicable rights. In supergroups and channels, any subset of the rights can be chosen for an administrator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusAdministrator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the current user can edit the administrator privileges for the called user
  can_be_edited: bool,
  /// Rights of the administrator
  rights: ChatAdministratorRights,
}

impl RObject for ChatMemberStatusAdministrator {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMemberStatusAdministrator"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatMemberStatus for ChatMemberStatusAdministrator {}

impl ChatMemberStatusAdministrator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMemberStatusAdministratorBuilder {
    let inner = ChatMemberStatusAdministrator {
      td_name: "chatMemberStatusAdministrator".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatMemberStatusAdministratorBuilder { inner }
  }

  pub fn can_be_edited(&self) -> bool {
    self.can_be_edited
  }

  pub fn rights(&self) -> &ChatAdministratorRights {
    &self.rights
  }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusAdministratorBuilder {
  inner: ChatMemberStatusAdministrator,
}

impl RTDChatMemberStatusAdministratorBuilder {
  pub fn build(&self) -> ChatMemberStatusAdministrator {
    self.inner.clone()
  }

  pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
    self.inner.can_be_edited = can_be_edited;
    self
  }

  pub fn rights<T: AsRef<ChatAdministratorRights>>(&mut self, rights: T) -> &mut Self {
    self.inner.rights = rights.as_ref().clone();
    self
  }
}

impl AsRef<ChatMemberStatusAdministrator> for ChatMemberStatusAdministrator {
  fn as_ref(&self) -> &ChatMemberStatusAdministrator {
    self
  }
}

impl AsRef<ChatMemberStatusAdministrator> for RTDChatMemberStatusAdministratorBuilder {
  fn as_ref(&self) -> &ChatMemberStatusAdministrator {
    &self.inner
  }
}

/// The user or the chat was banned (and hence is not a member of the chat). Implies the user can't return to the chat, view messages, or be used as a participant identifier to join a video chat of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusBanned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever. Always 0 in basic groups
  banned_until_date: i64,
}

impl RObject for ChatMemberStatusBanned {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMemberStatusBanned"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatMemberStatus for ChatMemberStatusBanned {}

impl ChatMemberStatusBanned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMemberStatusBannedBuilder {
    let inner = ChatMemberStatusBanned {
      td_name: "chatMemberStatusBanned".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatMemberStatusBannedBuilder { inner }
  }

  pub fn banned_until_date(&self) -> i64 {
    self.banned_until_date
  }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusBannedBuilder {
  inner: ChatMemberStatusBanned,
}

impl RTDChatMemberStatusBannedBuilder {
  pub fn build(&self) -> ChatMemberStatusBanned {
    self.inner.clone()
  }

  pub fn banned_until_date(&mut self, banned_until_date: i64) -> &mut Self {
    self.inner.banned_until_date = banned_until_date;
    self
  }
}

impl AsRef<ChatMemberStatusBanned> for ChatMemberStatusBanned {
  fn as_ref(&self) -> &ChatMemberStatusBanned {
    self
  }
}

impl AsRef<ChatMemberStatusBanned> for RTDChatMemberStatusBannedBuilder {
  fn as_ref(&self) -> &ChatMemberStatusBanned {
    &self.inner
  }
}

/// The user is the owner of the chat and has all the administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusCreator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the creator isn't shown in the chat member list and sends messages anonymously; applicable to supergroups only
  is_anonymous: bool,
  /// True, if the user is a member of the chat
  is_member: bool,
}

impl RObject for ChatMemberStatusCreator {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMemberStatusCreator"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatMemberStatus for ChatMemberStatusCreator {}

impl ChatMemberStatusCreator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMemberStatusCreatorBuilder {
    let inner = ChatMemberStatusCreator {
      td_name: "chatMemberStatusCreator".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatMemberStatusCreatorBuilder { inner }
  }

  pub fn is_anonymous(&self) -> bool {
    self.is_anonymous
  }

  pub fn is_member(&self) -> bool {
    self.is_member
  }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusCreatorBuilder {
  inner: ChatMemberStatusCreator,
}

impl RTDChatMemberStatusCreatorBuilder {
  pub fn build(&self) -> ChatMemberStatusCreator {
    self.inner.clone()
  }

  pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
    self.inner.is_anonymous = is_anonymous;
    self
  }

  pub fn is_member(&mut self, is_member: bool) -> &mut Self {
    self.inner.is_member = is_member;
    self
  }
}

impl AsRef<ChatMemberStatusCreator> for ChatMemberStatusCreator {
  fn as_ref(&self) -> &ChatMemberStatusCreator {
    self
  }
}

impl AsRef<ChatMemberStatusCreator> for RTDChatMemberStatusCreatorBuilder {
  fn as_ref(&self) -> &ChatMemberStatusCreator {
    &self.inner
  }
}

/// The user or the chat is not a chat member
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatMemberStatusLeft {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMemberStatusLeft"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatMemberStatus for ChatMemberStatusLeft {}

impl ChatMemberStatusLeft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMemberStatusLeftBuilder {
    let inner = ChatMemberStatusLeft {
      td_name: "chatMemberStatusLeft".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatMemberStatusLeftBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusLeftBuilder {
  inner: ChatMemberStatusLeft,
}

impl RTDChatMemberStatusLeftBuilder {
  pub fn build(&self) -> ChatMemberStatusLeft {
    self.inner.clone()
  }
}

impl AsRef<ChatMemberStatusLeft> for ChatMemberStatusLeft {
  fn as_ref(&self) -> &ChatMemberStatusLeft {
    self
  }
}

impl AsRef<ChatMemberStatusLeft> for RTDChatMemberStatusLeftBuilder {
  fn as_ref(&self) -> &ChatMemberStatusLeft {
    &self.inner
  }
}

/// The user is a member of the chat, without any additional privileges or restrictions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the user will be removed from the chat because of the expired subscription; 0 if never. Ignored in setChatMemberStatus
  member_until_date: i64,
}

impl RObject for ChatMemberStatusMember {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMemberStatusMember"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatMemberStatus for ChatMemberStatusMember {}

impl ChatMemberStatusMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMemberStatusMemberBuilder {
    let inner = ChatMemberStatusMember {
      td_name: "chatMemberStatusMember".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatMemberStatusMemberBuilder { inner }
  }

  pub fn member_until_date(&self) -> i64 {
    self.member_until_date
  }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusMemberBuilder {
  inner: ChatMemberStatusMember,
}

impl RTDChatMemberStatusMemberBuilder {
  pub fn build(&self) -> ChatMemberStatusMember {
    self.inner.clone()
  }

  pub fn member_until_date(&mut self, member_until_date: i64) -> &mut Self {
    self.inner.member_until_date = member_until_date;
    self
  }
}

impl AsRef<ChatMemberStatusMember> for ChatMemberStatusMember {
  fn as_ref(&self) -> &ChatMemberStatusMember {
    self
  }
}

impl AsRef<ChatMemberStatusMember> for RTDChatMemberStatusMemberBuilder {
  fn as_ref(&self) -> &ChatMemberStatusMember {
    &self.inner
  }
}

/// The user is under certain restrictions in the chat. Not supported in basic groups and channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the user is a member of the chat
  is_member: bool,
  /// Point in time (Unix timestamp) when restrictions will be lifted from the user; 0 if never. If the user is restricted for more than 366 days or for less than 30 seconds from the current time, the user is considered to be restricted forever
  restricted_until_date: i64,
  /// User permissions in the chat
  permissions: ChatPermissions,
}

impl RObject for ChatMemberStatusRestricted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatMemberStatusRestricted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatMemberStatus for ChatMemberStatusRestricted {}

impl ChatMemberStatusRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatMemberStatusRestrictedBuilder {
    let inner = ChatMemberStatusRestricted {
      td_name: "chatMemberStatusRestricted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatMemberStatusRestrictedBuilder { inner }
  }

  pub fn is_member(&self) -> bool {
    self.is_member
  }

  pub fn restricted_until_date(&self) -> i64 {
    self.restricted_until_date
  }

  pub fn permissions(&self) -> &ChatPermissions {
    &self.permissions
  }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusRestrictedBuilder {
  inner: ChatMemberStatusRestricted,
}

impl RTDChatMemberStatusRestrictedBuilder {
  pub fn build(&self) -> ChatMemberStatusRestricted {
    self.inner.clone()
  }

  pub fn is_member(&mut self, is_member: bool) -> &mut Self {
    self.inner.is_member = is_member;
    self
  }

  pub fn restricted_until_date(&mut self, restricted_until_date: i64) -> &mut Self {
    self.inner.restricted_until_date = restricted_until_date;
    self
  }

  pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
    self.inner.permissions = permissions.as_ref().clone();
    self
  }
}

impl AsRef<ChatMemberStatusRestricted> for ChatMemberStatusRestricted {
  fn as_ref(&self) -> &ChatMemberStatusRestricted {
    self
  }
}

impl AsRef<ChatMemberStatusRestricted> for RTDChatMemberStatusRestrictedBuilder {
  fn as_ref(&self) -> &ChatMemberStatusRestricted {
    &self.inner
  }
}
