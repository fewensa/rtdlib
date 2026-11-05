use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about status of a user in a giveaway
pub trait TDGiveawayParticipantStatus: Debug + RObject {}

/// Contains information about status of a user in a giveaway
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiveawayParticipantStatus {
  #[doc(hidden)]
  _Default(()),
  /// The user can't participate in the giveaway, because they are an administrator in one of the chats that created the giveaway
  Administrator(GiveawayParticipantStatusAdministrator),
  /// The user can't participate in the giveaway, because they have already been member of the chat
  AlreadyWasMember(GiveawayParticipantStatusAlreadyWasMember),
  /// The user can't participate in the giveaway, because their phone number is from a disallowed country
  DisallowedCountry(GiveawayParticipantStatusDisallowedCountry),
  /// The user is eligible for the giveaway
  Eligible(GiveawayParticipantStatusEligible),
  /// The user participates in the giveaway
  Participating(GiveawayParticipantStatusParticipating),
}

impl Default for GiveawayParticipantStatus {
  fn default() -> Self {
    GiveawayParticipantStatus::_Default(())
  }
}

impl<'de> Deserialize<'de> for GiveawayParticipantStatus {
  fn deserialize<D>(deserializer: D) -> Result<GiveawayParticipantStatus, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GiveawayParticipantStatus,
      (giveawayParticipantStatusAdministrator, Administrator);
      (giveawayParticipantStatusAlreadyWasMember, AlreadyWasMember);
      (giveawayParticipantStatusDisallowedCountry, DisallowedCountry);
      (giveawayParticipantStatusEligible, Eligible);
      (giveawayParticipantStatusParticipating, Participating);

    )(deserializer)
  }
}

