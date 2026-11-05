use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of poll voters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of poll voters found
  total_count: i64,
  /// List of poll voters
  voters: Vec<PollVoter>,
}

impl RObject for PollVoters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PollVoters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVotersBuilder {
    let inner = PollVoters {
      td_name: "pollVoters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollVotersBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn voters(&self) -> &Vec<PollVoter> {
    &self.voters
  }
}

#[doc(hidden)]
pub struct RTDPollVotersBuilder {
  inner: PollVoters,
}

impl RTDPollVotersBuilder {
  pub fn build(&self) -> PollVoters {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn voters(&mut self, voters: Vec<PollVoter>) -> &mut Self {
    self.inner.voters = voters;
    self
  }
}

impl AsRef<PollVoters> for PollVoters {
  fn as_ref(&self) -> &PollVoters {
    self
  }
}

impl AsRef<PollVoters> for RTDPollVotersBuilder {
  fn as_ref(&self) -> &PollVoters {
    &self.inner
  }
}
