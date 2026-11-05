use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes privacy settings of a story
pub trait TDStoryPrivacySettings: Debug + RObject {}

/// Describes privacy settings of a story
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StoryPrivacySettings {
  #[doc(hidden)]
  _Default(()),
  /// The story can be viewed by all close friends
  CloseFriends(StoryPrivacySettingsCloseFriends),
  /// The story can be viewed by all contacts except chosen users
  Contacts(StoryPrivacySettingsContacts),
  /// The story can be viewed by everyone
  Everyone(StoryPrivacySettingsEveryone),
  /// The story can be viewed by certain specified users
  SelectedUsers(StoryPrivacySettingsSelectedUsers),
}

impl Default for StoryPrivacySettings {
  fn default() -> Self {
    StoryPrivacySettings::_Default(())
  }
}

impl<'de> Deserialize<'de> for StoryPrivacySettings {
  fn deserialize<D>(deserializer: D) -> Result<StoryPrivacySettings, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StoryPrivacySettings,
      (storyPrivacySettingsCloseFriends, CloseFriends);
      (storyPrivacySettingsContacts, Contacts);
      (storyPrivacySettingsEveryone, Everyone);
      (storyPrivacySettingsSelectedUsers, SelectedUsers);

    )(deserializer)
  }
}

impl RObject for StoryPrivacySettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StoryPrivacySettings::CloseFriends(t) => t.td_name(),
      StoryPrivacySettings::Contacts(t) => t.td_name(),
      StoryPrivacySettings::Everyone(t) => t.td_name(),
      StoryPrivacySettings::SelectedUsers(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StoryPrivacySettings::CloseFriends(t) => t.extra(),
      StoryPrivacySettings::Contacts(t) => t.extra(),
      StoryPrivacySettings::Everyone(t) => t.extra(),
      StoryPrivacySettings::SelectedUsers(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryPrivacySettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StoryPrivacySettings::_Default(_))
  }

  pub fn is_close_friends(&self) -> bool {
    matches!(self, StoryPrivacySettings::CloseFriends(_))
  }
  pub fn is_contacts(&self) -> bool {
    matches!(self, StoryPrivacySettings::Contacts(_))
  }
  pub fn is_everyone(&self) -> bool {
    matches!(self, StoryPrivacySettings::Everyone(_))
  }
  pub fn is_selected_users(&self) -> bool {
    matches!(self, StoryPrivacySettings::SelectedUsers(_))
  }

  pub fn on_close_friends<F: FnOnce(&StoryPrivacySettingsCloseFriends)>(&self, fnc: F) -> &Self {
    if let StoryPrivacySettings::CloseFriends(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_contacts<F: FnOnce(&StoryPrivacySettingsContacts)>(&self, fnc: F) -> &Self {
    if let StoryPrivacySettings::Contacts(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_everyone<F: FnOnce(&StoryPrivacySettingsEveryone)>(&self, fnc: F) -> &Self {
    if let StoryPrivacySettings::Everyone(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_selected_users<F: FnOnce(&StoryPrivacySettingsSelectedUsers)>(&self, fnc: F) -> &Self {
    if let StoryPrivacySettings::SelectedUsers(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_close_friends(&self) -> Option<&StoryPrivacySettingsCloseFriends> {
    if let StoryPrivacySettings::CloseFriends(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_contacts(&self) -> Option<&StoryPrivacySettingsContacts> {
    if let StoryPrivacySettings::Contacts(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_everyone(&self) -> Option<&StoryPrivacySettingsEveryone> {
    if let StoryPrivacySettings::Everyone(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_selected_users(&self) -> Option<&StoryPrivacySettingsSelectedUsers> {
    if let StoryPrivacySettings::SelectedUsers(t) = self {
      return Some(t);
    }
    None
  }

  pub fn close_friends<T: AsRef<StoryPrivacySettingsCloseFriends>>(t: T) -> Self {
    StoryPrivacySettings::CloseFriends(t.as_ref().clone())
  }

  pub fn contacts<T: AsRef<StoryPrivacySettingsContacts>>(t: T) -> Self {
    StoryPrivacySettings::Contacts(t.as_ref().clone())
  }

  pub fn everyone<T: AsRef<StoryPrivacySettingsEveryone>>(t: T) -> Self {
    StoryPrivacySettings::Everyone(t.as_ref().clone())
  }

  pub fn selected_users<T: AsRef<StoryPrivacySettingsSelectedUsers>>(t: T) -> Self {
    StoryPrivacySettings::SelectedUsers(t.as_ref().clone())
  }
}

impl AsRef<StoryPrivacySettings> for StoryPrivacySettings {
  fn as_ref(&self) -> &StoryPrivacySettings {
    self
  }
}

/// The story can be viewed by all close friends
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryPrivacySettingsCloseFriends {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StoryPrivacySettingsCloseFriends {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyPrivacySettingsCloseFriends"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryPrivacySettings for StoryPrivacySettingsCloseFriends {}

impl StoryPrivacySettingsCloseFriends {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryPrivacySettingsCloseFriendsBuilder {
    let inner = StoryPrivacySettingsCloseFriends {
      td_name: "storyPrivacySettingsCloseFriends".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStoryPrivacySettingsCloseFriendsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStoryPrivacySettingsCloseFriendsBuilder {
  inner: StoryPrivacySettingsCloseFriends,
}

impl RTDStoryPrivacySettingsCloseFriendsBuilder {
  pub fn build(&self) -> StoryPrivacySettingsCloseFriends {
    self.inner.clone()
  }
}

impl AsRef<StoryPrivacySettingsCloseFriends> for StoryPrivacySettingsCloseFriends {
  fn as_ref(&self) -> &StoryPrivacySettingsCloseFriends {
    self
  }
}

impl AsRef<StoryPrivacySettingsCloseFriends> for RTDStoryPrivacySettingsCloseFriendsBuilder {
  fn as_ref(&self) -> &StoryPrivacySettingsCloseFriends {
    &self.inner
  }
}

/// The story can be viewed by all contacts except chosen users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryPrivacySettingsContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifiers of the contacts that can't see the story; always unknown and empty for non-owned stories
  except_user_ids: Vec<i64>,
}

impl RObject for StoryPrivacySettingsContacts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyPrivacySettingsContacts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryPrivacySettings for StoryPrivacySettingsContacts {}

impl StoryPrivacySettingsContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryPrivacySettingsContactsBuilder {
    let inner = StoryPrivacySettingsContacts {
      td_name: "storyPrivacySettingsContacts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryPrivacySettingsContactsBuilder { inner }
  }

  pub fn except_user_ids(&self) -> &Vec<i64> {
    &self.except_user_ids
  }
}

#[doc(hidden)]
pub struct RTDStoryPrivacySettingsContactsBuilder {
  inner: StoryPrivacySettingsContacts,
}

impl RTDStoryPrivacySettingsContactsBuilder {
  pub fn build(&self) -> StoryPrivacySettingsContacts {
    self.inner.clone()
  }

  pub fn except_user_ids(&mut self, except_user_ids: Vec<i64>) -> &mut Self {
    self.inner.except_user_ids = except_user_ids;
    self
  }
}

impl AsRef<StoryPrivacySettingsContacts> for StoryPrivacySettingsContacts {
  fn as_ref(&self) -> &StoryPrivacySettingsContacts {
    self
  }
}

impl AsRef<StoryPrivacySettingsContacts> for RTDStoryPrivacySettingsContactsBuilder {
  fn as_ref(&self) -> &StoryPrivacySettingsContacts {
    &self.inner
  }
}

/// The story can be viewed by everyone
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryPrivacySettingsEveryone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifiers of the users that can't see the story; always unknown and empty for non-owned stories
  except_user_ids: Vec<i64>,
}

impl RObject for StoryPrivacySettingsEveryone {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyPrivacySettingsEveryone"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryPrivacySettings for StoryPrivacySettingsEveryone {}

impl StoryPrivacySettingsEveryone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryPrivacySettingsEveryoneBuilder {
    let inner = StoryPrivacySettingsEveryone {
      td_name: "storyPrivacySettingsEveryone".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryPrivacySettingsEveryoneBuilder { inner }
  }

  pub fn except_user_ids(&self) -> &Vec<i64> {
    &self.except_user_ids
  }
}

#[doc(hidden)]
pub struct RTDStoryPrivacySettingsEveryoneBuilder {
  inner: StoryPrivacySettingsEveryone,
}

impl RTDStoryPrivacySettingsEveryoneBuilder {
  pub fn build(&self) -> StoryPrivacySettingsEveryone {
    self.inner.clone()
  }

  pub fn except_user_ids(&mut self, except_user_ids: Vec<i64>) -> &mut Self {
    self.inner.except_user_ids = except_user_ids;
    self
  }
}

impl AsRef<StoryPrivacySettingsEveryone> for StoryPrivacySettingsEveryone {
  fn as_ref(&self) -> &StoryPrivacySettingsEveryone {
    self
  }
}

impl AsRef<StoryPrivacySettingsEveryone> for RTDStoryPrivacySettingsEveryoneBuilder {
  fn as_ref(&self) -> &StoryPrivacySettingsEveryone {
    &self.inner
  }
}

/// The story can be viewed by certain specified users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryPrivacySettingsSelectedUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifiers of the users; always unknown and empty for non-owned stories
  user_ids: Vec<i64>,
}

impl RObject for StoryPrivacySettingsSelectedUsers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyPrivacySettingsSelectedUsers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryPrivacySettings for StoryPrivacySettingsSelectedUsers {}

impl StoryPrivacySettingsSelectedUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryPrivacySettingsSelectedUsersBuilder {
    let inner = StoryPrivacySettingsSelectedUsers {
      td_name: "storyPrivacySettingsSelectedUsers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryPrivacySettingsSelectedUsersBuilder { inner }
  }

  pub fn user_ids(&self) -> &Vec<i64> {
    &self.user_ids
  }
}

#[doc(hidden)]
pub struct RTDStoryPrivacySettingsSelectedUsersBuilder {
  inner: StoryPrivacySettingsSelectedUsers,
}

impl RTDStoryPrivacySettingsSelectedUsersBuilder {
  pub fn build(&self) -> StoryPrivacySettingsSelectedUsers {
    self.inner.clone()
  }

  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }
}

impl AsRef<StoryPrivacySettingsSelectedUsers> for StoryPrivacySettingsSelectedUsers {
  fn as_ref(&self) -> &StoryPrivacySettingsSelectedUsers {
    self
  }
}

impl AsRef<StoryPrivacySettingsSelectedUsers> for RTDStoryPrivacySettingsSelectedUsersBuilder {
  fn as_ref(&self) -> &StoryPrivacySettingsSelectedUsers {
    &self.inner
  }
}
