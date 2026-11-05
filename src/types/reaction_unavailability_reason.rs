use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes why the current user can't add reactions to the message, despite some other users can
pub trait TDReactionUnavailabilityReason: Debug + RObject {}

/// Describes why the current user can't add reactions to the message, despite some other users can
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReactionUnavailabilityReason {
  #[doc(hidden)]
  _Default(()),
  /// The user is an anonymous administrator in the supergroup, but isn't a creator of it, so they can't vote on behalf of the supergroup
  AnonymousAdministrator(ReactionUnavailabilityReasonAnonymousAdministrator),
  /// The user isn't a member of the supergroup and can't send messages and reactions there without joining
  Guest(ReactionUnavailabilityReasonGuest),
  /// The user is restricted in the chat
  Restricted(ReactionUnavailabilityReasonRestricted),
}

impl Default for ReactionUnavailabilityReason {
  fn default() -> Self {
    ReactionUnavailabilityReason::_Default(())
  }
}

impl<'de> Deserialize<'de> for ReactionUnavailabilityReason {
  fn deserialize<D>(deserializer: D) -> Result<ReactionUnavailabilityReason, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ReactionUnavailabilityReason,
      (reactionUnavailabilityReasonAnonymousAdministrator, AnonymousAdministrator);
      (reactionUnavailabilityReasonGuest, Guest);
      (reactionUnavailabilityReasonRestricted, Restricted);

    )(deserializer)
  }
}

