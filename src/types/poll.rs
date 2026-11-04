use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Poll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique poll identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Poll question; 1-300 characters; may contain only custom emoji entities
  question: FormattedText,
  /// List of poll answer options
  options: Vec<PollOption>,
  /// Total number of voters, participating in the poll
  total_voter_count: i64,
  /// Identifiers of recent voters, if the poll is non-anonymous and poll results are available
  recent_voter_ids: Vec<MessageSender>,
  /// True, if the current user can get voters in the poll using getPollVoters
  can_get_voters: bool,
  /// True, if the current user can see results of the poll
  can_see_results: bool,
  /// True, if the poll is anonymous
  is_anonymous: bool,
  /// True, if multiple answer options can be chosen simultaneously
  allows_multiple_answers: bool,
  /// True, if the poll can be answered multiple times
  allows_revoting: bool,
  /// True, if only the users that are members of the chat for more than a day will be able to vote
  members_only: bool,
  /// The list of two-letter ISO 3166-1 alpha-2 codes of countries, users from which will be able to vote. If empty, then all users can participate in the poll
  country_codes: Vec<String>,
  /// The list of 0-based poll option identifiers in which the options of the poll must be shown; empty if the order of options must not be changed
  option_order: Vec<i64>,
  /// Type of the poll
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: PollType,
  /// Amount of time the poll will be active after creation, in seconds
  open_period: i64,
  /// Point in time (Unix timestamp) when the poll will automatically be closed
  close_date: i64,
  /// True, if the poll is closed
  is_closed: bool,
  /// The reason describing, why the current user can't vote in the poll; may be null if the user can vote in the poll
  vote_restriction_reason: Option<PollVoteRestrictionReason>,
}

impl RObject for Poll {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "poll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Poll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollBuilder {
    let inner = Poll {
      td_name: "poll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn question(&self) -> &FormattedText {
    &self.question
  }

  pub fn options(&self) -> &Vec<PollOption> {
    &self.options
  }

  pub fn total_voter_count(&self) -> i64 {
    self.total_voter_count
  }

  pub fn recent_voter_ids(&self) -> &Vec<MessageSender> {
    &self.recent_voter_ids
  }

  pub fn can_get_voters(&self) -> bool {
    self.can_get_voters
  }

  pub fn can_see_results(&self) -> bool {
    self.can_see_results
  }

  pub fn is_anonymous(&self) -> bool {
    self.is_anonymous
  }

  pub fn allows_multiple_answers(&self) -> bool {
    self.allows_multiple_answers
  }

  pub fn allows_revoting(&self) -> bool {
    self.allows_revoting
  }

  pub fn members_only(&self) -> bool {
    self.members_only
  }

  pub fn country_codes(&self) -> &Vec<String> {
    &self.country_codes
  }

  pub fn option_order(&self) -> &Vec<i64> {
    &self.option_order
  }

  pub fn type_(&self) -> &PollType {
    &self.type_
  }

  pub fn open_period(&self) -> i64 {
    self.open_period
  }

  pub fn close_date(&self) -> i64 {
    self.close_date
  }

  pub fn is_closed(&self) -> bool {
    self.is_closed
  }

  pub fn vote_restriction_reason(&self) -> &Option<PollVoteRestrictionReason> {
    &self.vote_restriction_reason
  }
}

#[doc(hidden)]
pub struct RTDPollBuilder {
  inner: Poll,
}

impl RTDPollBuilder {
  pub fn build(&self) -> Poll {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn question<T: AsRef<FormattedText>>(&mut self, question: T) -> &mut Self {
    self.inner.question = question.as_ref().clone();
    self
  }

  pub fn options(&mut self, options: Vec<PollOption>) -> &mut Self {
    self.inner.options = options;
    self
  }

  pub fn total_voter_count(&mut self, total_voter_count: i64) -> &mut Self {
    self.inner.total_voter_count = total_voter_count;
    self
  }

  pub fn recent_voter_ids(&mut self, recent_voter_ids: Vec<MessageSender>) -> &mut Self {
    self.inner.recent_voter_ids = recent_voter_ids;
    self
  }

  pub fn can_get_voters(&mut self, can_get_voters: bool) -> &mut Self {
    self.inner.can_get_voters = can_get_voters;
    self
  }

  pub fn can_see_results(&mut self, can_see_results: bool) -> &mut Self {
    self.inner.can_see_results = can_see_results;
    self
  }

  pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
    self.inner.is_anonymous = is_anonymous;
    self
  }

  pub fn allows_multiple_answers(&mut self, allows_multiple_answers: bool) -> &mut Self {
    self.inner.allows_multiple_answers = allows_multiple_answers;
    self
  }

  pub fn allows_revoting(&mut self, allows_revoting: bool) -> &mut Self {
    self.inner.allows_revoting = allows_revoting;
    self
  }

  pub fn members_only(&mut self, members_only: bool) -> &mut Self {
    self.inner.members_only = members_only;
    self
  }

  pub fn country_codes(&mut self, country_codes: Vec<String>) -> &mut Self {
    self.inner.country_codes = country_codes;
    self
  }

  pub fn option_order(&mut self, option_order: Vec<i64>) -> &mut Self {
    self.inner.option_order = option_order;
    self
  }

  pub fn type_<T: AsRef<PollType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn open_period(&mut self, open_period: i64) -> &mut Self {
    self.inner.open_period = open_period;
    self
  }

  pub fn close_date(&mut self, close_date: i64) -> &mut Self {
    self.inner.close_date = close_date;
    self
  }

  pub fn is_closed(&mut self, is_closed: bool) -> &mut Self {
    self.inner.is_closed = is_closed;
    self
  }

  pub fn vote_restriction_reason<T: AsRef<PollVoteRestrictionReason>>(
    &mut self,
    vote_restriction_reason: T,
  ) -> &mut Self {
    self.inner.vote_restriction_reason = Some(vote_restriction_reason.as_ref().clone());
    self
  }
}

impl AsRef<Poll> for Poll {
  fn as_ref(&self) -> &Poll {
    self
  }
}

impl AsRef<Poll> for RTDPollBuilder {
  fn as_ref(&self) -> &Poll {
    &self.inner
  }
}
