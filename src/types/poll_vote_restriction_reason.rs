use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Reason of vote restriction in the poll for the current user
pub trait TDPollVoteRestrictionReason: Debug + RObject {}

/// Reason of vote restriction in the poll for the current user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PollVoteRestrictionReason {
  #[doc(hidden)]
  _Default(()),
  /// The poll is closed
  Closed(PollVoteRestrictionReasonClosed),
  /// The user is from a country, users from which aren't allowed to vote
  CountryRestricted(PollVoteRestrictionReasonCountryRestricted),
  /// The user must be a member of the chat for at least a day to vote
  MembershipRequired(PollVoteRestrictionReasonMembershipRequired),
  /// The poll can't be voted by the user due to some other reason
  Other(PollVoteRestrictionReasonOther),
  /// The poll is from a scheduled message
  Scheduled(PollVoteRestrictionReasonScheduled),
  /// The poll isn't sent yet
  YetUnsent(PollVoteRestrictionReasonYetUnsent),
}

impl Default for PollVoteRestrictionReason {
  fn default() -> Self {
    PollVoteRestrictionReason::_Default(())
  }
}

impl<'de> Deserialize<'de> for PollVoteRestrictionReason {
  fn deserialize<D>(deserializer: D) -> Result<PollVoteRestrictionReason, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PollVoteRestrictionReason,
      (pollVoteRestrictionReasonClosed, Closed);
      (pollVoteRestrictionReasonCountryRestricted, CountryRestricted);
      (pollVoteRestrictionReasonMembershipRequired, MembershipRequired);
      (pollVoteRestrictionReasonOther, Other);
      (pollVoteRestrictionReasonScheduled, Scheduled);
      (pollVoteRestrictionReasonYetUnsent, YetUnsent);

    )(deserializer)
  }
}

