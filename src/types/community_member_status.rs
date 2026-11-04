use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Provides information about the status of a member in a community
pub trait TDCommunityMemberStatus: Debug + RObject {}

/// Provides information about the status of a member in a community
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CommunityMemberStatus {
  #[doc(hidden)]
  _Default(()),
  /// The user is a member of the community and has some additional privileges
  Administrator(CommunityMemberStatusAdministrator),
  /// The user or the chat was banned in the community; implies ban in all chats in the community
  Banned(CommunityMemberStatusBanned),
  /// The user is the owner of the community and has all the administrator privileges
  Creator(CommunityMemberStatusCreator),
  /// The user or the chat is not a community member
  Left(CommunityMemberStatusLeft),
  /// The user is a member of the community, without any additional privileges or restrictions
  Member(CommunityMemberStatusMember),
}

impl Default for CommunityMemberStatus {
  fn default() -> Self {
    CommunityMemberStatus::_Default(())
  }
}

impl<'de> Deserialize<'de> for CommunityMemberStatus {
  fn deserialize<D>(deserializer: D) -> Result<CommunityMemberStatus, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CommunityMemberStatus,
      (communityMemberStatusAdministrator, Administrator);
      (communityMemberStatusBanned, Banned);
      (communityMemberStatusCreator, Creator);
      (communityMemberStatusLeft, Left);
      (communityMemberStatusMember, Member);

    )(deserializer)
  }
}

