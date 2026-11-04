use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A detailed statistics about a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A graph containing number of story views and shares
  story_interaction_graph: StatisticalGraph,
  /// A graph containing number of story reactions
  story_reaction_graph: StatisticalGraph,
}

impl RObject for StoryStatistics {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyStatistics"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryStatisticsBuilder {
    let inner = StoryStatistics {
      td_name: "storyStatistics".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryStatisticsBuilder { inner }
  }

  pub fn story_interaction_graph(&self) -> &StatisticalGraph {
    &self.story_interaction_graph
  }

  pub fn story_reaction_graph(&self) -> &StatisticalGraph {
    &self.story_reaction_graph
  }
}

#[doc(hidden)]
pub struct RTDStoryStatisticsBuilder {
  inner: StoryStatistics,
}

impl RTDStoryStatisticsBuilder {
  pub fn build(&self) -> StoryStatistics {
    self.inner.clone()
  }

  pub fn story_interaction_graph<T: AsRef<StatisticalGraph>>(
    &mut self,
    story_interaction_graph: T,
  ) -> &mut Self {
    self.inner.story_interaction_graph = story_interaction_graph.as_ref().clone();
    self
  }

  pub fn story_reaction_graph<T: AsRef<StatisticalGraph>>(
    &mut self,
    story_reaction_graph: T,
  ) -> &mut Self {
    self.inner.story_reaction_graph = story_reaction_graph.as_ref().clone();
    self
  }
}

impl AsRef<StoryStatistics> for StoryStatistics {
  fn as_ref(&self) -> &StoryStatistics {
    self
  }
}

impl AsRef<StoryStatistics> for RTDStoryStatisticsBuilder {
  fn as_ref(&self) -> &StoryStatistics {
    &self.inner
  }
}
