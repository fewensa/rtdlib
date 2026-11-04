use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of stories found by a search
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundStories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of stories found
  total_count: i64,
  /// List of stories
  stories: Vec<Story>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for FoundStories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundStories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundStories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundStoriesBuilder {
    let inner = FoundStories {
      td_name: "foundStories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundStoriesBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn stories(&self) -> &Vec<Story> {
    &self.stories
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDFoundStoriesBuilder {
  inner: FoundStories,
}

impl RTDFoundStoriesBuilder {
  pub fn build(&self) -> FoundStories {
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

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<FoundStories> for FoundStories {
  fn as_ref(&self) -> &FoundStories {
    self
  }
}

impl AsRef<FoundStories> for RTDFoundStoriesBuilder {
  fn as_ref(&self) -> &FoundStories {
    &self.inner
  }
}
