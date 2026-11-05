use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a list of stories
pub trait TDStoryList: Debug + RObject {}

/// Describes a list of stories
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StoryList {
  #[doc(hidden)]
  _Default(()),
  /// The list of stories, shown in the Archive chat list
  Archive(StoryListArchive),
  /// The list of stories, shown in the main chat list and folder chat lists
  Main(StoryListMain),
}

impl Default for StoryList {
  fn default() -> Self {
    StoryList::_Default(())
  }
}

impl<'de> Deserialize<'de> for StoryList {
  fn deserialize<D>(deserializer: D) -> Result<StoryList, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StoryList,
      (storyListArchive, Archive);
      (storyListMain, Main);

    )(deserializer)
  }
}

impl RObject for StoryList {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StoryList::Archive(t) => t.td_name(),
      StoryList::Main(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StoryList::Archive(t) => t.extra(),
      StoryList::Main(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StoryList::_Default(_))
  }

  pub fn is_archive(&self) -> bool {
    matches!(self, StoryList::Archive(_))
  }
  pub fn is_main(&self) -> bool {
    matches!(self, StoryList::Main(_))
  }

  pub fn on_archive<F: FnOnce(&StoryListArchive)>(&self, fnc: F) -> &Self {
    if let StoryList::Archive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_main<F: FnOnce(&StoryListMain)>(&self, fnc: F) -> &Self {
    if let StoryList::Main(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_archive(&self) -> Option<&StoryListArchive> {
    if let StoryList::Archive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_main(&self) -> Option<&StoryListMain> {
    if let StoryList::Main(t) = self {
      return Some(t);
    }
    None
  }

  pub fn archive<T: AsRef<StoryListArchive>>(t: T) -> Self {
    StoryList::Archive(t.as_ref().clone())
  }

  pub fn main<T: AsRef<StoryListMain>>(t: T) -> Self {
    StoryList::Main(t.as_ref().clone())
  }
}

impl AsRef<StoryList> for StoryList {
  fn as_ref(&self) -> &StoryList {
    self
  }
}

/// The list of stories, shown in the Archive chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryListArchive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StoryListArchive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyListArchive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryList for StoryListArchive {}

impl StoryListArchive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryListArchiveBuilder {
    let inner = StoryListArchive {
      td_name: "storyListArchive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStoryListArchiveBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStoryListArchiveBuilder {
  inner: StoryListArchive,
}

impl RTDStoryListArchiveBuilder {
  pub fn build(&self) -> StoryListArchive {
    self.inner.clone()
  }
}

impl AsRef<StoryListArchive> for StoryListArchive {
  fn as_ref(&self) -> &StoryListArchive {
    self
  }
}

impl AsRef<StoryListArchive> for RTDStoryListArchiveBuilder {
  fn as_ref(&self) -> &StoryListArchive {
    &self.inner
  }
}

/// The list of stories, shown in the main chat list and folder chat lists
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryListMain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StoryListMain {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyListMain"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryList for StoryListMain {}

impl StoryListMain {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryListMainBuilder {
    let inner = StoryListMain {
      td_name: "storyListMain".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStoryListMainBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStoryListMainBuilder {
  inner: StoryListMain,
}

impl RTDStoryListMainBuilder {
  pub fn build(&self) -> StoryListMain {
    self.inner.clone()
  }
}

impl AsRef<StoryListMain> for StoryListMain {
  fn as_ref(&self) -> &StoryListMain {
    self
  }
}

impl AsRef<StoryListMain> for RTDStoryListMainBuilder {
  fn as_ref(&self) -> &StoryListMain {
    &self.inner
  }
}
