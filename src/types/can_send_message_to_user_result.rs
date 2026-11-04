use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of canSendMessageToUser
pub trait TDCanSendMessageToUserResult: Debug + RObject {}

/// Describes result of canSendMessageToUser
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CanSendMessageToUserResult {
  #[doc(hidden)]
  _Default(()),
  /// Checks whether the current user can message another user or try to create a chat with them
  CanSendMessageToUser(CanSendMessageToUser),
  /// The user can be messaged
  Ok(CanSendMessageToUserResultOk),
  /// The user can be messaged, but the messages are paid
  UserHasPaidMessages(CanSendMessageToUserResultUserHasPaidMessages),
  /// The user can't be messaged, because they are deleted or unknown
  UserIsDeleted(CanSendMessageToUserResultUserIsDeleted),
  /// The user can't be messaged, because they restrict new chats with non-contacts
  UserRestrictsNewChats(CanSendMessageToUserResultUserRestrictsNewChats),
}

impl Default for CanSendMessageToUserResult {
  fn default() -> Self {
    CanSendMessageToUserResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for CanSendMessageToUserResult {
  fn deserialize<D>(deserializer: D) -> Result<CanSendMessageToUserResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CanSendMessageToUserResult,
      (canSendMessageToUser, CanSendMessageToUser);
      (canSendMessageToUserResultOk, Ok);
      (canSendMessageToUserResultUserHasPaidMessages, UserHasPaidMessages);
      (canSendMessageToUserResultUserIsDeleted, UserIsDeleted);
      (canSendMessageToUserResultUserRestrictsNewChats, UserRestrictsNewChats);

    )(deserializer)
  }
}

