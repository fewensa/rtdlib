use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents interaction with a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInteraction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user or chat that made the interaction
  actor_id: MessageSender,
  /// Approximate point in time (Unix timestamp) when the interaction happened
  interaction_date: i64,
  /// Block list to which the actor is added; may be null if none or for chat stories
  block_list: Option<BlockList>,
  /// Type of the interaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: StoryInteractionType,
}

impl RObject for StoryInteraction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyInteraction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryInteraction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryInteractionBuilder {
    let inner = StoryInteraction {
      td_name: "storyInteraction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryInteractionBuilder { inner }
  }

  pub fn actor_id(&self) -> &MessageSender {
    &self.actor_id
  }

  pub fn interaction_date(&self) -> i64 {
    self.interaction_date
  }

  pub fn block_list(&self) -> &Option<BlockList> {
    &self.block_list
  }

  pub fn type_(&self) -> &StoryInteractionType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDStoryInteractionBuilder {
  inner: StoryInteraction,
}

impl RTDStoryInteractionBuilder {
  pub fn build(&self) -> StoryInteraction {
    self.inner.clone()
  }

  pub fn actor_id<T: AsRef<MessageSender>>(&mut self, actor_id: T) -> &mut Self {
    self.inner.actor_id = actor_id.as_ref().clone();
    self
  }

  pub fn interaction_date(&mut self, interaction_date: i64) -> &mut Self {
    self.inner.interaction_date = interaction_date;
    self
  }

  pub fn block_list<T: AsRef<BlockList>>(&mut self, block_list: T) -> &mut Self {
    self.inner.block_list = Some(block_list.as_ref().clone());
    self
  }

  pub fn type_<T: AsRef<StoryInteractionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<StoryInteraction> for StoryInteraction {
  fn as_ref(&self) -> &StoryInteraction {
    self
  }
}

impl AsRef<StoryInteraction> for RTDStoryInteractionBuilder {
  fn as_ref(&self) -> &StoryInteraction {
    &self.inner
  }
}
