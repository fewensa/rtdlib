use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of interactions with a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInteractions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of interactions found
  total_count: i64,
  /// Approximate total number of found forwards and reposts; always 0 for chat stories
  total_forward_count: i64,
  /// Approximate total number of found reactions; always 0 for chat stories
  total_reaction_count: i64,
  /// List of story interactions
  interactions: Vec<StoryInteraction>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for StoryInteractions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyInteractions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryInteractions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryInteractionsBuilder {
    let inner = StoryInteractions {
      td_name: "storyInteractions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryInteractionsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn total_forward_count(&self) -> i64 {
    self.total_forward_count
  }

  pub fn total_reaction_count(&self) -> i64 {
    self.total_reaction_count
  }

  pub fn interactions(&self) -> &Vec<StoryInteraction> {
    &self.interactions
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDStoryInteractionsBuilder {
  inner: StoryInteractions,
}

impl RTDStoryInteractionsBuilder {
  pub fn build(&self) -> StoryInteractions {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn total_forward_count(&mut self, total_forward_count: i64) -> &mut Self {
    self.inner.total_forward_count = total_forward_count;
    self
  }

  pub fn total_reaction_count(&mut self, total_reaction_count: i64) -> &mut Self {
    self.inner.total_reaction_count = total_reaction_count;
    self
  }

  pub fn interactions(&mut self, interactions: Vec<StoryInteraction>) -> &mut Self {
    self.inner.interactions = interactions;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<StoryInteractions> for StoryInteractions {
  fn as_ref(&self) -> &StoryInteractions {
    self
  }
}

impl AsRef<StoryInteractions> for RTDStoryInteractionsBuilder {
  fn as_ref(&self) -> &StoryInteractions {
    &self.inner
  }
}
