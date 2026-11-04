use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents result of checking whether a username can be set for a chat
pub trait TDCheckChatUsernameResult: Debug + RObject {}

/// Represents result of checking whether a username can be set for a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CheckChatUsernameResult {
  #[doc(hidden)]
  _Default(()),
  /// Checks whether a username can be set for a new bot or assigned as a secondary username to an existing bot. Use checkChatUsername to check username for other chat types
  CheckBotUsername(CheckBotUsername),
  /// Checks whether a username can be set for a chat
  CheckChatUsername(CheckChatUsername),
  /// The username can be set
  Ok(CheckChatUsernameResultOk),
  /// The user has too many chats with username, one of them must be made private first
  PublicChatsTooMany(CheckChatUsernameResultPublicChatsTooMany),
  /// The user can't be a member of a public supergroup
  PublicGroupsUnavailable(CheckChatUsernameResultPublicGroupsUnavailable),
  /// The username is invalid
  UsernameInvalid(CheckChatUsernameResultUsernameInvalid),
  /// The username is occupied
  UsernameOccupied(CheckChatUsernameResultUsernameOccupied),
  /// The username can be purchased at https://fragment.com. Information about the username can be received using getCollectibleItemInfo
  UsernamePurchasable(CheckChatUsernameResultUsernamePurchasable),
}

impl Default for CheckChatUsernameResult {
  fn default() -> Self {
    CheckChatUsernameResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for CheckChatUsernameResult {
  fn deserialize<D>(deserializer: D) -> Result<CheckChatUsernameResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CheckChatUsernameResult,
      (checkBotUsername, CheckBotUsername);
      (checkChatUsername, CheckChatUsername);
      (checkChatUsernameResultOk, Ok);
      (checkChatUsernameResultPublicChatsTooMany, PublicChatsTooMany);
      (checkChatUsernameResultPublicGroupsUnavailable, PublicGroupsUnavailable);
      (checkChatUsernameResultUsernameInvalid, UsernameInvalid);
      (checkChatUsernameResultUsernameOccupied, UsernameOccupied);
      (checkChatUsernameResultUsernamePurchasable, UsernamePurchasable);

    )(deserializer)
  }
}

