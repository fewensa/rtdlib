use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a single rule for managing user privacy settings
pub trait TDUserPrivacySettingRule: Debug + RObject {}

/// Represents a single rule for managing user privacy settings
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UserPrivacySettingRule {
  #[doc(hidden)]
  _Default(()),
  /// A rule to allow all users to do something
  AllowAll(UserPrivacySettingRuleAllowAll),
  /// A rule to allow all bots to do something
  AllowBots(UserPrivacySettingRuleAllowBots),
  /// A rule to allow all members of certain specified basic groups and supergroups to do something
  AllowChatMembers(UserPrivacySettingRuleAllowChatMembers),
  /// A rule to allow all contacts of the user to do something
  AllowContacts(UserPrivacySettingRuleAllowContacts),
  /// A rule to allow all Premium Users to do something; currently, allowed only for userPrivacySettingAllowChatInvites
  AllowPremiumUsers(UserPrivacySettingRuleAllowPremiumUsers),
  /// A rule to allow certain specified users to do something
  AllowUsers(UserPrivacySettingRuleAllowUsers),
  /// A rule to restrict all users from doing something
  RestrictAll(UserPrivacySettingRuleRestrictAll),
  /// A rule to restrict all bots from doing something
  RestrictBots(UserPrivacySettingRuleRestrictBots),
  /// A rule to restrict all members of specified basic groups and supergroups from doing something
  RestrictChatMembers(UserPrivacySettingRuleRestrictChatMembers),
  /// A rule to restrict all contacts of the user from doing something
  RestrictContacts(UserPrivacySettingRuleRestrictContacts),
  /// A rule to restrict all specified users from doing something
  RestrictUsers(UserPrivacySettingRuleRestrictUsers),
}

impl Default for UserPrivacySettingRule {
  fn default() -> Self {
    UserPrivacySettingRule::_Default(())
  }
}

impl<'de> Deserialize<'de> for UserPrivacySettingRule {
  fn deserialize<D>(deserializer: D) -> Result<UserPrivacySettingRule, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UserPrivacySettingRule,
      (userPrivacySettingRuleAllowAll, AllowAll);
      (userPrivacySettingRuleAllowBots, AllowBots);
      (userPrivacySettingRuleAllowChatMembers, AllowChatMembers);
      (userPrivacySettingRuleAllowContacts, AllowContacts);
      (userPrivacySettingRuleAllowPremiumUsers, AllowPremiumUsers);
      (userPrivacySettingRuleAllowUsers, AllowUsers);
      (userPrivacySettingRuleRestrictAll, RestrictAll);
      (userPrivacySettingRuleRestrictBots, RestrictBots);
      (userPrivacySettingRuleRestrictChatMembers, RestrictChatMembers);
      (userPrivacySettingRuleRestrictContacts, RestrictContacts);
      (userPrivacySettingRuleRestrictUsers, RestrictUsers);

    )(deserializer)
  }
}

