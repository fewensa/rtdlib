use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents result of starting a live story
pub trait TDStartLiveStoryResult: Debug + RObject {}

/// Represents result of starting a live story
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StartLiveStoryResult {
  #[doc(hidden)]
  _Default(()),
  /// Starts a new live story on behalf of a chat; requires can_post_stories administrator right for channel chats
  StartLiveStory(StartLiveStory),
  /// The live story failed to post with an error to be handled
  Fail(StartLiveStoryResultFail),
  /// The live story was successfully posted
  Ok(StartLiveStoryResultOk),
}

impl Default for StartLiveStoryResult {
  fn default() -> Self {
    StartLiveStoryResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for StartLiveStoryResult {
  fn deserialize<D>(deserializer: D) -> Result<StartLiveStoryResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StartLiveStoryResult,
      (startLiveStory, StartLiveStory);
      (startLiveStoryResultFail, Fail);
      (startLiveStoryResultOk, Ok);

    )(deserializer)
  }
}

impl RObject for StartLiveStoryResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StartLiveStoryResult::StartLiveStory(t) => t.td_name(),
      StartLiveStoryResult::Fail(t) => t.td_name(),
      StartLiveStoryResult::Ok(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StartLiveStoryResult::StartLiveStory(t) => t.extra(),
      StartLiveStoryResult::Fail(t) => t.extra(),
      StartLiveStoryResult::Ok(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StartLiveStoryResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StartLiveStoryResult::_Default(_))
  }

  pub fn is_start_live_story(&self) -> bool {
    matches!(self, StartLiveStoryResult::StartLiveStory(_))
  }
  pub fn is_fail(&self) -> bool {
    matches!(self, StartLiveStoryResult::Fail(_))
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, StartLiveStoryResult::Ok(_))
  }

  pub fn on_start_live_story<F: FnOnce(&StartLiveStory)>(&self, fnc: F) -> &Self {
    if let StartLiveStoryResult::StartLiveStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fail<F: FnOnce(&StartLiveStoryResultFail)>(&self, fnc: F) -> &Self {
    if let StartLiveStoryResult::Fail(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ok<F: FnOnce(&StartLiveStoryResultOk)>(&self, fnc: F) -> &Self {
    if let StartLiveStoryResult::Ok(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_start_live_story(&self) -> Option<&StartLiveStory> {
    if let StartLiveStoryResult::StartLiveStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fail(&self) -> Option<&StartLiveStoryResultFail> {
    if let StartLiveStoryResult::Fail(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ok(&self) -> Option<&StartLiveStoryResultOk> {
    if let StartLiveStoryResult::Ok(t) = self {
      return Some(t);
    }
    None
  }

  pub fn start_live_story<T: AsRef<StartLiveStory>>(t: T) -> Self {
    StartLiveStoryResult::StartLiveStory(t.as_ref().clone())
  }

  pub fn fail<T: AsRef<StartLiveStoryResultFail>>(t: T) -> Self {
    StartLiveStoryResult::Fail(t.as_ref().clone())
  }

  pub fn ok<T: AsRef<StartLiveStoryResultOk>>(t: T) -> Self {
    StartLiveStoryResult::Ok(t.as_ref().clone())
  }
}

impl AsRef<StartLiveStoryResult> for StartLiveStoryResult {
  fn as_ref(&self) -> &StartLiveStoryResult {
    self
  }
}

/// The live story failed to post with an error to be handled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartLiveStoryResultFail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the error; other error types may be returned as regular errors
  error_type: CanPostStoryResult,
}

impl RObject for StartLiveStoryResultFail {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "startLiveStoryResultFail"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStartLiveStoryResult for StartLiveStoryResultFail {}

impl StartLiveStoryResultFail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStartLiveStoryResultFailBuilder {
    let inner = StartLiveStoryResultFail {
      td_name: "startLiveStoryResultFail".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStartLiveStoryResultFailBuilder { inner }
  }

  pub fn error_type(&self) -> &CanPostStoryResult {
    &self.error_type
  }
}

#[doc(hidden)]
pub struct RTDStartLiveStoryResultFailBuilder {
  inner: StartLiveStoryResultFail,
}

impl RTDStartLiveStoryResultFailBuilder {
  pub fn build(&self) -> StartLiveStoryResultFail {
    self.inner.clone()
  }

  pub fn error_type<T: AsRef<CanPostStoryResult>>(&mut self, error_type: T) -> &mut Self {
    self.inner.error_type = error_type.as_ref().clone();
    self
  }
}

impl AsRef<StartLiveStoryResultFail> for StartLiveStoryResultFail {
  fn as_ref(&self) -> &StartLiveStoryResultFail {
    self
  }
}

impl AsRef<StartLiveStoryResultFail> for RTDStartLiveStoryResultFailBuilder {
  fn as_ref(&self) -> &StartLiveStoryResultFail {
    &self.inner
  }
}

/// The live story was successfully posted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartLiveStoryResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The live story
  story: Story,
}

impl RObject for StartLiveStoryResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "startLiveStoryResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStartLiveStoryResult for StartLiveStoryResultOk {}

impl StartLiveStoryResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStartLiveStoryResultOkBuilder {
    let inner = StartLiveStoryResultOk {
      td_name: "startLiveStoryResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStartLiveStoryResultOkBuilder { inner }
  }

  pub fn story(&self) -> &Story {
    &self.story
  }
}

#[doc(hidden)]
pub struct RTDStartLiveStoryResultOkBuilder {
  inner: StartLiveStoryResultOk,
}

impl RTDStartLiveStoryResultOkBuilder {
  pub fn build(&self) -> StartLiveStoryResultOk {
    self.inner.clone()
  }

  pub fn story<T: AsRef<Story>>(&mut self, story: T) -> &mut Self {
    self.inner.story = story.as_ref().clone();
    self
  }
}

impl AsRef<StartLiveStoryResultOk> for StartLiveStoryResultOk {
  fn as_ref(&self) -> &StartLiveStoryResultOk {
    self
  }
}

impl AsRef<StartLiveStoryResultOk> for RTDStartLiveStoryResultOkBuilder {
  fn as_ref(&self) -> &StartLiveStoryResultOk {
    &self.inner
  }
}
