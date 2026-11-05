use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of group call participant invitation
pub trait TDInviteGroupCallParticipantResult: Debug + RObject {}

/// Describes result of group call participant invitation
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InviteGroupCallParticipantResult {
  #[doc(hidden)]
  _Default(()),
  /// Invites a user to an active group call; for group calls not bound to a chat only. Sends a service message of the type messageGroupCall. The group call can have at most getOption("group_call_participant_count_max") participants
  InviteGroupCallParticipant(InviteGroupCallParticipant),
  /// The user was invited and a service message of the type messageGroupCall was sent which can be used in declineGroupCallInvitation to cancel the invitation
  Success(InviteGroupCallParticipantResultSuccess),
  /// The user can't be invited because they are already a participant of the call
  UserAlreadyParticipant(InviteGroupCallParticipantResultUserAlreadyParticipant),
  /// The user can't be invited due to their privacy settings
  UserPrivacyRestricted(InviteGroupCallParticipantResultUserPrivacyRestricted),
  /// The user can't be invited because they were banned by the owner of the call and can be invited back only by the owner of the group call
  UserWasBanned(InviteGroupCallParticipantResultUserWasBanned),
}

impl Default for InviteGroupCallParticipantResult {
  fn default() -> Self {
    InviteGroupCallParticipantResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for InviteGroupCallParticipantResult {
  fn deserialize<D>(deserializer: D) -> Result<InviteGroupCallParticipantResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InviteGroupCallParticipantResult,
      (inviteGroupCallParticipant, InviteGroupCallParticipant);
      (inviteGroupCallParticipantResultSuccess, Success);
      (inviteGroupCallParticipantResultUserAlreadyParticipant, UserAlreadyParticipant);
      (inviteGroupCallParticipantResultUserPrivacyRestricted, UserPrivacyRestricted);
      (inviteGroupCallParticipantResultUserWasBanned, UserWasBanned);

    )(deserializer)
  }
}

impl RObject for InviteGroupCallParticipantResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InviteGroupCallParticipantResult::InviteGroupCallParticipant(t) => t.td_name(),
      InviteGroupCallParticipantResult::Success(t) => t.td_name(),
      InviteGroupCallParticipantResult::UserAlreadyParticipant(t) => t.td_name(),
      InviteGroupCallParticipantResult::UserPrivacyRestricted(t) => t.td_name(),
      InviteGroupCallParticipantResult::UserWasBanned(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InviteGroupCallParticipantResult::InviteGroupCallParticipant(t) => t.extra(),
      InviteGroupCallParticipantResult::Success(t) => t.extra(),
      InviteGroupCallParticipantResult::UserAlreadyParticipant(t) => t.extra(),
      InviteGroupCallParticipantResult::UserPrivacyRestricted(t) => t.extra(),
      InviteGroupCallParticipantResult::UserWasBanned(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InviteGroupCallParticipantResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InviteGroupCallParticipantResult::_Default(_))
  }

  pub fn is_invite_group_call_participant(&self) -> bool {
    matches!(
      self,
      InviteGroupCallParticipantResult::InviteGroupCallParticipant(_)
    )
  }
  pub fn is_success(&self) -> bool {
    matches!(self, InviteGroupCallParticipantResult::Success(_))
  }
  pub fn is_user_already_participant(&self) -> bool {
    matches!(
      self,
      InviteGroupCallParticipantResult::UserAlreadyParticipant(_)
    )
  }
  pub fn is_user_privacy_restricted(&self) -> bool {
    matches!(
      self,
      InviteGroupCallParticipantResult::UserPrivacyRestricted(_)
    )
  }
  pub fn is_user_was_banned(&self) -> bool {
    matches!(self, InviteGroupCallParticipantResult::UserWasBanned(_))
  }

  pub fn on_invite_group_call_participant<F: FnOnce(&InviteGroupCallParticipant)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InviteGroupCallParticipantResult::InviteGroupCallParticipant(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_success<F: FnOnce(&InviteGroupCallParticipantResultSuccess)>(&self, fnc: F) -> &Self {
    if let InviteGroupCallParticipantResult::Success(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_already_participant<
    F: FnOnce(&InviteGroupCallParticipantResultUserAlreadyParticipant),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let InviteGroupCallParticipantResult::UserAlreadyParticipant(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_privacy_restricted<
    F: FnOnce(&InviteGroupCallParticipantResultUserPrivacyRestricted),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let InviteGroupCallParticipantResult::UserPrivacyRestricted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_was_banned<F: FnOnce(&InviteGroupCallParticipantResultUserWasBanned)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InviteGroupCallParticipantResult::UserWasBanned(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_invite_group_call_participant(&self) -> Option<&InviteGroupCallParticipant> {
    if let InviteGroupCallParticipantResult::InviteGroupCallParticipant(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_success(&self) -> Option<&InviteGroupCallParticipantResultSuccess> {
    if let InviteGroupCallParticipantResult::Success(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_already_participant(
    &self,
  ) -> Option<&InviteGroupCallParticipantResultUserAlreadyParticipant> {
    if let InviteGroupCallParticipantResult::UserAlreadyParticipant(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_privacy_restricted(
    &self,
  ) -> Option<&InviteGroupCallParticipantResultUserPrivacyRestricted> {
    if let InviteGroupCallParticipantResult::UserPrivacyRestricted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_was_banned(&self) -> Option<&InviteGroupCallParticipantResultUserWasBanned> {
    if let InviteGroupCallParticipantResult::UserWasBanned(t) = self {
      return Some(t);
    }
    None
  }

  pub fn invite_group_call_participant<T: AsRef<InviteGroupCallParticipant>>(t: T) -> Self {
    InviteGroupCallParticipantResult::InviteGroupCallParticipant(t.as_ref().clone())
  }

  pub fn success<T: AsRef<InviteGroupCallParticipantResultSuccess>>(t: T) -> Self {
    InviteGroupCallParticipantResult::Success(t.as_ref().clone())
  }

  pub fn user_already_participant<
    T: AsRef<InviteGroupCallParticipantResultUserAlreadyParticipant>,
  >(
    t: T,
  ) -> Self {
    InviteGroupCallParticipantResult::UserAlreadyParticipant(t.as_ref().clone())
  }

  pub fn user_privacy_restricted<
    T: AsRef<InviteGroupCallParticipantResultUserPrivacyRestricted>,
  >(
    t: T,
  ) -> Self {
    InviteGroupCallParticipantResult::UserPrivacyRestricted(t.as_ref().clone())
  }

  pub fn user_was_banned<T: AsRef<InviteGroupCallParticipantResultUserWasBanned>>(t: T) -> Self {
    InviteGroupCallParticipantResult::UserWasBanned(t.as_ref().clone())
  }
}

impl AsRef<InviteGroupCallParticipantResult> for InviteGroupCallParticipantResult {
  fn as_ref(&self) -> &InviteGroupCallParticipantResult {
    self
  }
}

/// The user was invited and a service message of the type messageGroupCall was sent which can be used in declineGroupCallInvitation to cancel the invitation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteGroupCallParticipantResultSuccess {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat with the invitation message
  chat_id: i64,
  /// Identifier of the message
  message_id: i64,
}

impl RObject for InviteGroupCallParticipantResultSuccess {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inviteGroupCallParticipantResultSuccess"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInviteGroupCallParticipantResult for InviteGroupCallParticipantResultSuccess {}

impl InviteGroupCallParticipantResultSuccess {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInviteGroupCallParticipantResultSuccessBuilder {
    let inner = InviteGroupCallParticipantResultSuccess {
      td_name: "inviteGroupCallParticipantResultSuccess".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInviteGroupCallParticipantResultSuccessBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDInviteGroupCallParticipantResultSuccessBuilder {
  inner: InviteGroupCallParticipantResultSuccess,
}

impl RTDInviteGroupCallParticipantResultSuccessBuilder {
  pub fn build(&self) -> InviteGroupCallParticipantResultSuccess {
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

impl AsRef<InviteGroupCallParticipantResultSuccess> for InviteGroupCallParticipantResultSuccess {
  fn as_ref(&self) -> &InviteGroupCallParticipantResultSuccess {
    self
  }
}

impl AsRef<InviteGroupCallParticipantResultSuccess>
  for RTDInviteGroupCallParticipantResultSuccessBuilder
{
  fn as_ref(&self) -> &InviteGroupCallParticipantResultSuccess {
    &self.inner
  }
}

/// The user can't be invited because they are already a participant of the call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteGroupCallParticipantResultUserAlreadyParticipant {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InviteGroupCallParticipantResultUserAlreadyParticipant {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inviteGroupCallParticipantResultUserAlreadyParticipant"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInviteGroupCallParticipantResult for InviteGroupCallParticipantResultUserAlreadyParticipant {}

impl InviteGroupCallParticipantResultUserAlreadyParticipant {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInviteGroupCallParticipantResultUserAlreadyParticipantBuilder {
    let inner = InviteGroupCallParticipantResultUserAlreadyParticipant {
      td_name: "inviteGroupCallParticipantResultUserAlreadyParticipant".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInviteGroupCallParticipantResultUserAlreadyParticipantBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInviteGroupCallParticipantResultUserAlreadyParticipantBuilder {
  inner: InviteGroupCallParticipantResultUserAlreadyParticipant,
}

impl RTDInviteGroupCallParticipantResultUserAlreadyParticipantBuilder {
  pub fn build(&self) -> InviteGroupCallParticipantResultUserAlreadyParticipant {
    self.inner.clone()
  }
}

impl AsRef<InviteGroupCallParticipantResultUserAlreadyParticipant>
  for InviteGroupCallParticipantResultUserAlreadyParticipant
{
  fn as_ref(&self) -> &InviteGroupCallParticipantResultUserAlreadyParticipant {
    self
  }
}

impl AsRef<InviteGroupCallParticipantResultUserAlreadyParticipant>
  for RTDInviteGroupCallParticipantResultUserAlreadyParticipantBuilder
{
  fn as_ref(&self) -> &InviteGroupCallParticipantResultUserAlreadyParticipant {
    &self.inner
  }
}

/// The user can't be invited due to their privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteGroupCallParticipantResultUserPrivacyRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InviteGroupCallParticipantResultUserPrivacyRestricted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inviteGroupCallParticipantResultUserPrivacyRestricted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInviteGroupCallParticipantResult for InviteGroupCallParticipantResultUserPrivacyRestricted {}

impl InviteGroupCallParticipantResultUserPrivacyRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInviteGroupCallParticipantResultUserPrivacyRestrictedBuilder {
    let inner = InviteGroupCallParticipantResultUserPrivacyRestricted {
      td_name: "inviteGroupCallParticipantResultUserPrivacyRestricted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInviteGroupCallParticipantResultUserPrivacyRestrictedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInviteGroupCallParticipantResultUserPrivacyRestrictedBuilder {
  inner: InviteGroupCallParticipantResultUserPrivacyRestricted,
}

impl RTDInviteGroupCallParticipantResultUserPrivacyRestrictedBuilder {
  pub fn build(&self) -> InviteGroupCallParticipantResultUserPrivacyRestricted {
    self.inner.clone()
  }
}

impl AsRef<InviteGroupCallParticipantResultUserPrivacyRestricted>
  for InviteGroupCallParticipantResultUserPrivacyRestricted
{
  fn as_ref(&self) -> &InviteGroupCallParticipantResultUserPrivacyRestricted {
    self
  }
}

impl AsRef<InviteGroupCallParticipantResultUserPrivacyRestricted>
  for RTDInviteGroupCallParticipantResultUserPrivacyRestrictedBuilder
{
  fn as_ref(&self) -> &InviteGroupCallParticipantResultUserPrivacyRestricted {
    &self.inner
  }
}

/// The user can't be invited because they were banned by the owner of the call and can be invited back only by the owner of the group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteGroupCallParticipantResultUserWasBanned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InviteGroupCallParticipantResultUserWasBanned {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inviteGroupCallParticipantResultUserWasBanned"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInviteGroupCallParticipantResult for InviteGroupCallParticipantResultUserWasBanned {}

impl InviteGroupCallParticipantResultUserWasBanned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInviteGroupCallParticipantResultUserWasBannedBuilder {
    let inner = InviteGroupCallParticipantResultUserWasBanned {
      td_name: "inviteGroupCallParticipantResultUserWasBanned".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInviteGroupCallParticipantResultUserWasBannedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInviteGroupCallParticipantResultUserWasBannedBuilder {
  inner: InviteGroupCallParticipantResultUserWasBanned,
}

impl RTDInviteGroupCallParticipantResultUserWasBannedBuilder {
  pub fn build(&self) -> InviteGroupCallParticipantResultUserWasBanned {
    self.inner.clone()
  }
}

impl AsRef<InviteGroupCallParticipantResultUserWasBanned>
  for InviteGroupCallParticipantResultUserWasBanned
{
  fn as_ref(&self) -> &InviteGroupCallParticipantResultUserWasBanned {
    self
  }
}

impl AsRef<InviteGroupCallParticipantResultUserWasBanned>
  for RTDInviteGroupCallParticipantResultUserWasBannedBuilder
{
  fn as_ref(&self) -> &InviteGroupCallParticipantResultUserWasBanned {
    &self.inner
  }
}
