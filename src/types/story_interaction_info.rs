use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about interactions with a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInteractionInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of times the story was viewed
  view_count: i64,
  /// Number of times the story was forwarded; 0 if none or unknown
  forward_count: i64,
  /// Number of reactions added to the story; 0 if none or unknown
  reaction_count: i64,
  /// Identifiers of at most 3 recent viewers of the story
  recent_viewer_user_ids: Vec<i64>,
}

impl RObject for StoryInteractionInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyInteractionInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryInteractionInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryInteractionInfoBuilder {
    let inner = StoryInteractionInfo {
      td_name: "storyInteractionInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryInteractionInfoBuilder { inner }
  }

  pub fn view_count(&self) -> i64 {
    self.view_count
  }

  pub fn forward_count(&self) -> i64 {
    self.forward_count
  }

  pub fn reaction_count(&self) -> i64 {
    self.reaction_count
  }

  pub fn recent_viewer_user_ids(&self) -> &Vec<i64> {
    &self.recent_viewer_user_ids
  }
}

#[doc(hidden)]
pub struct RTDStoryInteractionInfoBuilder {
  inner: StoryInteractionInfo,
}

impl RTDStoryInteractionInfoBuilder {
  pub fn build(&self) -> StoryInteractionInfo {
    self.inner.clone()
  }

  pub fn view_count(&mut self, view_count: i64) -> &mut Self {
    self.inner.view_count = view_count;
    self
  }

  pub fn forward_count(&mut self, forward_count: i64) -> &mut Self {
    self.inner.forward_count = forward_count;
    self
  }

  pub fn reaction_count(&mut self, reaction_count: i64) -> &mut Self {
    self.inner.reaction_count = reaction_count;
    self
  }

  pub fn recent_viewer_user_ids(&mut self, recent_viewer_user_ids: Vec<i64>) -> &mut Self {
    self.inner.recent_viewer_user_ids = recent_viewer_user_ids;
    self
  }
}

impl AsRef<StoryInteractionInfo> for StoryInteractionInfo {
  fn as_ref(&self) -> &StoryInteractionInfo {
    self
  }
}

impl AsRef<StoryInteractionInfo> for RTDStoryInteractionInfoBuilder {
  fn as_ref(&self) -> &StoryInteractionInfo {
    &self.inner
  }
}