impl RObject for PollVoteRestrictionReason {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PollVoteRestrictionReason::Closed(t) => t.td_name(),
      PollVoteRestrictionReason::CountryRestricted(t) => t.td_name(),
      PollVoteRestrictionReason::MembershipRequired(t) => t.td_name(),
      PollVoteRestrictionReason::Other(t) => t.td_name(),
      PollVoteRestrictionReason::Scheduled(t) => t.td_name(),
      PollVoteRestrictionReason::YetUnsent(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PollVoteRestrictionReason::Closed(t) => t.extra(),
      PollVoteRestrictionReason::CountryRestricted(t) => t.extra(),
      PollVoteRestrictionReason::MembershipRequired(t) => t.extra(),
      PollVoteRestrictionReason::Other(t) => t.extra(),
      PollVoteRestrictionReason::Scheduled(t) => t.extra(),
      PollVoteRestrictionReason::YetUnsent(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PollVoteRestrictionReason {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PollVoteRestrictionReason::_Default(_))
  }

  pub fn is_closed(&self) -> bool {
    matches!(self, PollVoteRestrictionReason::Closed(_))
  }
  pub fn is_country_restricted(&self) -> bool {
    matches!(self, PollVoteRestrictionReason::CountryRestricted(_))
  }
  pub fn is_membership_required(&self) -> bool {
    matches!(self, PollVoteRestrictionReason::MembershipRequired(_))
  }
  pub fn is_other(&self) -> bool {
    matches!(self, PollVoteRestrictionReason::Other(_))
  }
  pub fn is_scheduled(&self) -> bool {
    matches!(self, PollVoteRestrictionReason::Scheduled(_))
  }
  pub fn is_yet_unsent(&self) -> bool {
    matches!(self, PollVoteRestrictionReason::YetUnsent(_))
  }

  pub fn on_closed<F: FnOnce(&PollVoteRestrictionReasonClosed)>(&self, fnc: F) -> &Self {
    if let PollVoteRestrictionReason::Closed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_country_restricted<F: FnOnce(&PollVoteRestrictionReasonCountryRestricted)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PollVoteRestrictionReason::CountryRestricted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_membership_required<F: FnOnce(&PollVoteRestrictionReasonMembershipRequired)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PollVoteRestrictionReason::MembershipRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_other<F: FnOnce(&PollVoteRestrictionReasonOther)>(&self, fnc: F) -> &Self {
    if let PollVoteRestrictionReason::Other(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_scheduled<F: FnOnce(&PollVoteRestrictionReasonScheduled)>(&self, fnc: F) -> &Self {
    if let PollVoteRestrictionReason::Scheduled(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_yet_unsent<F: FnOnce(&PollVoteRestrictionReasonYetUnsent)>(&self, fnc: F) -> &Self {
    if let PollVoteRestrictionReason::YetUnsent(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_closed(&self) -> Option<&PollVoteRestrictionReasonClosed> {
    if let PollVoteRestrictionReason::Closed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_country_restricted(&self) -> Option<&PollVoteRestrictionReasonCountryRestricted> {
    if let PollVoteRestrictionReason::CountryRestricted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_membership_required(&self) -> Option<&PollVoteRestrictionReasonMembershipRequired> {
    if let PollVoteRestrictionReason::MembershipRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_other(&self) -> Option<&PollVoteRestrictionReasonOther> {
    if let PollVoteRestrictionReason::Other(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_scheduled(&self) -> Option<&PollVoteRestrictionReasonScheduled> {
    if let PollVoteRestrictionReason::Scheduled(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_yet_unsent(&self) -> Option<&PollVoteRestrictionReasonYetUnsent> {
    if let PollVoteRestrictionReason::YetUnsent(t) = self {
      return Some(t);
    }
    None
  }

  pub fn closed<T: AsRef<PollVoteRestrictionReasonClosed>>(t: T) -> Self {
    PollVoteRestrictionReason::Closed(t.as_ref().clone())
  }

  pub fn country_restricted<T: AsRef<PollVoteRestrictionReasonCountryRestricted>>(t: T) -> Self {
    PollVoteRestrictionReason::CountryRestricted(t.as_ref().clone())
  }

  pub fn membership_required<T: AsRef<PollVoteRestrictionReasonMembershipRequired>>(t: T) -> Self {
    PollVoteRestrictionReason::MembershipRequired(t.as_ref().clone())
  }

  pub fn other<T: AsRef<PollVoteRestrictionReasonOther>>(t: T) -> Self {
    PollVoteRestrictionReason::Other(t.as_ref().clone())
  }

  pub fn scheduled<T: AsRef<PollVoteRestrictionReasonScheduled>>(t: T) -> Self {
    PollVoteRestrictionReason::Scheduled(t.as_ref().clone())
  }

  pub fn yet_unsent<T: AsRef<PollVoteRestrictionReasonYetUnsent>>(t: T) -> Self {
    PollVoteRestrictionReason::YetUnsent(t.as_ref().clone())
  }
}

impl AsRef<PollVoteRestrictionReason> for PollVoteRestrictionReason {
  fn as_ref(&self) -> &PollVoteRestrictionReason {
    self
  }
}

/// The poll is closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoteRestrictionReasonClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PollVoteRestrictionReasonClosed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoteRestrictionReasonClosed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollVoteRestrictionReason for PollVoteRestrictionReasonClosed {}

impl PollVoteRestrictionReasonClosed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVoteRestrictionReasonClosedBuilder {
    let inner = PollVoteRestrictionReasonClosed {
      td_name: "pollVoteRestrictionReasonClosed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPollVoteRestrictionReasonClosedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPollVoteRestrictionReasonClosedBuilder {
  inner: PollVoteRestrictionReasonClosed,
}

impl RTDPollVoteRestrictionReasonClosedBuilder {
  pub fn build(&self) -> PollVoteRestrictionReasonClosed {
    self.inner.clone()
  }
}

impl AsRef<PollVoteRestrictionReasonClosed> for PollVoteRestrictionReasonClosed {
  fn as_ref(&self) -> &PollVoteRestrictionReasonClosed {
    self
  }
}

impl AsRef<PollVoteRestrictionReasonClosed> for RTDPollVoteRestrictionReasonClosedBuilder {
  fn as_ref(&self) -> &PollVoteRestrictionReasonClosed {
    &self.inner
  }
}

/// The user is from a country, users from which aren't allowed to vote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoteRestrictionReasonCountryRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Two-letter ISO 3166-1 alpha-2 code of the current user's country
  country_code: String,
}

impl RObject for PollVoteRestrictionReasonCountryRestricted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoteRestrictionReasonCountryRestricted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollVoteRestrictionReason for PollVoteRestrictionReasonCountryRestricted {}

impl PollVoteRestrictionReasonCountryRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVoteRestrictionReasonCountryRestrictedBuilder {
    let inner = PollVoteRestrictionReasonCountryRestricted {
      td_name: "pollVoteRestrictionReasonCountryRestricted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollVoteRestrictionReasonCountryRestrictedBuilder { inner }
  }

  pub fn country_code(&self) -> &String {
    &self.country_code
  }
}

#[doc(hidden)]
pub struct RTDPollVoteRestrictionReasonCountryRestrictedBuilder {
  inner: PollVoteRestrictionReasonCountryRestricted,
}

impl RTDPollVoteRestrictionReasonCountryRestrictedBuilder {
  pub fn build(&self) -> PollVoteRestrictionReasonCountryRestricted {
    self.inner.clone()
  }

  pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
    self.inner.country_code = country_code.as_ref().to_string();
    self
  }
}

impl AsRef<PollVoteRestrictionReasonCountryRestricted>
  for PollVoteRestrictionReasonCountryRestricted
{
  fn as_ref(&self) -> &PollVoteRestrictionReasonCountryRestricted {
    self
  }
}

impl AsRef<PollVoteRestrictionReasonCountryRestricted>
  for RTDPollVoteRestrictionReasonCountryRestrictedBuilder
{
  fn as_ref(&self) -> &PollVoteRestrictionReasonCountryRestricted {
    &self.inner
  }
}

/// The user must be a member of the chat for at least a day to vote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoteRestrictionReasonMembershipRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat which must be joined for at least a day before the user can vote
  chat_id: i64,
}

impl RObject for PollVoteRestrictionReasonMembershipRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoteRestrictionReasonMembershipRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollVoteRestrictionReason for PollVoteRestrictionReasonMembershipRequired {}

impl PollVoteRestrictionReasonMembershipRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVoteRestrictionReasonMembershipRequiredBuilder {
    let inner = PollVoteRestrictionReasonMembershipRequired {
      td_name: "pollVoteRestrictionReasonMembershipRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollVoteRestrictionReasonMembershipRequiredBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDPollVoteRestrictionReasonMembershipRequiredBuilder {
  inner: PollVoteRestrictionReasonMembershipRequired,
}

impl RTDPollVoteRestrictionReasonMembershipRequiredBuilder {
  pub fn build(&self) -> PollVoteRestrictionReasonMembershipRequired {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<PollVoteRestrictionReasonMembershipRequired>
  for PollVoteRestrictionReasonMembershipRequired
{
  fn as_ref(&self) -> &PollVoteRestrictionReasonMembershipRequired {
    self
  }
}

impl AsRef<PollVoteRestrictionReasonMembershipRequired>
  for RTDPollVoteRestrictionReasonMembershipRequiredBuilder
{
  fn as_ref(&self) -> &PollVoteRestrictionReasonMembershipRequired {
    &self.inner
  }
}

/// The poll can't be voted by the user due to some other reason
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoteRestrictionReasonOther {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PollVoteRestrictionReasonOther {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoteRestrictionReasonOther"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollVoteRestrictionReason for PollVoteRestrictionReasonOther {}

impl PollVoteRestrictionReasonOther {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVoteRestrictionReasonOtherBuilder {
    let inner = PollVoteRestrictionReasonOther {
      td_name: "pollVoteRestrictionReasonOther".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPollVoteRestrictionReasonOtherBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPollVoteRestrictionReasonOtherBuilder {
  inner: PollVoteRestrictionReasonOther,
}

impl RTDPollVoteRestrictionReasonOtherBuilder {
  pub fn build(&self) -> PollVoteRestrictionReasonOther {
    self.inner.clone()
  }
}

impl AsRef<PollVoteRestrictionReasonOther> for PollVoteRestrictionReasonOther {
  fn as_ref(&self) -> &PollVoteRestrictionReasonOther {
    self
  }
}

impl AsRef<PollVoteRestrictionReasonOther> for RTDPollVoteRestrictionReasonOtherBuilder {
  fn as_ref(&self) -> &PollVoteRestrictionReasonOther {
    &self.inner
  }
}

/// The poll is from a scheduled message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoteRestrictionReasonScheduled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PollVoteRestrictionReasonScheduled {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoteRestrictionReasonScheduled"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollVoteRestrictionReason for PollVoteRestrictionReasonScheduled {}

impl PollVoteRestrictionReasonScheduled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVoteRestrictionReasonScheduledBuilder {
    let inner = PollVoteRestrictionReasonScheduled {
      td_name: "pollVoteRestrictionReasonScheduled".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPollVoteRestrictionReasonScheduledBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPollVoteRestrictionReasonScheduledBuilder {
  inner: PollVoteRestrictionReasonScheduled,
}

impl RTDPollVoteRestrictionReasonScheduledBuilder {
  pub fn build(&self) -> PollVoteRestrictionReasonScheduled {
    self.inner.clone()
  }
}

impl AsRef<PollVoteRestrictionReasonScheduled> for PollVoteRestrictionReasonScheduled {
  fn as_ref(&self) -> &PollVoteRestrictionReasonScheduled {
    self
  }
}

impl AsRef<PollVoteRestrictionReasonScheduled> for RTDPollVoteRestrictionReasonScheduledBuilder {
  fn as_ref(&self) -> &PollVoteRestrictionReasonScheduled {
    &self.inner
  }
}

/// The poll isn't sent yet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoteRestrictionReasonYetUnsent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PollVoteRestrictionReasonYetUnsent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoteRestrictionReasonYetUnsent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPollVoteRestrictionReason for PollVoteRestrictionReasonYetUnsent {}

impl PollVoteRestrictionReasonYetUnsent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVoteRestrictionReasonYetUnsentBuilder {
    let inner = PollVoteRestrictionReasonYetUnsent {
      td_name: "pollVoteRestrictionReasonYetUnsent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPollVoteRestrictionReasonYetUnsentBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPollVoteRestrictionReasonYetUnsentBuilder {
  inner: PollVoteRestrictionReasonYetUnsent,
}

impl RTDPollVoteRestrictionReasonYetUnsentBuilder {
  pub fn build(&self) -> PollVoteRestrictionReasonYetUnsent {
    self.inner.clone()
  }
}

impl AsRef<PollVoteRestrictionReasonYetUnsent> for PollVoteRestrictionReasonYetUnsent {
  fn as_ref(&self) -> &PollVoteRestrictionReasonYetUnsent {
    self
  }
}

impl AsRef<PollVoteRestrictionReasonYetUnsent> for RTDPollVoteRestrictionReasonYetUnsentBuilder {
  fn as_ref(&self) -> &PollVoteRestrictionReasonYetUnsent {
    &self.inner
  }
}