impl RObject for GiveawayParticipantStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      GiveawayParticipantStatus::Administrator(t) => t.td_name(),
      GiveawayParticipantStatus::AlreadyWasMember(t) => t.td_name(),
      GiveawayParticipantStatus::DisallowedCountry(t) => t.td_name(),
      GiveawayParticipantStatus::Eligible(t) => t.td_name(),
      GiveawayParticipantStatus::Participating(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      GiveawayParticipantStatus::Administrator(t) => t.extra(),
      GiveawayParticipantStatus::AlreadyWasMember(t) => t.extra(),
      GiveawayParticipantStatus::DisallowedCountry(t) => t.extra(),
      GiveawayParticipantStatus::Eligible(t) => t.extra(),
      GiveawayParticipantStatus::Participating(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiveawayParticipantStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, GiveawayParticipantStatus::_Default(_))
  }

  pub fn is_administrator(&self) -> bool {
    matches!(self, GiveawayParticipantStatus::Administrator(_))
  }
  pub fn is_already_was_member(&self) -> bool {
    matches!(self, GiveawayParticipantStatus::AlreadyWasMember(_))
  }
  pub fn is_disallowed_country(&self) -> bool {
    matches!(self, GiveawayParticipantStatus::DisallowedCountry(_))
  }
  pub fn is_eligible(&self) -> bool {
    matches!(self, GiveawayParticipantStatus::Eligible(_))
  }
  pub fn is_participating(&self) -> bool {
    matches!(self, GiveawayParticipantStatus::Participating(_))
  }

  pub fn on_administrator<F: FnOnce(&GiveawayParticipantStatusAdministrator)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let GiveawayParticipantStatus::Administrator(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_already_was_member<F: FnOnce(&GiveawayParticipantStatusAlreadyWasMember)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let GiveawayParticipantStatus::AlreadyWasMember(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_disallowed_country<F: FnOnce(&GiveawayParticipantStatusDisallowedCountry)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let GiveawayParticipantStatus::DisallowedCountry(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_eligible<F: FnOnce(&GiveawayParticipantStatusEligible)>(&self, fnc: F) -> &Self {
    if let GiveawayParticipantStatus::Eligible(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_participating<F: FnOnce(&GiveawayParticipantStatusParticipating)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let GiveawayParticipantStatus::Participating(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_administrator(&self) -> Option<&GiveawayParticipantStatusAdministrator> {
    if let GiveawayParticipantStatus::Administrator(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_already_was_member(&self) -> Option<&GiveawayParticipantStatusAlreadyWasMember> {
    if let GiveawayParticipantStatus::AlreadyWasMember(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_disallowed_country(&self) -> Option<&GiveawayParticipantStatusDisallowedCountry> {
    if let GiveawayParticipantStatus::DisallowedCountry(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_eligible(&self) -> Option<&GiveawayParticipantStatusEligible> {
    if let GiveawayParticipantStatus::Eligible(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_participating(&self) -> Option<&GiveawayParticipantStatusParticipating> {
    if let GiveawayParticipantStatus::Participating(t) = self {
      return Some(t);
    }
    None
  }

  pub fn administrator<T: AsRef<GiveawayParticipantStatusAdministrator>>(t: T) -> Self {
    GiveawayParticipantStatus::Administrator(t.as_ref().clone())
  }

  pub fn already_was_member<T: AsRef<GiveawayParticipantStatusAlreadyWasMember>>(t: T) -> Self {
    GiveawayParticipantStatus::AlreadyWasMember(t.as_ref().clone())
  }

  pub fn disallowed_country<T: AsRef<GiveawayParticipantStatusDisallowedCountry>>(t: T) -> Self {
    GiveawayParticipantStatus::DisallowedCountry(t.as_ref().clone())
  }

  pub fn eligible<T: AsRef<GiveawayParticipantStatusEligible>>(t: T) -> Self {
    GiveawayParticipantStatus::Eligible(t.as_ref().clone())
  }

  pub fn participating<T: AsRef<GiveawayParticipantStatusParticipating>>(t: T) -> Self {
    GiveawayParticipantStatus::Participating(t.as_ref().clone())
  }
}

impl AsRef<GiveawayParticipantStatus> for GiveawayParticipantStatus {
  fn as_ref(&self) -> &GiveawayParticipantStatus {
    self
  }
}

/// The user can't participate in the giveaway, because they are an administrator in one of the chats that created the giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayParticipantStatusAdministrator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat administered by the user
  chat_id: i64,
}

impl RObject for GiveawayParticipantStatusAdministrator {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayParticipantStatusAdministrator"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayParticipantStatus for GiveawayParticipantStatusAdministrator {}

impl GiveawayParticipantStatusAdministrator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayParticipantStatusAdministratorBuilder {
    let inner = GiveawayParticipantStatusAdministrator {
      td_name: "giveawayParticipantStatusAdministrator".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiveawayParticipantStatusAdministratorBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDGiveawayParticipantStatusAdministratorBuilder {
  inner: GiveawayParticipantStatusAdministrator,
}

impl RTDGiveawayParticipantStatusAdministratorBuilder {
  pub fn build(&self) -> GiveawayParticipantStatusAdministrator {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<GiveawayParticipantStatusAdministrator> for GiveawayParticipantStatusAdministrator {
  fn as_ref(&self) -> &GiveawayParticipantStatusAdministrator {
    self
  }
}

impl AsRef<GiveawayParticipantStatusAdministrator>
  for RTDGiveawayParticipantStatusAdministratorBuilder
{
  fn as_ref(&self) -> &GiveawayParticipantStatusAdministrator {
    &self.inner
  }
}

/// The user can't participate in the giveaway, because they have already been member of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayParticipantStatusAlreadyWasMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the user joined the chat
  joined_chat_date: i64,
}

impl RObject for GiveawayParticipantStatusAlreadyWasMember {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayParticipantStatusAlreadyWasMember"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayParticipantStatus for GiveawayParticipantStatusAlreadyWasMember {}

impl GiveawayParticipantStatusAlreadyWasMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayParticipantStatusAlreadyWasMemberBuilder {
    let inner = GiveawayParticipantStatusAlreadyWasMember {
      td_name: "giveawayParticipantStatusAlreadyWasMember".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiveawayParticipantStatusAlreadyWasMemberBuilder { inner }
  }

  pub fn joined_chat_date(&self) -> i64 {
    self.joined_chat_date
  }
}

#[doc(hidden)]
pub struct RTDGiveawayParticipantStatusAlreadyWasMemberBuilder {
  inner: GiveawayParticipantStatusAlreadyWasMember,
}

impl RTDGiveawayParticipantStatusAlreadyWasMemberBuilder {
  pub fn build(&self) -> GiveawayParticipantStatusAlreadyWasMember {
    self.inner.clone()
  }

  pub fn joined_chat_date(&mut self, joined_chat_date: i64) -> &mut Self {
    self.inner.joined_chat_date = joined_chat_date;
    self
  }
}

impl AsRef<GiveawayParticipantStatusAlreadyWasMember>
  for GiveawayParticipantStatusAlreadyWasMember
{
  fn as_ref(&self) -> &GiveawayParticipantStatusAlreadyWasMember {
    self
  }
}

impl AsRef<GiveawayParticipantStatusAlreadyWasMember>
  for RTDGiveawayParticipantStatusAlreadyWasMemberBuilder
{
  fn as_ref(&self) -> &GiveawayParticipantStatusAlreadyWasMember {
    &self.inner
  }
}

/// The user can't participate in the giveaway, because their phone number is from a disallowed country
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayParticipantStatusDisallowedCountry {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A two-letter ISO 3166-1 alpha-2 country code of the user's country
  user_country_code: String,
}

impl RObject for GiveawayParticipantStatusDisallowedCountry {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayParticipantStatusDisallowedCountry"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayParticipantStatus for GiveawayParticipantStatusDisallowedCountry {}

impl GiveawayParticipantStatusDisallowedCountry {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayParticipantStatusDisallowedCountryBuilder {
    let inner = GiveawayParticipantStatusDisallowedCountry {
      td_name: "giveawayParticipantStatusDisallowedCountry".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiveawayParticipantStatusDisallowedCountryBuilder { inner }
  }

  pub fn user_country_code(&self) -> &String {
    &self.user_country_code
  }
}

#[doc(hidden)]
pub struct RTDGiveawayParticipantStatusDisallowedCountryBuilder {
  inner: GiveawayParticipantStatusDisallowedCountry,
}

impl RTDGiveawayParticipantStatusDisallowedCountryBuilder {
  pub fn build(&self) -> GiveawayParticipantStatusDisallowedCountry {
    self.inner.clone()
  }

  pub fn user_country_code<T: AsRef<str>>(&mut self, user_country_code: T) -> &mut Self {
    self.inner.user_country_code = user_country_code.as_ref().to_string();
    self
  }
}

impl AsRef<GiveawayParticipantStatusDisallowedCountry>
  for GiveawayParticipantStatusDisallowedCountry
{
  fn as_ref(&self) -> &GiveawayParticipantStatusDisallowedCountry {
    self
  }
}

impl AsRef<GiveawayParticipantStatusDisallowedCountry>
  for RTDGiveawayParticipantStatusDisallowedCountryBuilder
{
  fn as_ref(&self) -> &GiveawayParticipantStatusDisallowedCountry {
    &self.inner
  }
}

/// The user is eligible for the giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayParticipantStatusEligible {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GiveawayParticipantStatusEligible {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayParticipantStatusEligible"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayParticipantStatus for GiveawayParticipantStatusEligible {}

impl GiveawayParticipantStatusEligible {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayParticipantStatusEligibleBuilder {
    let inner = GiveawayParticipantStatusEligible {
      td_name: "giveawayParticipantStatusEligible".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGiveawayParticipantStatusEligibleBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGiveawayParticipantStatusEligibleBuilder {
  inner: GiveawayParticipantStatusEligible,
}

impl RTDGiveawayParticipantStatusEligibleBuilder {
  pub fn build(&self) -> GiveawayParticipantStatusEligible {
    self.inner.clone()
  }
}

impl AsRef<GiveawayParticipantStatusEligible> for GiveawayParticipantStatusEligible {
  fn as_ref(&self) -> &GiveawayParticipantStatusEligible {
    self
  }
}

impl AsRef<GiveawayParticipantStatusEligible> for RTDGiveawayParticipantStatusEligibleBuilder {
  fn as_ref(&self) -> &GiveawayParticipantStatusEligible {
    &self.inner
  }
}

/// The user participates in the giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayParticipantStatusParticipating {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GiveawayParticipantStatusParticipating {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayParticipantStatusParticipating"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayParticipantStatus for GiveawayParticipantStatusParticipating {}

impl GiveawayParticipantStatusParticipating {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayParticipantStatusParticipatingBuilder {
    let inner = GiveawayParticipantStatusParticipating {
      td_name: "giveawayParticipantStatusParticipating".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGiveawayParticipantStatusParticipatingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGiveawayParticipantStatusParticipatingBuilder {
  inner: GiveawayParticipantStatusParticipating,
}

impl RTDGiveawayParticipantStatusParticipatingBuilder {
  pub fn build(&self) -> GiveawayParticipantStatusParticipating {
    self.inner.clone()
  }
}

impl AsRef<GiveawayParticipantStatusParticipating> for GiveawayParticipantStatusParticipating {
  fn as_ref(&self) -> &GiveawayParticipantStatusParticipating {
    self
  }
}

impl AsRef<GiveawayParticipantStatusParticipating>
  for RTDGiveawayParticipantStatusParticipatingBuilder
{
  fn as_ref(&self) -> &GiveawayParticipantStatusParticipating {
    &self.inner
  }
}
