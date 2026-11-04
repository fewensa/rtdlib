use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes state of active stories posted by a chat
pub trait TDActiveStoryState: Debug + RObject {}

/// Describes state of active stories posted by a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ActiveStoryState {
  #[doc(hidden)]
  _Default(()),
  /// The chat has an active live story
  Live(ActiveStoryStateLive),
  /// The chat has active stories, all of which were read
  Read(ActiveStoryStateRead),
  /// The chat has some unread active stories
  Unread(ActiveStoryStateUnread),
}

impl Default for ActiveStoryState {
  fn default() -> Self {
    ActiveStoryState::_Default(())
  }
}

impl<'de> Deserialize<'de> for ActiveStoryState {
  fn deserialize<D>(deserializer: D) -> Result<ActiveStoryState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ActiveStoryState,
      (activeStoryStateLive, Live);
      (activeStoryStateRead, Read);
      (activeStoryStateUnread, Unread);

    )(deserializer)
  }
}

impl RObject for ActiveStoryState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ActiveStoryState::Live(t) => t.td_name(),
      ActiveStoryState::Read(t) => t.td_name(),
      ActiveStoryState::Unread(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ActiveStoryState::Live(t) => t.extra(),
      ActiveStoryState::Read(t) => t.extra(),
      ActiveStoryState::Unread(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ActiveStoryState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ActiveStoryState::_Default(_))
  }

  pub fn is_live(&self) -> bool {
    matches!(self, ActiveStoryState::Live(_))
  }
  pub fn is_read(&self) -> bool {
    matches!(self, ActiveStoryState::Read(_))
  }
  pub fn is_unread(&self) -> bool {
    matches!(self, ActiveStoryState::Unread(_))
  }

  pub fn on_live<F: FnOnce(&ActiveStoryStateLive)>(&self, fnc: F) -> &Self {
    if let ActiveStoryState::Live(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_read<F: FnOnce(&ActiveStoryStateRead)>(&self, fnc: F) -> &Self {
    if let ActiveStoryState::Read(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unread<F: FnOnce(&ActiveStoryStateUnread)>(&self, fnc: F) -> &Self {
    if let ActiveStoryState::Unread(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_live(&self) -> Option<&ActiveStoryStateLive> {
    if let ActiveStoryState::Live(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_read(&self) -> Option<&ActiveStoryStateRead> {
    if let ActiveStoryState::Read(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unread(&self) -> Option<&ActiveStoryStateUnread> {
    if let ActiveStoryState::Unread(t) = self {
      return Some(t);
    }
    None
  }

  pub fn live<T: AsRef<ActiveStoryStateLive>>(t: T) -> Self {
    ActiveStoryState::Live(t.as_ref().clone())
  }

  pub fn read<T: AsRef<ActiveStoryStateRead>>(t: T) -> Self {
    ActiveStoryState::Read(t.as_ref().clone())
  }

  pub fn unread<T: AsRef<ActiveStoryStateUnread>>(t: T) -> Self {
    ActiveStoryState::Unread(t.as_ref().clone())
  }
}

impl AsRef<ActiveStoryState> for ActiveStoryState {
  fn as_ref(&self) -> &ActiveStoryState {
    self
  }
}

/// The chat has an active live story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveStoryStateLive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the active live story
  story_id: i64,
}

impl RObject for ActiveStoryStateLive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "activeStoryStateLive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDActiveStoryState for ActiveStoryStateLive {}

impl ActiveStoryStateLive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDActiveStoryStateLiveBuilder {
    let inner = ActiveStoryStateLive {
      td_name: "activeStoryStateLive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDActiveStoryStateLiveBuilder { inner }
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDActiveStoryStateLiveBuilder {
  inner: ActiveStoryStateLive,
}

impl RTDActiveStoryStateLiveBuilder {
  pub fn build(&self) -> ActiveStoryStateLive {
    self.inner.clone()
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<ActiveStoryStateLive> for ActiveStoryStateLive {
  fn as_ref(&self) -> &ActiveStoryStateLive {
    self
  }
}

impl AsRef<ActiveStoryStateLive> for RTDActiveStoryStateLiveBuilder {
  fn as_ref(&self) -> &ActiveStoryStateLive {
    &self.inner
  }
}

/// The chat has active stories, all of which were read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveStoryStateRead {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ActiveStoryStateRead {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "activeStoryStateRead"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDActiveStoryState for ActiveStoryStateRead {}

impl ActiveStoryStateRead {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDActiveStoryStateReadBuilder {
    let inner = ActiveStoryStateRead {
      td_name: "activeStoryStateRead".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDActiveStoryStateReadBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDActiveStoryStateReadBuilder {
  inner: ActiveStoryStateRead,
}

impl RTDActiveStoryStateReadBuilder {
  pub fn build(&self) -> ActiveStoryStateRead {
    self.inner.clone()
  }
}

impl AsRef<ActiveStoryStateRead> for ActiveStoryStateRead {
  fn as_ref(&self) -> &ActiveStoryStateRead {
    self
  }
}

impl AsRef<ActiveStoryStateRead> for RTDActiveStoryStateReadBuilder {
  fn as_ref(&self) -> &ActiveStoryStateRead {
    &self.inner
  }
}

/// The chat has some unread active stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveStoryStateUnread {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ActiveStoryStateUnread {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "activeStoryStateUnread"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDActiveStoryState for ActiveStoryStateUnread {}

impl ActiveStoryStateUnread {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDActiveStoryStateUnreadBuilder {
    let inner = ActiveStoryStateUnread {
      td_name: "activeStoryStateUnread".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDActiveStoryStateUnreadBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDActiveStoryStateUnreadBuilder {
  inner: ActiveStoryStateUnread,
}

impl RTDActiveStoryStateUnreadBuilder {
  pub fn build(&self) -> ActiveStoryStateUnread {
    self.inner.clone()
  }
}

impl AsRef<ActiveStoryStateUnread> for ActiveStoryStateUnread {
  fn as_ref(&self) -> &ActiveStoryStateUnread {
    self
  }
}

impl AsRef<ActiveStoryStateUnread> for RTDActiveStoryStateUnreadBuilder {
  fn as_ref(&self) -> &ActiveStoryStateUnread {
    &self.inner
  }
}