impl RObject for CheckChatUsernameResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      CheckChatUsernameResult::CheckBotUsername(t) => t.td_name(),
      CheckChatUsernameResult::CheckChatUsername(t) => t.td_name(),
      CheckChatUsernameResult::Ok(t) => t.td_name(),
      CheckChatUsernameResult::PublicChatsTooMany(t) => t.td_name(),
      CheckChatUsernameResult::PublicGroupsUnavailable(t) => t.td_name(),
      CheckChatUsernameResult::UsernameInvalid(t) => t.td_name(),
      CheckChatUsernameResult::UsernameOccupied(t) => t.td_name(),
      CheckChatUsernameResult::UsernamePurchasable(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      CheckChatUsernameResult::CheckBotUsername(t) => t.extra(),
      CheckChatUsernameResult::CheckChatUsername(t) => t.extra(),
      CheckChatUsernameResult::Ok(t) => t.extra(),
      CheckChatUsernameResult::PublicChatsTooMany(t) => t.extra(),
      CheckChatUsernameResult::PublicGroupsUnavailable(t) => t.extra(),
      CheckChatUsernameResult::UsernameInvalid(t) => t.extra(),
      CheckChatUsernameResult::UsernameOccupied(t) => t.extra(),
      CheckChatUsernameResult::UsernamePurchasable(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CheckChatUsernameResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, CheckChatUsernameResult::_Default(_))
  }

  pub fn is_check_bot_username(&self) -> bool {
    matches!(self, CheckChatUsernameResult::CheckBotUsername(_))
  }
  pub fn is_check_chat_username(&self) -> bool {
    matches!(self, CheckChatUsernameResult::CheckChatUsername(_))
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, CheckChatUsernameResult::Ok(_))
  }
  pub fn is_public_chats_too_many(&self) -> bool {
    matches!(self, CheckChatUsernameResult::PublicChatsTooMany(_))
  }
  pub fn is_public_groups_unavailable(&self) -> bool {
    matches!(self, CheckChatUsernameResult::PublicGroupsUnavailable(_))
  }
  pub fn is_username_invalid(&self) -> bool {
    matches!(self, CheckChatUsernameResult::UsernameInvalid(_))
  }
  pub fn is_username_occupied(&self) -> bool {
    matches!(self, CheckChatUsernameResult::UsernameOccupied(_))
  }
  pub fn is_username_purchasable(&self) -> bool {
    matches!(self, CheckChatUsernameResult::UsernamePurchasable(_))
  }

  pub fn on_check_bot_username<F: FnOnce(&CheckBotUsername)>(&self, fnc: F) -> &Self {
    if let CheckChatUsernameResult::CheckBotUsername(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_check_chat_username<F: FnOnce(&CheckChatUsername)>(&self, fnc: F) -> &Self {
    if let CheckChatUsernameResult::CheckChatUsername(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ok<F: FnOnce(&CheckChatUsernameResultOk)>(&self, fnc: F) -> &Self {
    if let CheckChatUsernameResult::Ok(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_public_chats_too_many<F: FnOnce(&CheckChatUsernameResultPublicChatsTooMany)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CheckChatUsernameResult::PublicChatsTooMany(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_public_groups_unavailable<
    F: FnOnce(&CheckChatUsernameResultPublicGroupsUnavailable),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let CheckChatUsernameResult::PublicGroupsUnavailable(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_username_invalid<F: FnOnce(&CheckChatUsernameResultUsernameInvalid)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CheckChatUsernameResult::UsernameInvalid(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_username_occupied<F: FnOnce(&CheckChatUsernameResultUsernameOccupied)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CheckChatUsernameResult::UsernameOccupied(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_username_purchasable<F: FnOnce(&CheckChatUsernameResultUsernamePurchasable)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let CheckChatUsernameResult::UsernamePurchasable(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_check_bot_username(&self) -> Option<&CheckBotUsername> {
    if let CheckChatUsernameResult::CheckBotUsername(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_check_chat_username(&self) -> Option<&CheckChatUsername> {
    if let CheckChatUsernameResult::CheckChatUsername(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ok(&self) -> Option<&CheckChatUsernameResultOk> {
    if let CheckChatUsernameResult::Ok(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_public_chats_too_many(&self) -> Option<&CheckChatUsernameResultPublicChatsTooMany> {
    if let CheckChatUsernameResult::PublicChatsTooMany(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_public_groups_unavailable(
    &self,
  ) -> Option<&CheckChatUsernameResultPublicGroupsUnavailable> {
    if let CheckChatUsernameResult::PublicGroupsUnavailable(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_username_invalid(&self) -> Option<&CheckChatUsernameResultUsernameInvalid> {
    if let CheckChatUsernameResult::UsernameInvalid(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_username_occupied(&self) -> Option<&CheckChatUsernameResultUsernameOccupied> {
    if let CheckChatUsernameResult::UsernameOccupied(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_username_purchasable(&self) -> Option<&CheckChatUsernameResultUsernamePurchasable> {
    if let CheckChatUsernameResult::UsernamePurchasable(t) = self {
      return Some(t);
    }
    None
  }

  pub fn check_bot_username<T: AsRef<CheckBotUsername>>(t: T) -> Self {
    CheckChatUsernameResult::CheckBotUsername(t.as_ref().clone())
  }

  pub fn check_chat_username<T: AsRef<CheckChatUsername>>(t: T) -> Self {
    CheckChatUsernameResult::CheckChatUsername(t.as_ref().clone())
  }

  pub fn ok<T: AsRef<CheckChatUsernameResultOk>>(t: T) -> Self {
    CheckChatUsernameResult::Ok(t.as_ref().clone())
  }

  pub fn public_chats_too_many<T: AsRef<CheckChatUsernameResultPublicChatsTooMany>>(t: T) -> Self {
    CheckChatUsernameResult::PublicChatsTooMany(t.as_ref().clone())
  }

  pub fn public_groups_unavailable<T: AsRef<CheckChatUsernameResultPublicGroupsUnavailable>>(
    t: T,
  ) -> Self {
    CheckChatUsernameResult::PublicGroupsUnavailable(t.as_ref().clone())
  }

  pub fn username_invalid<T: AsRef<CheckChatUsernameResultUsernameInvalid>>(t: T) -> Self {
    CheckChatUsernameResult::UsernameInvalid(t.as_ref().clone())
  }

  pub fn username_occupied<T: AsRef<CheckChatUsernameResultUsernameOccupied>>(t: T) -> Self {
    CheckChatUsernameResult::UsernameOccupied(t.as_ref().clone())
  }

  pub fn username_purchasable<T: AsRef<CheckChatUsernameResultUsernamePurchasable>>(t: T) -> Self {
    CheckChatUsernameResult::UsernamePurchasable(t.as_ref().clone())
  }
}

impl AsRef<CheckChatUsernameResult> for CheckChatUsernameResult {
  fn as_ref(&self) -> &CheckChatUsernameResult {
    self
  }
}

/// The username can be set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CheckChatUsernameResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "checkChatUsernameResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultOk {}

impl CheckChatUsernameResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCheckChatUsernameResultOkBuilder {
    let inner = CheckChatUsernameResultOk {
      td_name: "checkChatUsernameResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCheckChatUsernameResultOkBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultOkBuilder {
  inner: CheckChatUsernameResultOk,
}

impl RTDCheckChatUsernameResultOkBuilder {
  pub fn build(&self) -> CheckChatUsernameResultOk {
    self.inner.clone()
  }
}

impl AsRef<CheckChatUsernameResultOk> for CheckChatUsernameResultOk {
  fn as_ref(&self) -> &CheckChatUsernameResultOk {
    self
  }
}

impl AsRef<CheckChatUsernameResultOk> for RTDCheckChatUsernameResultOkBuilder {
  fn as_ref(&self) -> &CheckChatUsernameResultOk {
    &self.inner
  }
}

/// The user has too many chats with username, one of them must be made private first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultPublicChatsTooMany {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CheckChatUsernameResultPublicChatsTooMany {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "checkChatUsernameResultPublicChatsTooMany"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultPublicChatsTooMany {}

impl CheckChatUsernameResultPublicChatsTooMany {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCheckChatUsernameResultPublicChatsTooManyBuilder {
    let inner = CheckChatUsernameResultPublicChatsTooMany {
      td_name: "checkChatUsernameResultPublicChatsTooMany".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCheckChatUsernameResultPublicChatsTooManyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultPublicChatsTooManyBuilder {
  inner: CheckChatUsernameResultPublicChatsTooMany,
}

impl RTDCheckChatUsernameResultPublicChatsTooManyBuilder {
  pub fn build(&self) -> CheckChatUsernameResultPublicChatsTooMany {
    self.inner.clone()
  }
}

impl AsRef<CheckChatUsernameResultPublicChatsTooMany>
  for CheckChatUsernameResultPublicChatsTooMany
{
  fn as_ref(&self) -> &CheckChatUsernameResultPublicChatsTooMany {
    self
  }
}

impl AsRef<CheckChatUsernameResultPublicChatsTooMany>
  for RTDCheckChatUsernameResultPublicChatsTooManyBuilder
{
  fn as_ref(&self) -> &CheckChatUsernameResultPublicChatsTooMany {
    &self.inner
  }
}

/// The user can't be a member of a public supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultPublicGroupsUnavailable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CheckChatUsernameResultPublicGroupsUnavailable {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "checkChatUsernameResultPublicGroupsUnavailable"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultPublicGroupsUnavailable {}

impl CheckChatUsernameResultPublicGroupsUnavailable {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder {
    let inner = CheckChatUsernameResultPublicGroupsUnavailable {
      td_name: "checkChatUsernameResultPublicGroupsUnavailable".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder {
  inner: CheckChatUsernameResultPublicGroupsUnavailable,
}

impl RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder {
  pub fn build(&self) -> CheckChatUsernameResultPublicGroupsUnavailable {
    self.inner.clone()
  }
}

impl AsRef<CheckChatUsernameResultPublicGroupsUnavailable>
  for CheckChatUsernameResultPublicGroupsUnavailable
{
  fn as_ref(&self) -> &CheckChatUsernameResultPublicGroupsUnavailable {
    self
  }
}

impl AsRef<CheckChatUsernameResultPublicGroupsUnavailable>
  for RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder
{
  fn as_ref(&self) -> &CheckChatUsernameResultPublicGroupsUnavailable {
    &self.inner
  }
}

/// The username is invalid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernameInvalid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CheckChatUsernameResultUsernameInvalid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "checkChatUsernameResultUsernameInvalid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultUsernameInvalid {}

impl CheckChatUsernameResultUsernameInvalid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCheckChatUsernameResultUsernameInvalidBuilder {
    let inner = CheckChatUsernameResultUsernameInvalid {
      td_name: "checkChatUsernameResultUsernameInvalid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCheckChatUsernameResultUsernameInvalidBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultUsernameInvalidBuilder {
  inner: CheckChatUsernameResultUsernameInvalid,
}

impl RTDCheckChatUsernameResultUsernameInvalidBuilder {
  pub fn build(&self) -> CheckChatUsernameResultUsernameInvalid {
    self.inner.clone()
  }
}

impl AsRef<CheckChatUsernameResultUsernameInvalid> for CheckChatUsernameResultUsernameInvalid {
  fn as_ref(&self) -> &CheckChatUsernameResultUsernameInvalid {
    self
  }
}

impl AsRef<CheckChatUsernameResultUsernameInvalid>
  for RTDCheckChatUsernameResultUsernameInvalidBuilder
{
  fn as_ref(&self) -> &CheckChatUsernameResultUsernameInvalid {
    &self.inner
  }
}

/// The username is occupied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernameOccupied {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CheckChatUsernameResultUsernameOccupied {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "checkChatUsernameResultUsernameOccupied"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultUsernameOccupied {}

impl CheckChatUsernameResultUsernameOccupied {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCheckChatUsernameResultUsernameOccupiedBuilder {
    let inner = CheckChatUsernameResultUsernameOccupied {
      td_name: "checkChatUsernameResultUsernameOccupied".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCheckChatUsernameResultUsernameOccupiedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultUsernameOccupiedBuilder {
  inner: CheckChatUsernameResultUsernameOccupied,
}

impl RTDCheckChatUsernameResultUsernameOccupiedBuilder {
  pub fn build(&self) -> CheckChatUsernameResultUsernameOccupied {
    self.inner.clone()
  }
}

impl AsRef<CheckChatUsernameResultUsernameOccupied> for CheckChatUsernameResultUsernameOccupied {
  fn as_ref(&self) -> &CheckChatUsernameResultUsernameOccupied {
    self
  }
}

impl AsRef<CheckChatUsernameResultUsernameOccupied>
  for RTDCheckChatUsernameResultUsernameOccupiedBuilder
{
  fn as_ref(&self) -> &CheckChatUsernameResultUsernameOccupied {
    &self.inner
  }
}

/// The username can be purchased at https://fragment.com. Information about the username can be received using getCollectibleItemInfo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernamePurchasable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CheckChatUsernameResultUsernamePurchasable {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "checkChatUsernameResultUsernamePurchasable"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultUsernamePurchasable {}

impl CheckChatUsernameResultUsernamePurchasable {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCheckChatUsernameResultUsernamePurchasableBuilder {
    let inner = CheckChatUsernameResultUsernamePurchasable {
      td_name: "checkChatUsernameResultUsernamePurchasable".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCheckChatUsernameResultUsernamePurchasableBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCheckChatUsernameResultUsernamePurchasableBuilder {
  inner: CheckChatUsernameResultUsernamePurchasable,
}

impl RTDCheckChatUsernameResultUsernamePurchasableBuilder {
  pub fn build(&self) -> CheckChatUsernameResultUsernamePurchasable {
    self.inner.clone()
  }
}

impl AsRef<CheckChatUsernameResultUsernamePurchasable>
  for CheckChatUsernameResultUsernamePurchasable
{
  fn as_ref(&self) -> &CheckChatUsernameResultUsernamePurchasable {
    self
  }
}

impl AsRef<CheckChatUsernameResultUsernamePurchasable>
  for RTDCheckChatUsernameResultUsernamePurchasableBuilder
{
  fn as_ref(&self) -> &CheckChatUsernameResultUsernamePurchasable {
    &self.inner
  }
}