impl RObject for UserPrivacySettingRule {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      UserPrivacySettingRule::AllowAll(t) => t.td_name(),
      UserPrivacySettingRule::AllowBots(t) => t.td_name(),
      UserPrivacySettingRule::AllowChatMembers(t) => t.td_name(),
      UserPrivacySettingRule::AllowContacts(t) => t.td_name(),
      UserPrivacySettingRule::AllowPremiumUsers(t) => t.td_name(),
      UserPrivacySettingRule::AllowUsers(t) => t.td_name(),
      UserPrivacySettingRule::RestrictAll(t) => t.td_name(),
      UserPrivacySettingRule::RestrictBots(t) => t.td_name(),
      UserPrivacySettingRule::RestrictChatMembers(t) => t.td_name(),
      UserPrivacySettingRule::RestrictContacts(t) => t.td_name(),
      UserPrivacySettingRule::RestrictUsers(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      UserPrivacySettingRule::AllowAll(t) => t.extra(),
      UserPrivacySettingRule::AllowBots(t) => t.extra(),
      UserPrivacySettingRule::AllowChatMembers(t) => t.extra(),
      UserPrivacySettingRule::AllowContacts(t) => t.extra(),
      UserPrivacySettingRule::AllowPremiumUsers(t) => t.extra(),
      UserPrivacySettingRule::AllowUsers(t) => t.extra(),
      UserPrivacySettingRule::RestrictAll(t) => t.extra(),
      UserPrivacySettingRule::RestrictBots(t) => t.extra(),
      UserPrivacySettingRule::RestrictChatMembers(t) => t.extra(),
      UserPrivacySettingRule::RestrictContacts(t) => t.extra(),
      UserPrivacySettingRule::RestrictUsers(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserPrivacySettingRule {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, UserPrivacySettingRule::_Default(_))
  }

  pub fn is_allow_all(&self) -> bool {
    matches!(self, UserPrivacySettingRule::AllowAll(_))
  }
  pub fn is_allow_bots(&self) -> bool {
    matches!(self, UserPrivacySettingRule::AllowBots(_))
  }
  pub fn is_allow_chat_members(&self) -> bool {
    matches!(self, UserPrivacySettingRule::AllowChatMembers(_))
  }
  pub fn is_allow_contacts(&self) -> bool {
    matches!(self, UserPrivacySettingRule::AllowContacts(_))
  }
  pub fn is_allow_premium_users(&self) -> bool {
    matches!(self, UserPrivacySettingRule::AllowPremiumUsers(_))
  }
  pub fn is_allow_users(&self) -> bool {
    matches!(self, UserPrivacySettingRule::AllowUsers(_))
  }
  pub fn is_restrict_all(&self) -> bool {
    matches!(self, UserPrivacySettingRule::RestrictAll(_))
  }
  pub fn is_restrict_bots(&self) -> bool {
    matches!(self, UserPrivacySettingRule::RestrictBots(_))
  }
  pub fn is_restrict_chat_members(&self) -> bool {
    matches!(self, UserPrivacySettingRule::RestrictChatMembers(_))
  }
  pub fn is_restrict_contacts(&self) -> bool {
    matches!(self, UserPrivacySettingRule::RestrictContacts(_))
  }
  pub fn is_restrict_users(&self) -> bool {
    matches!(self, UserPrivacySettingRule::RestrictUsers(_))
  }

  pub fn on_allow_all<F: FnOnce(&UserPrivacySettingRuleAllowAll)>(&self, fnc: F) -> &Self {
    if let UserPrivacySettingRule::AllowAll(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_bots<F: FnOnce(&UserPrivacySettingRuleAllowBots)>(&self, fnc: F) -> &Self {
    if let UserPrivacySettingRule::AllowBots(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_chat_members<F: FnOnce(&UserPrivacySettingRuleAllowChatMembers)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySettingRule::AllowChatMembers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_contacts<F: FnOnce(&UserPrivacySettingRuleAllowContacts)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySettingRule::AllowContacts(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_premium_users<F: FnOnce(&UserPrivacySettingRuleAllowPremiumUsers)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySettingRule::AllowPremiumUsers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_users<F: FnOnce(&UserPrivacySettingRuleAllowUsers)>(&self, fnc: F) -> &Self {
    if let UserPrivacySettingRule::AllowUsers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restrict_all<F: FnOnce(&UserPrivacySettingRuleRestrictAll)>(&self, fnc: F) -> &Self {
    if let UserPrivacySettingRule::RestrictAll(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restrict_bots<F: FnOnce(&UserPrivacySettingRuleRestrictBots)>(&self, fnc: F) -> &Self {
    if let UserPrivacySettingRule::RestrictBots(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restrict_chat_members<F: FnOnce(&UserPrivacySettingRuleRestrictChatMembers)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySettingRule::RestrictChatMembers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restrict_contacts<F: FnOnce(&UserPrivacySettingRuleRestrictContacts)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySettingRule::RestrictContacts(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restrict_users<F: FnOnce(&UserPrivacySettingRuleRestrictUsers)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySettingRule::RestrictUsers(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_allow_all(&self) -> Option<&UserPrivacySettingRuleAllowAll> {
    if let UserPrivacySettingRule::AllowAll(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_bots(&self) -> Option<&UserPrivacySettingRuleAllowBots> {
    if let UserPrivacySettingRule::AllowBots(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_chat_members(&self) -> Option<&UserPrivacySettingRuleAllowChatMembers> {
    if let UserPrivacySettingRule::AllowChatMembers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_contacts(&self) -> Option<&UserPrivacySettingRuleAllowContacts> {
    if let UserPrivacySettingRule::AllowContacts(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_premium_users(&self) -> Option<&UserPrivacySettingRuleAllowPremiumUsers> {
    if let UserPrivacySettingRule::AllowPremiumUsers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_users(&self) -> Option<&UserPrivacySettingRuleAllowUsers> {
    if let UserPrivacySettingRule::AllowUsers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restrict_all(&self) -> Option<&UserPrivacySettingRuleRestrictAll> {
    if let UserPrivacySettingRule::RestrictAll(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restrict_bots(&self) -> Option<&UserPrivacySettingRuleRestrictBots> {
    if let UserPrivacySettingRule::RestrictBots(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restrict_chat_members(&self) -> Option<&UserPrivacySettingRuleRestrictChatMembers> {
    if let UserPrivacySettingRule::RestrictChatMembers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restrict_contacts(&self) -> Option<&UserPrivacySettingRuleRestrictContacts> {
    if let UserPrivacySettingRule::RestrictContacts(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restrict_users(&self) -> Option<&UserPrivacySettingRuleRestrictUsers> {
    if let UserPrivacySettingRule::RestrictUsers(t) = self {
      return Some(t);
    }
    None
  }

  pub fn allow_all<T: AsRef<UserPrivacySettingRuleAllowAll>>(t: T) -> Self {
    UserPrivacySettingRule::AllowAll(t.as_ref().clone())
  }

  pub fn allow_bots<T: AsRef<UserPrivacySettingRuleAllowBots>>(t: T) -> Self {
    UserPrivacySettingRule::AllowBots(t.as_ref().clone())
  }

  pub fn allow_chat_members<T: AsRef<UserPrivacySettingRuleAllowChatMembers>>(t: T) -> Self {
    UserPrivacySettingRule::AllowChatMembers(t.as_ref().clone())
  }

  pub fn allow_contacts<T: AsRef<UserPrivacySettingRuleAllowContacts>>(t: T) -> Self {
    UserPrivacySettingRule::AllowContacts(t.as_ref().clone())
  }

  pub fn allow_premium_users<T: AsRef<UserPrivacySettingRuleAllowPremiumUsers>>(t: T) -> Self {
    UserPrivacySettingRule::AllowPremiumUsers(t.as_ref().clone())
  }

  pub fn allow_users<T: AsRef<UserPrivacySettingRuleAllowUsers>>(t: T) -> Self {
    UserPrivacySettingRule::AllowUsers(t.as_ref().clone())
  }

  pub fn restrict_all<T: AsRef<UserPrivacySettingRuleRestrictAll>>(t: T) -> Self {
    UserPrivacySettingRule::RestrictAll(t.as_ref().clone())
  }

  pub fn restrict_bots<T: AsRef<UserPrivacySettingRuleRestrictBots>>(t: T) -> Self {
    UserPrivacySettingRule::RestrictBots(t.as_ref().clone())
  }

  pub fn restrict_chat_members<T: AsRef<UserPrivacySettingRuleRestrictChatMembers>>(t: T) -> Self {
    UserPrivacySettingRule::RestrictChatMembers(t.as_ref().clone())
  }

  pub fn restrict_contacts<T: AsRef<UserPrivacySettingRuleRestrictContacts>>(t: T) -> Self {
    UserPrivacySettingRule::RestrictContacts(t.as_ref().clone())
  }

  pub fn restrict_users<T: AsRef<UserPrivacySettingRuleRestrictUsers>>(t: T) -> Self {
    UserPrivacySettingRule::RestrictUsers(t.as_ref().clone())
  }
}

impl AsRef<UserPrivacySettingRule> for UserPrivacySettingRule {
  fn as_ref(&self) -> &UserPrivacySettingRule {
    self
  }
}

/// A rule to allow all users to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowAll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingRuleAllowAll {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleAllowAll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowAll {}

impl UserPrivacySettingRuleAllowAll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowAllBuilder {
    let inner = UserPrivacySettingRuleAllowAll {
      td_name: "userPrivacySettingRuleAllowAll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingRuleAllowAllBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowAllBuilder {
  inner: UserPrivacySettingRuleAllowAll,
}

impl RTDUserPrivacySettingRuleAllowAllBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowAll {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingRuleAllowAll> for UserPrivacySettingRuleAllowAll {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowAll {
    self
  }
}

impl AsRef<UserPrivacySettingRuleAllowAll> for RTDUserPrivacySettingRuleAllowAllBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowAll {
    &self.inner
  }
}

/// A rule to allow all bots to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingRuleAllowBots {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleAllowBots"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowBots {}

impl UserPrivacySettingRuleAllowBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowBotsBuilder {
    let inner = UserPrivacySettingRuleAllowBots {
      td_name: "userPrivacySettingRuleAllowBots".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingRuleAllowBotsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowBotsBuilder {
  inner: UserPrivacySettingRuleAllowBots,
}

impl RTDUserPrivacySettingRuleAllowBotsBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowBots {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingRuleAllowBots> for UserPrivacySettingRuleAllowBots {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowBots {
    self
  }
}

impl AsRef<UserPrivacySettingRuleAllowBots> for RTDUserPrivacySettingRuleAllowBotsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowBots {
    &self.inner
  }
}

/// A rule to allow all members of certain specified basic groups and supergroups to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat identifiers, total number of chats in all rules must not exceed 20
  chat_ids: Vec<i64>,
}

impl RObject for UserPrivacySettingRuleAllowChatMembers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleAllowChatMembers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowChatMembers {}

impl UserPrivacySettingRuleAllowChatMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowChatMembersBuilder {
    let inner = UserPrivacySettingRuleAllowChatMembers {
      td_name: "userPrivacySettingRuleAllowChatMembers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserPrivacySettingRuleAllowChatMembersBuilder { inner }
  }

  pub fn chat_ids(&self) -> &Vec<i64> {
    &self.chat_ids
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowChatMembersBuilder {
  inner: UserPrivacySettingRuleAllowChatMembers,
}

impl RTDUserPrivacySettingRuleAllowChatMembersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowChatMembers {
    self.inner.clone()
  }

  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }
}

impl AsRef<UserPrivacySettingRuleAllowChatMembers> for UserPrivacySettingRuleAllowChatMembers {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowChatMembers {
    self
  }
}

impl AsRef<UserPrivacySettingRuleAllowChatMembers>
  for RTDUserPrivacySettingRuleAllowChatMembersBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowChatMembers {
    &self.inner
  }
}

/// A rule to allow all contacts of the user to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingRuleAllowContacts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleAllowContacts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowContacts {}

impl UserPrivacySettingRuleAllowContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowContactsBuilder {
    let inner = UserPrivacySettingRuleAllowContacts {
      td_name: "userPrivacySettingRuleAllowContacts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingRuleAllowContactsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowContactsBuilder {
  inner: UserPrivacySettingRuleAllowContacts,
}

impl RTDUserPrivacySettingRuleAllowContactsBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowContacts {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingRuleAllowContacts> for UserPrivacySettingRuleAllowContacts {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowContacts {
    self
  }
}

impl AsRef<UserPrivacySettingRuleAllowContacts> for RTDUserPrivacySettingRuleAllowContactsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowContacts {
    &self.inner
  }
}

/// A rule to allow all Premium Users to do something; currently, allowed only for userPrivacySettingAllowChatInvites
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowPremiumUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingRuleAllowPremiumUsers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleAllowPremiumUsers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowPremiumUsers {}

impl UserPrivacySettingRuleAllowPremiumUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowPremiumUsersBuilder {
    let inner = UserPrivacySettingRuleAllowPremiumUsers {
      td_name: "userPrivacySettingRuleAllowPremiumUsers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingRuleAllowPremiumUsersBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowPremiumUsersBuilder {
  inner: UserPrivacySettingRuleAllowPremiumUsers,
}

impl RTDUserPrivacySettingRuleAllowPremiumUsersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowPremiumUsers {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingRuleAllowPremiumUsers> for UserPrivacySettingRuleAllowPremiumUsers {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowPremiumUsers {
    self
  }
}

impl AsRef<UserPrivacySettingRuleAllowPremiumUsers>
  for RTDUserPrivacySettingRuleAllowPremiumUsersBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowPremiumUsers {
    &self.inner
  }
}

/// A rule to allow certain specified users to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The user identifiers, total number of users in all rules must not exceed 1000
  user_ids: Vec<i64>,
}

impl RObject for UserPrivacySettingRuleAllowUsers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleAllowUsers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowUsers {}

impl UserPrivacySettingRuleAllowUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowUsersBuilder {
    let inner = UserPrivacySettingRuleAllowUsers {
      td_name: "userPrivacySettingRuleAllowUsers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserPrivacySettingRuleAllowUsersBuilder { inner }
  }

  pub fn user_ids(&self) -> &Vec<i64> {
    &self.user_ids
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowUsersBuilder {
  inner: UserPrivacySettingRuleAllowUsers,
}

impl RTDUserPrivacySettingRuleAllowUsersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowUsers {
    self.inner.clone()
  }

  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }
}

impl AsRef<UserPrivacySettingRuleAllowUsers> for UserPrivacySettingRuleAllowUsers {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowUsers {
    self
  }
}

impl AsRef<UserPrivacySettingRuleAllowUsers> for RTDUserPrivacySettingRuleAllowUsersBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowUsers {
    &self.inner
  }
}

/// A rule to restrict all users from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictAll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingRuleRestrictAll {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleRestrictAll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictAll {}

impl UserPrivacySettingRuleRestrictAll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictAllBuilder {
    let inner = UserPrivacySettingRuleRestrictAll {
      td_name: "userPrivacySettingRuleRestrictAll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingRuleRestrictAllBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictAllBuilder {
  inner: UserPrivacySettingRuleRestrictAll,
}

impl RTDUserPrivacySettingRuleRestrictAllBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictAll {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingRuleRestrictAll> for UserPrivacySettingRuleRestrictAll {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictAll {
    self
  }
}

impl AsRef<UserPrivacySettingRuleRestrictAll> for RTDUserPrivacySettingRuleRestrictAllBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictAll {
    &self.inner
  }
}

/// A rule to restrict all bots from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingRuleRestrictBots {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleRestrictBots"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictBots {}

impl UserPrivacySettingRuleRestrictBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictBotsBuilder {
    let inner = UserPrivacySettingRuleRestrictBots {
      td_name: "userPrivacySettingRuleRestrictBots".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingRuleRestrictBotsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictBotsBuilder {
  inner: UserPrivacySettingRuleRestrictBots,
}

impl RTDUserPrivacySettingRuleRestrictBotsBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictBots {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingRuleRestrictBots> for UserPrivacySettingRuleRestrictBots {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictBots {
    self
  }
}

impl AsRef<UserPrivacySettingRuleRestrictBots> for RTDUserPrivacySettingRuleRestrictBotsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictBots {
    &self.inner
  }
}

/// A rule to restrict all members of specified basic groups and supergroups from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat identifiers, total number of chats in all rules must not exceed 20
  chat_ids: Vec<i64>,
}

impl RObject for UserPrivacySettingRuleRestrictChatMembers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleRestrictChatMembers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictChatMembers {}

impl UserPrivacySettingRuleRestrictChatMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
    let inner = UserPrivacySettingRuleRestrictChatMembers {
      td_name: "userPrivacySettingRuleRestrictChatMembers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserPrivacySettingRuleRestrictChatMembersBuilder { inner }
  }

  pub fn chat_ids(&self) -> &Vec<i64> {
    &self.chat_ids
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
  inner: UserPrivacySettingRuleRestrictChatMembers,
}

impl RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictChatMembers {
    self.inner.clone()
  }

  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }
}

impl AsRef<UserPrivacySettingRuleRestrictChatMembers>
  for UserPrivacySettingRuleRestrictChatMembers
{
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictChatMembers {
    self
  }
}

impl AsRef<UserPrivacySettingRuleRestrictChatMembers>
  for RTDUserPrivacySettingRuleRestrictChatMembersBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictChatMembers {
    &self.inner
  }
}

/// A rule to restrict all contacts of the user from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingRuleRestrictContacts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleRestrictContacts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictContacts {}

impl UserPrivacySettingRuleRestrictContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictContactsBuilder {
    let inner = UserPrivacySettingRuleRestrictContacts {
      td_name: "userPrivacySettingRuleRestrictContacts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingRuleRestrictContactsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictContactsBuilder {
  inner: UserPrivacySettingRuleRestrictContacts,
}

impl RTDUserPrivacySettingRuleRestrictContactsBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictContacts {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingRuleRestrictContacts> for UserPrivacySettingRuleRestrictContacts {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictContacts {
    self
  }
}

impl AsRef<UserPrivacySettingRuleRestrictContacts>
  for RTDUserPrivacySettingRuleRestrictContactsBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictContacts {
    &self.inner
  }
}

/// A rule to restrict all specified users from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The user identifiers, total number of users in all rules must not exceed 1000
  user_ids: Vec<i64>,
}

impl RObject for UserPrivacySettingRuleRestrictUsers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingRuleRestrictUsers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictUsers {}

impl UserPrivacySettingRuleRestrictUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictUsersBuilder {
    let inner = UserPrivacySettingRuleRestrictUsers {
      td_name: "userPrivacySettingRuleRestrictUsers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserPrivacySettingRuleRestrictUsersBuilder { inner }
  }

  pub fn user_ids(&self) -> &Vec<i64> {
    &self.user_ids
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictUsersBuilder {
  inner: UserPrivacySettingRuleRestrictUsers,
}

impl RTDUserPrivacySettingRuleRestrictUsersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictUsers {
    self.inner.clone()
  }

  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }
}

impl AsRef<UserPrivacySettingRuleRestrictUsers> for UserPrivacySettingRuleRestrictUsers {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictUsers {
    self
  }
}

impl AsRef<UserPrivacySettingRuleRestrictUsers> for RTDUserPrivacySettingRuleRestrictUsersBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictUsers {
    &self.inner
  }
}
