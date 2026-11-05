use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a poll voter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollVoter {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The voter identifier
  voter_id: MessageSender,
  /// Point in time (Unix timestamp) when the vote was added
  date: i64,
}

impl RObject for PollVoter {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollVoter"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PollVoter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollVoterBuilder {
    let inner = PollVoter {
      td_name: "pollVoter".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollVoterBuilder { inner }
  }

  pub fn voter_id(&self) -> &MessageSender {
    &self.voter_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }
}

#[doc(hidden)]
pub struct RTDPollVoterBuilder {
  inner: PollVoter,
}

impl RTDPollVoterBuilder {
  pub fn build(&self) -> PollVoter {
    self.inner.clone()
  }

  pub fn voter_id<T: AsRef<MessageSender>>(&mut self, voter_id: T) -> &mut Self {
    self.inner.voter_id = voter_id.as_ref().clone();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }
}

impl AsRef<PollVoter> for PollVoter {
  fn as_ref(&self) -> &PollVoter {
    self
  }
}

impl AsRef<PollVoter> for RTDPollVoterBuilder {
  fn as_ref(&self) -> &PollVoter {
    &self.inner
  }
}
