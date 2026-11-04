use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A detailed statistics about poll votes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoteStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A graph containing distribution of votes in the poll
  vote_graph: StatisticalGraph,
}

impl RObject for PollVoteStatistics {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoteStatistics"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PollVoteStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVoteStatisticsBuilder {
    let inner = PollVoteStatistics {
      td_name: "pollVoteStatistics".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollVoteStatisticsBuilder { inner }
  }

  pub fn vote_graph(&self) -> &StatisticalGraph {
    &self.vote_graph
  }
}

#[doc(hidden)]
pub struct RTDPollVoteStatisticsBuilder {
  inner: PollVoteStatistics,
}

impl RTDPollVoteStatisticsBuilder {
  pub fn build(&self) -> PollVoteStatistics {
    self.inner.clone()
  }

  pub fn vote_graph<T: AsRef<StatisticalGraph>>(&mut self, vote_graph: T) -> &mut Self {
    self.inner.vote_graph = vote_graph.as_ref().clone();
    self
  }
}

impl AsRef<PollVoteStatistics> for PollVoteStatistics {
  fn as_ref(&self) -> &PollVoteStatistics {
    self
  }
}

impl AsRef<PollVoteStatistics> for RTDPollVoteStatisticsBuilder {
  fn as_ref(&self) -> &PollVoteStatistics {
    &self.inner
  }
}
