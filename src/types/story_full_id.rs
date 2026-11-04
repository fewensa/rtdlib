use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains identifier of a story along with identifier of the chat that posted it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryFullId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that posted the story
  poster_chat_id: i64,
  /// Unique story identifier among stories of the chat
  story_id: i64,
}

impl RObject for StoryFullId {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyFullId"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryFullId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryFullIdBuilder {
    let inner = StoryFullId {
      td_name: "storyFullId".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryFullIdBuilder { inner }
  }

  pub fn poster_chat_id(&self) -> i64 {
    self.poster_chat_id
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDStoryFullIdBuilder {
  inner: StoryFullId,
}

impl RTDStoryFullIdBuilder {
  pub fn build(&self) -> StoryFullId {
    self.inner.clone()
  }

  pub fn poster_chat_id(&mut self, poster_chat_id: i64) -> &mut Self {
    self.inner.poster_chat_id = poster_chat_id;
    self
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<StoryFullId> for StoryFullId {
  fn as_ref(&self) -> &StoryFullId {
    self
  }
}

impl AsRef<StoryFullId> for RTDStoryFullIdBuilder {
  fn as_ref(&self) -> &StoryFullId {
    &self.inner
  }
}
