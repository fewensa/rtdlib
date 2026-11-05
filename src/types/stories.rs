use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of stories found
  total_count: i64,
  /// The list of stories
  stories: Vec<Story>,
  /// Identifiers of the pinned stories; returned only in getChatPostedToChatPageStories with from_story_id == 0
  pinned_story_ids: Vec<i64>,
}

impl RObject for Stories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Stories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoriesBuilder {
    let inner = Stories {
      td_name: "stories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoriesBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn stories(&self) -> &Vec<Story> {
    &self.stories
  }

  pub fn pinned_story_ids(&self) -> &Vec<i64> {
    &self.pinned_story_ids
  }
}

#[doc(hidden)]
pub struct RTDStoriesBuilder {
  inner: Stories,
}

impl RTDStoriesBuilder {
  pub fn build(&self) -> Stories {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn stories(&mut self, stories: Vec<Story>) -> &mut Self {
    self.inner.stories = stories;
    self
  }

  pub fn pinned_story_ids(&mut self, pinned_story_ids: Vec<i64>) -> &mut Self {
    self.inner.pinned_story_ids = pinned_story_ids;
    self
  }
}

impl AsRef<Stories> for Stories {
  fn as_ref(&self) -> &Stories {
    self
  }
}

impl AsRef<Stories> for RTDStoriesBuilder {
  fn as_ref(&self) -> &Stories {
    &self.inner
  }
}