impl RObject for ReactionUnavailabilityReason {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ReactionUnavailabilityReason::AnonymousAdministrator(t) => t.td_name(),
      ReactionUnavailabilityReason::Guest(t) => t.td_name(),
      ReactionUnavailabilityReason::Restricted(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ReactionUnavailabilityReason::AnonymousAdministrator(t) => t.extra(),
      ReactionUnavailabilityReason::Guest(t) => t.extra(),
      ReactionUnavailabilityReason::Restricted(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReactionUnavailabilityReason {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ReactionUnavailabilityReason::_Default(_))
  }

  pub fn is_anonymous_administrator(&self) -> bool {
    matches!(
      self,
      ReactionUnavailabilityReason::AnonymousAdministrator(_)
    )
  }
  pub fn is_guest(&self) -> bool {
    matches!(self, ReactionUnavailabilityReason::Guest(_))
  }
  pub fn is_restricted(&self) -> bool {
    matches!(self, ReactionUnavailabilityReason::Restricted(_))
  }

  pub fn on_anonymous_administrator<
    F: FnOnce(&ReactionUnavailabilityReasonAnonymousAdministrator),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let ReactionUnavailabilityReason::AnonymousAdministrator(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_guest<F: FnOnce(&ReactionUnavailabilityReasonGuest)>(&self, fnc: F) -> &Self {
    if let ReactionUnavailabilityReason::Guest(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restricted<F: FnOnce(&ReactionUnavailabilityReasonRestricted)>(&self, fnc: F) -> &Self {
    if let ReactionUnavailabilityReason::Restricted(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_anonymous_administrator(
    &self,
  ) -> Option<&ReactionUnavailabilityReasonAnonymousAdministrator> {
    if let ReactionUnavailabilityReason::AnonymousAdministrator(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_guest(&self) -> Option<&ReactionUnavailabilityReasonGuest> {
    if let ReactionUnavailabilityReason::Guest(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restricted(&self) -> Option<&ReactionUnavailabilityReasonRestricted> {
    if let ReactionUnavailabilityReason::Restricted(t) = self {
      return Some(t);
    }
    None
  }

  pub fn anonymous_administrator<T: AsRef<ReactionUnavailabilityReasonAnonymousAdministrator>>(
    t: T,
  ) -> Self {
    ReactionUnavailabilityReason::AnonymousAdministrator(t.as_ref().clone())
  }

  pub fn guest<T: AsRef<ReactionUnavailabilityReasonGuest>>(t: T) -> Self {
    ReactionUnavailabilityReason::Guest(t.as_ref().clone())
  }

  pub fn restricted<T: AsRef<ReactionUnavailabilityReasonRestricted>>(t: T) -> Self {
    ReactionUnavailabilityReason::Restricted(t.as_ref().clone())
  }
}

impl AsRef<ReactionUnavailabilityReason> for ReactionUnavailabilityReason {
  fn as_ref(&self) -> &ReactionUnavailabilityReason {
    self
  }
}

/// The user is an anonymous administrator in the supergroup, but isn't a creator of it, so they can't vote on behalf of the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionUnavailabilityReasonAnonymousAdministrator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReactionUnavailabilityReasonAnonymousAdministrator {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionUnavailabilityReasonAnonymousAdministrator"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionUnavailabilityReason for ReactionUnavailabilityReasonAnonymousAdministrator {}

impl ReactionUnavailabilityReasonAnonymousAdministrator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionUnavailabilityReasonAnonymousAdministratorBuilder {
    let inner = ReactionUnavailabilityReasonAnonymousAdministrator {
      td_name: "reactionUnavailabilityReasonAnonymousAdministrator".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReactionUnavailabilityReasonAnonymousAdministratorBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReactionUnavailabilityReasonAnonymousAdministratorBuilder {
  inner: ReactionUnavailabilityReasonAnonymousAdministrator,
}

impl RTDReactionUnavailabilityReasonAnonymousAdministratorBuilder {
  pub fn build(&self) -> ReactionUnavailabilityReasonAnonymousAdministrator {
    self.inner.clone()
  }
}

impl AsRef<ReactionUnavailabilityReasonAnonymousAdministrator>
  for ReactionUnavailabilityReasonAnonymousAdministrator
{
  fn as_ref(&self) -> &ReactionUnavailabilityReasonAnonymousAdministrator {
    self
  }
}

impl AsRef<ReactionUnavailabilityReasonAnonymousAdministrator>
  for RTDReactionUnavailabilityReasonAnonymousAdministratorBuilder
{
  fn as_ref(&self) -> &ReactionUnavailabilityReasonAnonymousAdministrator {
    &self.inner
  }
}

/// The user isn't a member of the supergroup and can't send messages and reactions there without joining
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionUnavailabilityReasonGuest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReactionUnavailabilityReasonGuest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionUnavailabilityReasonGuest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionUnavailabilityReason for ReactionUnavailabilityReasonGuest {}

impl ReactionUnavailabilityReasonGuest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionUnavailabilityReasonGuestBuilder {
    let inner = ReactionUnavailabilityReasonGuest {
      td_name: "reactionUnavailabilityReasonGuest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReactionUnavailabilityReasonGuestBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReactionUnavailabilityReasonGuestBuilder {
  inner: ReactionUnavailabilityReasonGuest,
}

impl RTDReactionUnavailabilityReasonGuestBuilder {
  pub fn build(&self) -> ReactionUnavailabilityReasonGuest {
    self.inner.clone()
  }
}

impl AsRef<ReactionUnavailabilityReasonGuest> for ReactionUnavailabilityReasonGuest {
  fn as_ref(&self) -> &ReactionUnavailabilityReasonGuest {
    self
  }
}

impl AsRef<ReactionUnavailabilityReasonGuest> for RTDReactionUnavailabilityReasonGuestBuilder {
  fn as_ref(&self) -> &ReactionUnavailabilityReasonGuest {
    &self.inner
  }
}

/// The user is restricted in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionUnavailabilityReasonRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReactionUnavailabilityReasonRestricted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionUnavailabilityReasonRestricted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionUnavailabilityReason for ReactionUnavailabilityReasonRestricted {}

impl ReactionUnavailabilityReasonRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionUnavailabilityReasonRestrictedBuilder {
    let inner = ReactionUnavailabilityReasonRestricted {
      td_name: "reactionUnavailabilityReasonRestricted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReactionUnavailabilityReasonRestrictedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReactionUnavailabilityReasonRestrictedBuilder {
  inner: ReactionUnavailabilityReasonRestricted,
}

impl RTDReactionUnavailabilityReasonRestrictedBuilder {
  pub fn build(&self) -> ReactionUnavailabilityReasonRestricted {
    self.inner.clone()
  }
}

impl AsRef<ReactionUnavailabilityReasonRestricted> for ReactionUnavailabilityReasonRestricted {
  fn as_ref(&self) -> &ReactionUnavailabilityReasonRestricted {
    self
  }
}

impl AsRef<ReactionUnavailabilityReasonRestricted>
  for RTDReactionUnavailabilityReasonRestrictedBuilder
{
  fn as_ref(&self) -> &ReactionUnavailabilityReasonRestricted {
    &self.inner
  }
}