impl RObject for CanSendMessageToUserResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      CanSendMessageToUserResult::CanSendMessageToUser(t) => t.td_name(),
      CanSendMessageToUserResult::Ok(t) => t.td_name(),
      CanSendMessageToUserResult::UserHasPaidMessages(t) => t.td_name(),
      CanSendMessageToUserResult::UserIsDeleted(t) => t.td_name(),
      CanSendMessageToUserResult::UserRestrictsNewChats(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      CanSendMessageToUserResult::CanSendMessageToUser(t) => t.extra(),
      CanSendMessageToUserResult::Ok(t) => t.extra(),
      CanSendMessageToUserResult::UserHasPaidMessages(t) => t.extra(),
      CanSendMessageToUserResult::UserIsDeleted(t) => t.extra(),
      CanSendMessageToUserResult::UserRestrictsNewChats(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CanSendMessageToUserResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, CanSendMessageToUserResult::_Default(_))
  }

  pub fn is_can_send_message_to_user(&self) -> bool {
    matches!(self, CanSendMessageToUserResult::CanSendMessageToUser(_))
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, CanSendMessageToUserResult::Ok(_))
  }
  pub fn is_user_has_paid_messages(&self) -> bool {
    matches!(self, CanSendMessageToUserResult::UserHasPaidMessages(_))
  }
  pub fn is_user_is_deleted(&self) -> bool {
    matches!(self, CanSendMessageToUserResult::UserIsDeleted(_))
  }
  pub fn is_user_restricts_new_chats(&self) -> bool {
    matches!(self, CanSendMessageToUserResult::UserRestrictsNewChats(_))
  }

  pub fn on_can_send_message_to_user<F: FnOnce(&CanSendMessageToUser)>(&self, fnc: F) -> &Self {
    if let CanSendMessageToUserResult::CanSendMessageToUser(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ok<F: FnOnce(&CanSendMessageToUserResultOk)>(&self, fnc: F) -> &Self {
    if let CanSendMessageToUserResult::Ok(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_has_paid_messages<F: FnOnce(&CanSendMessageToUserResultUserHasPaidMessages)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CanSendMessageToUserResult::UserHasPaidMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_is_deleted<F: FnOnce(&CanSendMessageToUserResultUserIsDeleted)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CanSendMessageToUserResult::UserIsDeleted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_restricts_new_chats<
    F: FnOnce(&CanSendMessageToUserResultUserRestrictsNewChats),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let CanSendMessageToUserResult::UserRestrictsNewChats(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_can_send_message_to_user(&self) -> Option<&CanSendMessageToUser> {
    if let CanSendMessageToUserResult::CanSendMessageToUser(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ok(&self) -> Option<&CanSendMessageToUserResultOk> {
    if let CanSendMessageToUserResult::Ok(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_has_paid_messages(
    &self,
  ) -> Option<&CanSendMessageToUserResultUserHasPaidMessages> {
    if let CanSendMessageToUserResult::UserHasPaidMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_is_deleted(&self) -> Option<&CanSendMessageToUserResultUserIsDeleted> {
    if let CanSendMessageToUserResult::UserIsDeleted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_restricts_new_chats(
    &self,
  ) -> Option<&CanSendMessageToUserResultUserRestrictsNewChats> {
    if let CanSendMessageToUserResult::UserRestrictsNewChats(t) = self {
      return Some(t);
    }
    None
  }

  pub fn can_send_message_to_user<T: AsRef<CanSendMessageToUser>>(t: T) -> Self {
    CanSendMessageToUserResult::CanSendMessageToUser(t.as_ref().clone())
  }

  pub fn ok<T: AsRef<CanSendMessageToUserResultOk>>(t: T) -> Self {
    CanSendMessageToUserResult::Ok(t.as_ref().clone())
  }

  pub fn user_has_paid_messages<T: AsRef<CanSendMessageToUserResultUserHasPaidMessages>>(
    t: T,
  ) -> Self {
    CanSendMessageToUserResult::UserHasPaidMessages(t.as_ref().clone())
  }

  pub fn user_is_deleted<T: AsRef<CanSendMessageToUserResultUserIsDeleted>>(t: T) -> Self {
    CanSendMessageToUserResult::UserIsDeleted(t.as_ref().clone())
  }

  pub fn user_restricts_new_chats<T: AsRef<CanSendMessageToUserResultUserRestrictsNewChats>>(
    t: T,
  ) -> Self {
    CanSendMessageToUserResult::UserRestrictsNewChats(t.as_ref().clone())
  }
}

impl AsRef<CanSendMessageToUserResult> for CanSendMessageToUserResult {
  fn as_ref(&self) -> &CanSendMessageToUserResult {
    self
  }
}

/// The user can be messaged
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendMessageToUserResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CanSendMessageToUserResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canSendMessageToUserResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanSendMessageToUserResult for CanSendMessageToUserResultOk {}

impl CanSendMessageToUserResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanSendMessageToUserResultOkBuilder {
    let inner = CanSendMessageToUserResultOk {
      td_name: "canSendMessageToUserResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCanSendMessageToUserResultOkBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCanSendMessageToUserResultOkBuilder {
  inner: CanSendMessageToUserResultOk,
}

impl RTDCanSendMessageToUserResultOkBuilder {
  pub fn build(&self) -> CanSendMessageToUserResultOk {
    self.inner.clone()
  }
}

impl AsRef<CanSendMessageToUserResultOk> for CanSendMessageToUserResultOk {
  fn as_ref(&self) -> &CanSendMessageToUserResultOk {
    self
  }
}

impl AsRef<CanSendMessageToUserResultOk> for RTDCanSendMessageToUserResultOkBuilder {
  fn as_ref(&self) -> &CanSendMessageToUserResultOk {
    &self.inner
  }
}

/// The user can be messaged, but the messages are paid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendMessageToUserResultUserHasPaidMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of Telegram Stars that must be paid by the current user for each sent message to the user
  outgoing_paid_message_star_count: i64,
}

impl RObject for CanSendMessageToUserResultUserHasPaidMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canSendMessageToUserResultUserHasPaidMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanSendMessageToUserResult for CanSendMessageToUserResultUserHasPaidMessages {}

impl CanSendMessageToUserResultUserHasPaidMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanSendMessageToUserResultUserHasPaidMessagesBuilder {
    let inner = CanSendMessageToUserResultUserHasPaidMessages {
      td_name: "canSendMessageToUserResultUserHasPaidMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCanSendMessageToUserResultUserHasPaidMessagesBuilder { inner }
  }

  pub fn outgoing_paid_message_star_count(&self) -> i64 {
    self.outgoing_paid_message_star_count
  }
}

#[doc(hidden)]
pub struct RTDCanSendMessageToUserResultUserHasPaidMessagesBuilder {
  inner: CanSendMessageToUserResultUserHasPaidMessages,
}

impl RTDCanSendMessageToUserResultUserHasPaidMessagesBuilder {
  pub fn build(&self) -> CanSendMessageToUserResultUserHasPaidMessages {
    self.inner.clone()
  }

  pub fn outgoing_paid_message_star_count(
    &mut self,
    outgoing_paid_message_star_count: i64,
  ) -> &mut Self {
    self.inner.outgoing_paid_message_star_count = outgoing_paid_message_star_count;
    self
  }
}

impl AsRef<CanSendMessageToUserResultUserHasPaidMessages>
  for CanSendMessageToUserResultUserHasPaidMessages
{
  fn as_ref(&self) -> &CanSendMessageToUserResultUserHasPaidMessages {
    self
  }
}

impl AsRef<CanSendMessageToUserResultUserHasPaidMessages>
  for RTDCanSendMessageToUserResultUserHasPaidMessagesBuilder
{
  fn as_ref(&self) -> &CanSendMessageToUserResultUserHasPaidMessages {
    &self.inner
  }
}

/// The user can't be messaged, because they are deleted or unknown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendMessageToUserResultUserIsDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CanSendMessageToUserResultUserIsDeleted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canSendMessageToUserResultUserIsDeleted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanSendMessageToUserResult for CanSendMessageToUserResultUserIsDeleted {}

impl CanSendMessageToUserResultUserIsDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanSendMessageToUserResultUserIsDeletedBuilder {
    let inner = CanSendMessageToUserResultUserIsDeleted {
      td_name: "canSendMessageToUserResultUserIsDeleted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCanSendMessageToUserResultUserIsDeletedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCanSendMessageToUserResultUserIsDeletedBuilder {
  inner: CanSendMessageToUserResultUserIsDeleted,
}

impl RTDCanSendMessageToUserResultUserIsDeletedBuilder {
  pub fn build(&self) -> CanSendMessageToUserResultUserIsDeleted {
    self.inner.clone()
  }
}

impl AsRef<CanSendMessageToUserResultUserIsDeleted> for CanSendMessageToUserResultUserIsDeleted {
  fn as_ref(&self) -> &CanSendMessageToUserResultUserIsDeleted {
    self
  }
}

impl AsRef<CanSendMessageToUserResultUserIsDeleted>
  for RTDCanSendMessageToUserResultUserIsDeletedBuilder
{
  fn as_ref(&self) -> &CanSendMessageToUserResultUserIsDeleted {
    &self.inner
  }
}

/// The user can't be messaged, because they restrict new chats with non-contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendMessageToUserResultUserRestrictsNewChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CanSendMessageToUserResultUserRestrictsNewChats {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "canSendMessageToUserResultUserRestrictsNewChats"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCanSendMessageToUserResult for CanSendMessageToUserResultUserRestrictsNewChats {}

impl CanSendMessageToUserResultUserRestrictsNewChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCanSendMessageToUserResultUserRestrictsNewChatsBuilder {
    let inner = CanSendMessageToUserResultUserRestrictsNewChats {
      td_name: "canSendMessageToUserResultUserRestrictsNewChats".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCanSendMessageToUserResultUserRestrictsNewChatsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCanSendMessageToUserResultUserRestrictsNewChatsBuilder {
  inner: CanSendMessageToUserResultUserRestrictsNewChats,
}

impl RTDCanSendMessageToUserResultUserRestrictsNewChatsBuilder {
  pub fn build(&self) -> CanSendMessageToUserResultUserRestrictsNewChats {
    self.inner.clone()
  }
}

impl AsRef<CanSendMessageToUserResultUserRestrictsNewChats>
  for CanSendMessageToUserResultUserRestrictsNewChats
{
  fn as_ref(&self) -> &CanSendMessageToUserResultUserRestrictsNewChats {
    self
  }
}

impl AsRef<CanSendMessageToUserResultUserRestrictsNewChats>
  for RTDCanSendMessageToUserResultUserRestrictsNewChatsBuilder
{
  fn as_ref(&self) -> &CanSendMessageToUserResultUserRestrictsNewChats {
    &self.inner
  }
}
