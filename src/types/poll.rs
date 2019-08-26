
use crate::types::*;
use crate::errors::*;




/// Describes a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Poll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Unique poll identifier
  id: String,
  /// Poll question, 1-255 characters
  question: String,
  /// List of poll answer options
  options: Vec<PollOption>,
  /// Total number of voters, participating in the poll
  total_voter_count: i32,
  /// True, if the poll is closed
  is_closed: bool,
  
}

impl RObject for Poll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "poll" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Poll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPollBuilder {
    let mut inner = Poll::default();
    inner.td_name = "poll".to_string();
    RTDPollBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn question(&self) -> &String { &self.question }

  pub fn options(&self) -> &Vec<PollOption> { &self.options }

  pub fn total_voter_count(&self) -> i32 { self.total_voter_count }

  pub fn is_closed(&self) -> bool { self.is_closed }

}

#[doc(hidden)]
pub struct RTDPollBuilder {
  inner: Poll
}

impl RTDPollBuilder {
  pub fn build(&self) -> Poll { self.inner.clone() }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn question<T: AsRef<str>>(&mut self, question: T) -> &mut Self {
    self.inner.question = question.as_ref().to_string();
    self
  }

   
  pub fn options(&mut self, options: Vec<PollOption>) -> &mut Self {
    self.inner.options = options;
    self
  }

   
  pub fn total_voter_count(&mut self, total_voter_count: i32) -> &mut Self {
    self.inner.total_voter_count = total_voter_count;
    self
  }

   
  pub fn is_closed(&mut self, is_closed: bool) -> &mut Self {
    self.inner.is_closed = is_closed;
    self
  }

}

impl AsRef<Poll> for Poll {
  fn as_ref(&self) -> &Poll { self }
}

impl AsRef<Poll> for RTDPollBuilder {
  fn as_ref(&self) -> &Poll { &self.inner }
}



