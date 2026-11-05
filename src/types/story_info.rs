use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains basic information about a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique story identifier among stories of the chat
  story_id: i64,
  /// Point in time (Unix timestamp) when the story was published
  date: i64,
  /// True, if the story is available only to close friends
  is_for_close_friends: bool,
  /// True, if the story is a live story
  is_live: bool,
}

impl RObject for StoryInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryInfoBuilder {
    let inner = StoryInfo {
      td_name: "storyInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryInfoBuilder { inner }
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn is_for_close_friends(&self) -> bool {
    self.is_for_close_friends
  }

  pub fn is_live(&self) -> bool {
    self.is_live
  }
}

#[doc(hidden)]
pub struct RTDStoryInfoBuilder {
  inner: StoryInfo,
}

impl RTDStoryInfoBuilder {
  pub fn build(&self) -> StoryInfo {
    self.inner.clone()
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn is_for_close_friends(&mut self, is_for_close_friends: bool) -> &mut Self {
    self.inner.is_for_close_friends = is_for_close_friends;
    self
  }

  pub fn is_live(&mut self, is_live: bool) -> &mut Self {
    self.inner.is_live = is_live;
    self
  }
}

impl AsRef<StoryInfo> for StoryInfo {
  fn as_ref(&self) -> &StoryInfo {
    self
  }
}

impl AsRef<StoryInfo> for RTDStoryInfoBuilder {
  fn as_ref(&self) -> &StoryInfo {
    &self.inner
  }
}
