use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about original story that was reposted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryRepostInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Origin of the story that was reposted
  origin: StoryOrigin,
  /// True, if story content was modified during reposting; otherwise, story wasn't modified
  is_content_modified: bool,
}

impl RObject for StoryRepostInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyRepostInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryRepostInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryRepostInfoBuilder {
    let inner = StoryRepostInfo {
      td_name: "storyRepostInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryRepostInfoBuilder { inner }
  }

  pub fn origin(&self) -> &StoryOrigin {
    &self.origin
  }

  pub fn is_content_modified(&self) -> bool {
    self.is_content_modified
  }
}

#[doc(hidden)]
pub struct RTDStoryRepostInfoBuilder {
  inner: StoryRepostInfo,
}

impl RTDStoryRepostInfoBuilder {
  pub fn build(&self) -> StoryRepostInfo {
    self.inner.clone()
  }

  pub fn origin<T: AsRef<StoryOrigin>>(&mut self, origin: T) -> &mut Self {
    self.inner.origin = origin.as_ref().clone();
    self
  }

  pub fn is_content_modified(&mut self, is_content_modified: bool) -> &mut Self {
    self.inner.is_content_modified = is_content_modified;
    self
  }
}

impl AsRef<StoryRepostInfo> for StoryRepostInfo {
  fn as_ref(&self) -> &StoryRepostInfo {
    self
  }
}

impl AsRef<StoryRepostInfo> for RTDStoryRepostInfoBuilder {
  fn as_ref(&self) -> &StoryRepostInfo {
    &self.inner
  }
}
