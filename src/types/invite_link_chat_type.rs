use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of chat to which points an invite link
pub trait TDInviteLinkChatType: Debug + RObject {}

/// Describes the type of chat to which points an invite link
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InviteLinkChatType {
  #[doc(hidden)]
  _Default(()),
  /// The link is an invite link for a basic group
  BasicGroup(InviteLinkChatTypeBasicGroup),
  /// The link is an invite link for a channel
  Channel(InviteLinkChatTypeChannel),
  /// The link is an invite link for a supergroup
  Supergroup(InviteLinkChatTypeSupergroup),
}

impl Default for InviteLinkChatType {
  fn default() -> Self {
    InviteLinkChatType::_Default(())
  }
}

impl<'de> Deserialize<'de> for InviteLinkChatType {
  fn deserialize<D>(deserializer: D) -> Result<InviteLinkChatType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InviteLinkChatType,
      (inviteLinkChatTypeBasicGroup, BasicGroup);
      (inviteLinkChatTypeChannel, Channel);
      (inviteLinkChatTypeSupergroup, Supergroup);

    )(deserializer)
  }
}

impl RObject for InviteLinkChatType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InviteLinkChatType::BasicGroup(t) => t.td_name(),
      InviteLinkChatType::Channel(t) => t.td_name(),
      InviteLinkChatType::Supergroup(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InviteLinkChatType::BasicGroup(t) => t.extra(),
      InviteLinkChatType::Channel(t) => t.extra(),
      InviteLinkChatType::Supergroup(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InviteLinkChatType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InviteLinkChatType::_Default(_))
  }

  pub fn is_basic_group(&self) -> bool {
    matches!(self, InviteLinkChatType::BasicGroup(_))
  }
  pub fn is_channel(&self) -> bool {
    matches!(self, InviteLinkChatType::Channel(_))
  }
  pub fn is_supergroup(&self) -> bool {
    matches!(self, InviteLinkChatType::Supergroup(_))
  }

  pub fn on_basic_group<F: FnOnce(&InviteLinkChatTypeBasicGroup)>(&self, fnc: F) -> &Self {
    if let InviteLinkChatType::BasicGroup(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel<F: FnOnce(&InviteLinkChatTypeChannel)>(&self, fnc: F) -> &Self {
    if let InviteLinkChatType::Channel(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_supergroup<F: FnOnce(&InviteLinkChatTypeSupergroup)>(&self, fnc: F) -> &Self {
    if let InviteLinkChatType::Supergroup(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_basic_group(&self) -> Option<&InviteLinkChatTypeBasicGroup> {
    if let InviteLinkChatType::BasicGroup(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel(&self) -> Option<&InviteLinkChatTypeChannel> {
    if let InviteLinkChatType::Channel(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_supergroup(&self) -> Option<&InviteLinkChatTypeSupergroup> {
    if let InviteLinkChatType::Supergroup(t) = self {
      return Some(t);
    }
    None
  }

  pub fn basic_group<T: AsRef<InviteLinkChatTypeBasicGroup>>(t: T) -> Self {
    InviteLinkChatType::BasicGroup(t.as_ref().clone())
  }

  pub fn channel<T: AsRef<InviteLinkChatTypeChannel>>(t: T) -> Self {
    InviteLinkChatType::Channel(t.as_ref().clone())
  }

  pub fn supergroup<T: AsRef<InviteLinkChatTypeSupergroup>>(t: T) -> Self {
    InviteLinkChatType::Supergroup(t.as_ref().clone())
  }
}

impl AsRef<InviteLinkChatType> for InviteLinkChatType {
  fn as_ref(&self) -> &InviteLinkChatType {
    self
  }
}

/// The link is an invite link for a basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteLinkChatTypeBasicGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InviteLinkChatTypeBasicGroup {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inviteLinkChatTypeBasicGroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInviteLinkChatType for InviteLinkChatTypeBasicGroup {}

impl InviteLinkChatTypeBasicGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInviteLinkChatTypeBasicGroupBuilder {
    let inner = InviteLinkChatTypeBasicGroup {
      td_name: "inviteLinkChatTypeBasicGroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInviteLinkChatTypeBasicGroupBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInviteLinkChatTypeBasicGroupBuilder {
  inner: InviteLinkChatTypeBasicGroup,
}

impl RTDInviteLinkChatTypeBasicGroupBuilder {
  pub fn build(&self) -> InviteLinkChatTypeBasicGroup {
    self.inner.clone()
  }
}

impl AsRef<InviteLinkChatTypeBasicGroup> for InviteLinkChatTypeBasicGroup {
  fn as_ref(&self) -> &InviteLinkChatTypeBasicGroup {
    self
  }
}

impl AsRef<InviteLinkChatTypeBasicGroup> for RTDInviteLinkChatTypeBasicGroupBuilder {
  fn as_ref(&self) -> &InviteLinkChatTypeBasicGroup {
    &self.inner
  }
}

/// The link is an invite link for a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteLinkChatTypeChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InviteLinkChatTypeChannel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inviteLinkChatTypeChannel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInviteLinkChatType for InviteLinkChatTypeChannel {}

impl InviteLinkChatTypeChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInviteLinkChatTypeChannelBuilder {
    let inner = InviteLinkChatTypeChannel {
      td_name: "inviteLinkChatTypeChannel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInviteLinkChatTypeChannelBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInviteLinkChatTypeChannelBuilder {
  inner: InviteLinkChatTypeChannel,
}

impl RTDInviteLinkChatTypeChannelBuilder {
  pub fn build(&self) -> InviteLinkChatTypeChannel {
    self.inner.clone()
  }
}

impl AsRef<InviteLinkChatTypeChannel> for InviteLinkChatTypeChannel {
  fn as_ref(&self) -> &InviteLinkChatTypeChannel {
    self
  }
}

impl AsRef<InviteLinkChatTypeChannel> for RTDInviteLinkChatTypeChannelBuilder {
  fn as_ref(&self) -> &InviteLinkChatTypeChannel {
    &self.inner
  }
}

/// The link is an invite link for a supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteLinkChatTypeSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InviteLinkChatTypeSupergroup {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inviteLinkChatTypeSupergroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInviteLinkChatType for InviteLinkChatTypeSupergroup {}

impl InviteLinkChatTypeSupergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInviteLinkChatTypeSupergroupBuilder {
    let inner = InviteLinkChatTypeSupergroup {
      td_name: "inviteLinkChatTypeSupergroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInviteLinkChatTypeSupergroupBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInviteLinkChatTypeSupergroupBuilder {
  inner: InviteLinkChatTypeSupergroup,
}

impl RTDInviteLinkChatTypeSupergroupBuilder {
  pub fn build(&self) -> InviteLinkChatTypeSupergroup {
    self.inner.clone()
  }
}

impl AsRef<InviteLinkChatTypeSupergroup> for InviteLinkChatTypeSupergroup {
  fn as_ref(&self) -> &InviteLinkChatTypeSupergroup {
    self
  }
}

impl AsRef<InviteLinkChatTypeSupergroup> for RTDInviteLinkChatTypeSupergroupBuilder {
  fn as_ref(&self) -> &InviteLinkChatTypeSupergroup {
    &self.inner
  }
}
