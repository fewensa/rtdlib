use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about the origin of a story that was reposted
pub trait TDStoryOrigin: Debug + RObject {}

/// Contains information about the origin of a story that was reposted
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StoryOrigin {
  #[doc(hidden)]
  _Default(()),
  /// The original story was posted by an unknown user
  HiddenUser(StoryOriginHiddenUser),
  /// The original story was a public story that was posted by a known chat
  PublicStory(StoryOriginPublicStory),
}

impl Default for StoryOrigin {
  fn default() -> Self {
    StoryOrigin::_Default(())
  }
}

impl<'de> Deserialize<'de> for StoryOrigin {
  fn deserialize<D>(deserializer: D) -> Result<StoryOrigin, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StoryOrigin,
      (storyOriginHiddenUser, HiddenUser);
      (storyOriginPublicStory, PublicStory);

    )(deserializer)
  }
}

impl RObject for StoryOrigin {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StoryOrigin::HiddenUser(t) => t.td_name(),
      StoryOrigin::PublicStory(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StoryOrigin::HiddenUser(t) => t.extra(),
      StoryOrigin::PublicStory(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryOrigin {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StoryOrigin::_Default(_))
  }

  pub fn is_hidden_user(&self) -> bool {
    matches!(self, StoryOrigin::HiddenUser(_))
  }
  pub fn is_public_story(&self) -> bool {
    matches!(self, StoryOrigin::PublicStory(_))
  }

  pub fn on_hidden_user<F: FnOnce(&StoryOriginHiddenUser)>(&self, fnc: F) -> &Self {
    if let StoryOrigin::HiddenUser(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_public_story<F: FnOnce(&StoryOriginPublicStory)>(&self, fnc: F) -> &Self {
    if let StoryOrigin::PublicStory(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_hidden_user(&self) -> Option<&StoryOriginHiddenUser> {
    if let StoryOrigin::HiddenUser(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_public_story(&self) -> Option<&StoryOriginPublicStory> {
    if let StoryOrigin::PublicStory(t) = self {
      return Some(t);
    }
    None
  }

  pub fn hidden_user<T: AsRef<StoryOriginHiddenUser>>(t: T) -> Self {
    StoryOrigin::HiddenUser(t.as_ref().clone())
  }

  pub fn public_story<T: AsRef<StoryOriginPublicStory>>(t: T) -> Self {
    StoryOrigin::PublicStory(t.as_ref().clone())
  }
}

impl AsRef<StoryOrigin> for StoryOrigin {
  fn as_ref(&self) -> &StoryOrigin {
    self
  }
}

/// The original story was posted by an unknown user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryOriginHiddenUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the user or the chat that posted the story
  poster_name: String,
}

impl RObject for StoryOriginHiddenUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyOriginHiddenUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryOrigin for StoryOriginHiddenUser {}

impl StoryOriginHiddenUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryOriginHiddenUserBuilder {
    let inner = StoryOriginHiddenUser {
      td_name: "storyOriginHiddenUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryOriginHiddenUserBuilder { inner }
  }

  pub fn poster_name(&self) -> &String {
    &self.poster_name
  }
}

#[doc(hidden)]
pub struct RTDStoryOriginHiddenUserBuilder {
  inner: StoryOriginHiddenUser,
}

impl RTDStoryOriginHiddenUserBuilder {
  pub fn build(&self) -> StoryOriginHiddenUser {
    self.inner.clone()
  }

  pub fn poster_name<T: AsRef<str>>(&mut self, poster_name: T) -> &mut Self {
    self.inner.poster_name = poster_name.as_ref().to_string();
    self
  }
}

impl AsRef<StoryOriginHiddenUser> for StoryOriginHiddenUser {
  fn as_ref(&self) -> &StoryOriginHiddenUser {
    self
  }
}

impl AsRef<StoryOriginHiddenUser> for RTDStoryOriginHiddenUserBuilder {
  fn as_ref(&self) -> &StoryOriginHiddenUser {
    &self.inner
  }
}

/// The original story was a public story that was posted by a known chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryOriginPublicStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that posted original story
  chat_id: i64,
  /// Story identifier of the original story
  story_id: i64,
}

impl RObject for StoryOriginPublicStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyOriginPublicStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryOrigin for StoryOriginPublicStory {}

impl StoryOriginPublicStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryOriginPublicStoryBuilder {
    let inner = StoryOriginPublicStory {
      td_name: "storyOriginPublicStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryOriginPublicStoryBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDStoryOriginPublicStoryBuilder {
  inner: StoryOriginPublicStory,
}

impl RTDStoryOriginPublicStoryBuilder {
  pub fn build(&self) -> StoryOriginPublicStory {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<StoryOriginPublicStory> for StoryOriginPublicStory {
  fn as_ref(&self) -> &StoryOriginPublicStory {
    self
  }
}

impl AsRef<StoryOriginPublicStory> for RTDStoryOriginPublicStoryBuilder {
  fn as_ref(&self) -> &StoryOriginPublicStory {
    &self.inner
  }
}
