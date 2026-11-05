use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes sources of reactions for which notifications will be shown
pub trait TDReactionNotificationSource: Debug + RObject {}

/// Describes sources of reactions for which notifications will be shown
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReactionNotificationSource {
  #[doc(hidden)]
  _Default(()),
  /// Notifications for reactions are shown for all reactions
  All(ReactionNotificationSourceAll),
  /// Notifications for reactions are shown only for reactions from contacts
  Contacts(ReactionNotificationSourceContacts),
  /// Notifications for reactions are disabled
  None(ReactionNotificationSourceNone),
}

impl Default for ReactionNotificationSource {
  fn default() -> Self {
    ReactionNotificationSource::_Default(())
  }
}

impl<'de> Deserialize<'de> for ReactionNotificationSource {
  fn deserialize<D>(deserializer: D) -> Result<ReactionNotificationSource, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ReactionNotificationSource,
      (reactionNotificationSourceAll, All);
      (reactionNotificationSourceContacts, Contacts);
      (reactionNotificationSourceNone, None);

    )(deserializer)
  }
}

impl RObject for ReactionNotificationSource {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ReactionNotificationSource::All(t) => t.td_name(),
      ReactionNotificationSource::Contacts(t) => t.td_name(),
      ReactionNotificationSource::None(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ReactionNotificationSource::All(t) => t.extra(),
      ReactionNotificationSource::Contacts(t) => t.extra(),
      ReactionNotificationSource::None(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReactionNotificationSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ReactionNotificationSource::_Default(_))
  }

  pub fn is_all(&self) -> bool {
    matches!(self, ReactionNotificationSource::All(_))
  }
  pub fn is_contacts(&self) -> bool {
    matches!(self, ReactionNotificationSource::Contacts(_))
  }
  pub fn is_none(&self) -> bool {
    matches!(self, ReactionNotificationSource::None(_))
  }

  pub fn on_all<F: FnOnce(&ReactionNotificationSourceAll)>(&self, fnc: F) -> &Self {
    if let ReactionNotificationSource::All(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_contacts<F: FnOnce(&ReactionNotificationSourceContacts)>(&self, fnc: F) -> &Self {
    if let ReactionNotificationSource::Contacts(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_none<F: FnOnce(&ReactionNotificationSourceNone)>(&self, fnc: F) -> &Self {
    if let ReactionNotificationSource::None(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_all(&self) -> Option<&ReactionNotificationSourceAll> {
    if let ReactionNotificationSource::All(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_contacts(&self) -> Option<&ReactionNotificationSourceContacts> {
    if let ReactionNotificationSource::Contacts(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_none(&self) -> Option<&ReactionNotificationSourceNone> {
    if let ReactionNotificationSource::None(t) = self {
      return Some(t);
    }
    None
  }

  pub fn all<T: AsRef<ReactionNotificationSourceAll>>(t: T) -> Self {
    ReactionNotificationSource::All(t.as_ref().clone())
  }

  pub fn contacts<T: AsRef<ReactionNotificationSourceContacts>>(t: T) -> Self {
    ReactionNotificationSource::Contacts(t.as_ref().clone())
  }

  pub fn none<T: AsRef<ReactionNotificationSourceNone>>(t: T) -> Self {
    ReactionNotificationSource::None(t.as_ref().clone())
  }
}

impl AsRef<ReactionNotificationSource> for ReactionNotificationSource {
  fn as_ref(&self) -> &ReactionNotificationSource {
    self
  }
}

/// Notifications for reactions are shown for all reactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionNotificationSourceAll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReactionNotificationSourceAll {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionNotificationSourceAll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionNotificationSource for ReactionNotificationSourceAll {}

impl ReactionNotificationSourceAll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionNotificationSourceAllBuilder {
    let inner = ReactionNotificationSourceAll {
      td_name: "reactionNotificationSourceAll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReactionNotificationSourceAllBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReactionNotificationSourceAllBuilder {
  inner: ReactionNotificationSourceAll,
}

impl RTDReactionNotificationSourceAllBuilder {
  pub fn build(&self) -> ReactionNotificationSourceAll {
    self.inner.clone()
  }
}

impl AsRef<ReactionNotificationSourceAll> for ReactionNotificationSourceAll {
  fn as_ref(&self) -> &ReactionNotificationSourceAll {
    self
  }
}

impl AsRef<ReactionNotificationSourceAll> for RTDReactionNotificationSourceAllBuilder {
  fn as_ref(&self) -> &ReactionNotificationSourceAll {
    &self.inner
  }
}

/// Notifications for reactions are shown only for reactions from contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionNotificationSourceContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReactionNotificationSourceContacts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionNotificationSourceContacts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionNotificationSource for ReactionNotificationSourceContacts {}

impl ReactionNotificationSourceContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionNotificationSourceContactsBuilder {
    let inner = ReactionNotificationSourceContacts {
      td_name: "reactionNotificationSourceContacts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReactionNotificationSourceContactsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReactionNotificationSourceContactsBuilder {
  inner: ReactionNotificationSourceContacts,
}

impl RTDReactionNotificationSourceContactsBuilder {
  pub fn build(&self) -> ReactionNotificationSourceContacts {
    self.inner.clone()
  }
}

impl AsRef<ReactionNotificationSourceContacts> for ReactionNotificationSourceContacts {
  fn as_ref(&self) -> &ReactionNotificationSourceContacts {
    self
  }
}

impl AsRef<ReactionNotificationSourceContacts> for RTDReactionNotificationSourceContactsBuilder {
  fn as_ref(&self) -> &ReactionNotificationSourceContacts {
    &self.inner
  }
}

/// Notifications for reactions are disabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReactionNotificationSourceNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReactionNotificationSourceNone {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reactionNotificationSourceNone"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReactionNotificationSource for ReactionNotificationSourceNone {}

impl ReactionNotificationSourceNone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReactionNotificationSourceNoneBuilder {
    let inner = ReactionNotificationSourceNone {
      td_name: "reactionNotificationSourceNone".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReactionNotificationSourceNoneBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReactionNotificationSourceNoneBuilder {
  inner: ReactionNotificationSourceNone,
}

impl RTDReactionNotificationSourceNoneBuilder {
  pub fn build(&self) -> ReactionNotificationSourceNone {
    self.inner.clone()
  }
}

impl AsRef<ReactionNotificationSourceNone> for ReactionNotificationSourceNone {
  fn as_ref(&self) -> &ReactionNotificationSourceNone {
    self
  }
}

impl AsRef<ReactionNotificationSourceNone> for RTDReactionNotificationSourceNoneBuilder {
  fn as_ref(&self) -> &ReactionNotificationSourceNone {
    &self.inner
  }
}