impl RObject for CommunityMemberStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      CommunityMemberStatus::Administrator(t) => t.td_name(),
      CommunityMemberStatus::Banned(t) => t.td_name(),
      CommunityMemberStatus::Creator(t) => t.td_name(),
      CommunityMemberStatus::Left(t) => t.td_name(),
      CommunityMemberStatus::Member(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      CommunityMemberStatus::Administrator(t) => t.extra(),
      CommunityMemberStatus::Banned(t) => t.extra(),
      CommunityMemberStatus::Creator(t) => t.extra(),
      CommunityMemberStatus::Left(t) => t.extra(),
      CommunityMemberStatus::Member(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CommunityMemberStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, CommunityMemberStatus::_Default(_))
  }

  pub fn is_administrator(&self) -> bool {
    matches!(self, CommunityMemberStatus::Administrator(_))
  }
  pub fn is_banned(&self) -> bool {
    matches!(self, CommunityMemberStatus::Banned(_))
  }
  pub fn is_creator(&self) -> bool {
    matches!(self, CommunityMemberStatus::Creator(_))
  }
  pub fn is_left(&self) -> bool {
    matches!(self, CommunityMemberStatus::Left(_))
  }
  pub fn is_member(&self) -> bool {
    matches!(self, CommunityMemberStatus::Member(_))
  }

  pub fn on_administrator<F: FnOnce(&CommunityMemberStatusAdministrator)>(&self, fnc: F) -> &Self {
    if let CommunityMemberStatus::Administrator(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_banned<F: FnOnce(&CommunityMemberStatusBanned)>(&self, fnc: F) -> &Self {
    if let CommunityMemberStatus::Banned(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_creator<F: FnOnce(&CommunityMemberStatusCreator)>(&self, fnc: F) -> &Self {
    if let CommunityMemberStatus::Creator(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_left<F: FnOnce(&CommunityMemberStatusLeft)>(&self, fnc: F) -> &Self {
    if let CommunityMemberStatus::Left(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_member<F: FnOnce(&CommunityMemberStatusMember)>(&self, fnc: F) -> &Self {
    if let CommunityMemberStatus::Member(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_administrator(&self) -> Option<&CommunityMemberStatusAdministrator> {
    if let CommunityMemberStatus::Administrator(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_banned(&self) -> Option<&CommunityMemberStatusBanned> {
    if let CommunityMemberStatus::Banned(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_creator(&self) -> Option<&CommunityMemberStatusCreator> {
    if let CommunityMemberStatus::Creator(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_left(&self) -> Option<&CommunityMemberStatusLeft> {
    if let CommunityMemberStatus::Left(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_member(&self) -> Option<&CommunityMemberStatusMember> {
    if let CommunityMemberStatus::Member(t) = self {
      return Some(t);
    }
    None
  }

  pub fn administrator<T: AsRef<CommunityMemberStatusAdministrator>>(t: T) -> Self {
    CommunityMemberStatus::Administrator(t.as_ref().clone())
  }

  pub fn banned<T: AsRef<CommunityMemberStatusBanned>>(t: T) -> Self {
    CommunityMemberStatus::Banned(t.as_ref().clone())
  }

  pub fn creator<T: AsRef<CommunityMemberStatusCreator>>(t: T) -> Self {
    CommunityMemberStatus::Creator(t.as_ref().clone())
  }

  pub fn left<T: AsRef<CommunityMemberStatusLeft>>(t: T) -> Self {
    CommunityMemberStatus::Left(t.as_ref().clone())
  }

  pub fn member<T: AsRef<CommunityMemberStatusMember>>(t: T) -> Self {
    CommunityMemberStatus::Member(t.as_ref().clone())
  }
}

impl AsRef<CommunityMemberStatus> for CommunityMemberStatus {
  fn as_ref(&self) -> &CommunityMemberStatus {
    self
  }
}

/// The user is a member of the community and has some additional privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityMemberStatusAdministrator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the current user can edit the administrator privileges for the called user
  can_be_edited: bool,
  /// Rights of the administrator
  rights: CommunityAdministratorRights,
}

impl RObject for CommunityMemberStatusAdministrator {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityMemberStatusAdministrator"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCommunityMemberStatus for CommunityMemberStatusAdministrator {}

impl CommunityMemberStatusAdministrator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityMemberStatusAdministratorBuilder {
    let inner = CommunityMemberStatusAdministrator {
      td_name: "communityMemberStatusAdministrator".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCommunityMemberStatusAdministratorBuilder { inner }
  }

  pub fn can_be_edited(&self) -> bool {
    self.can_be_edited
  }

  pub fn rights(&self) -> &CommunityAdministratorRights {
    &self.rights
  }
}

#[doc(hidden)]
pub struct RTDCommunityMemberStatusAdministratorBuilder {
  inner: CommunityMemberStatusAdministrator,
}

impl RTDCommunityMemberStatusAdministratorBuilder {
  pub fn build(&self) -> CommunityMemberStatusAdministrator {
    self.inner.clone()
  }

  pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
    self.inner.can_be_edited = can_be_edited;
    self
  }

  pub fn rights<T: AsRef<CommunityAdministratorRights>>(&mut self, rights: T) -> &mut Self {
    self.inner.rights = rights.as_ref().clone();
    self
  }
}

impl AsRef<CommunityMemberStatusAdministrator> for CommunityMemberStatusAdministrator {
  fn as_ref(&self) -> &CommunityMemberStatusAdministrator {
    self
  }
}

impl AsRef<CommunityMemberStatusAdministrator> for RTDCommunityMemberStatusAdministratorBuilder {
  fn as_ref(&self) -> &CommunityMemberStatusAdministrator {
    &self.inner
  }
}

/// The user or the chat was banned in the community; implies ban in all chats in the community
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityMemberStatusBanned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CommunityMemberStatusBanned {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityMemberStatusBanned"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCommunityMemberStatus for CommunityMemberStatusBanned {}

impl CommunityMemberStatusBanned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityMemberStatusBannedBuilder {
    let inner = CommunityMemberStatusBanned {
      td_name: "communityMemberStatusBanned".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCommunityMemberStatusBannedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCommunityMemberStatusBannedBuilder {
  inner: CommunityMemberStatusBanned,
}

impl RTDCommunityMemberStatusBannedBuilder {
  pub fn build(&self) -> CommunityMemberStatusBanned {
    self.inner.clone()
  }
}

impl AsRef<CommunityMemberStatusBanned> for CommunityMemberStatusBanned {
  fn as_ref(&self) -> &CommunityMemberStatusBanned {
    self
  }
}

impl AsRef<CommunityMemberStatusBanned> for RTDCommunityMemberStatusBannedBuilder {
  fn as_ref(&self) -> &CommunityMemberStatusBanned {
    &self.inner
  }
}

/// The user is the owner of the community and has all the administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityMemberStatusCreator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CommunityMemberStatusCreator {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityMemberStatusCreator"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCommunityMemberStatus for CommunityMemberStatusCreator {}

impl CommunityMemberStatusCreator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityMemberStatusCreatorBuilder {
    let inner = CommunityMemberStatusCreator {
      td_name: "communityMemberStatusCreator".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCommunityMemberStatusCreatorBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCommunityMemberStatusCreatorBuilder {
  inner: CommunityMemberStatusCreator,
}

impl RTDCommunityMemberStatusCreatorBuilder {
  pub fn build(&self) -> CommunityMemberStatusCreator {
    self.inner.clone()
  }
}

impl AsRef<CommunityMemberStatusCreator> for CommunityMemberStatusCreator {
  fn as_ref(&self) -> &CommunityMemberStatusCreator {
    self
  }
}

impl AsRef<CommunityMemberStatusCreator> for RTDCommunityMemberStatusCreatorBuilder {
  fn as_ref(&self) -> &CommunityMemberStatusCreator {
    &self.inner
  }
}

/// The user or the chat is not a community member
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityMemberStatusLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CommunityMemberStatusLeft {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityMemberStatusLeft"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCommunityMemberStatus for CommunityMemberStatusLeft {}

impl CommunityMemberStatusLeft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityMemberStatusLeftBuilder {
    let inner = CommunityMemberStatusLeft {
      td_name: "communityMemberStatusLeft".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCommunityMemberStatusLeftBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCommunityMemberStatusLeftBuilder {
  inner: CommunityMemberStatusLeft,
}

impl RTDCommunityMemberStatusLeftBuilder {
  pub fn build(&self) -> CommunityMemberStatusLeft {
    self.inner.clone()
  }
}

impl AsRef<CommunityMemberStatusLeft> for CommunityMemberStatusLeft {
  fn as_ref(&self) -> &CommunityMemberStatusLeft {
    self
  }
}

impl AsRef<CommunityMemberStatusLeft> for RTDCommunityMemberStatusLeftBuilder {
  fn as_ref(&self) -> &CommunityMemberStatusLeft {
    &self.inner
  }
}

/// The user is a member of the community, without any additional privileges or restrictions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityMemberStatusMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for CommunityMemberStatusMember {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityMemberStatusMember"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDCommunityMemberStatus for CommunityMemberStatusMember {}

impl CommunityMemberStatusMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityMemberStatusMemberBuilder {
    let inner = CommunityMemberStatusMember {
      td_name: "communityMemberStatusMember".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDCommunityMemberStatusMemberBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDCommunityMemberStatusMemberBuilder {
  inner: CommunityMemberStatusMember,
}

impl RTDCommunityMemberStatusMemberBuilder {
  pub fn build(&self) -> CommunityMemberStatusMember {
    self.inner.clone()
  }
}

impl AsRef<CommunityMemberStatusMember> for CommunityMemberStatusMember {
  fn as_ref(&self) -> &CommunityMemberStatusMember {
    self
  }
}

impl AsRef<CommunityMemberStatusMember> for RTDCommunityMemberStatusMemberBuilder {
  fn as_ref(&self) -> &CommunityMemberStatusMember {
    &self.inner
  }
}
