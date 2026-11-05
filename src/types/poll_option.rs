use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes one answer option of a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the option in the poll; may be empty if yet unassigned
  id: String,
  /// Option text; 1-100 characters; may contain only custom emoji entities
  text: FormattedText,
  /// Option media; may be null if none. If present, currently, can be only of the types pollMediaAnimation, pollMediaLink, pollMediaLocation, pollMediaPhoto, pollMediaSticker, pollMediaVenue, or pollMediaVideo
  media: Option<PollMedia>,
  /// Number of voters for this option, available only for closed or voted polls, or if the current user is the creator of the poll
  voter_count: i64,
  /// The percentage of votes for this option; 0-100
  vote_percentage: i64,
  /// Identifiers of recent voters for the option, if the poll is non-anonymous and poll results are available
  recent_voter_ids: Vec<MessageSender>,
  /// True, if the option was chosen by the user
  is_chosen: bool,
  /// True, if the option is being chosen by a pending setPollAnswer request
  is_being_chosen: bool,
  /// Identifier of the user or chat who added the option; may be null if the option existed from creation of the poll
  author: Option<MessageSender>,
  /// Point in time (Unix timestamp) when the option was added; 0 if the option existed from creation of the poll
  addition_date: i64,
}

impl RObject for PollOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pollOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PollOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPollOptionBuilder {
    let inner = PollOption {
      td_name: "pollOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPollOptionBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn media(&self) -> &Option<PollMedia> {
    &self.media
  }

  pub fn voter_count(&self) -> i64 {
    self.voter_count
  }

  pub fn vote_percentage(&self) -> i64 {
    self.vote_percentage
  }

  pub fn recent_voter_ids(&self) -> &Vec<MessageSender> {
    &self.recent_voter_ids
  }

  pub fn is_chosen(&self) -> bool {
    self.is_chosen
  }

  pub fn is_being_chosen(&self) -> bool {
    self.is_being_chosen
  }

  pub fn author(&self) -> &Option<MessageSender> {
    &self.author
  }

  pub fn addition_date(&self) -> i64 {
    self.addition_date
  }
}

#[doc(hidden)]
pub struct RTDPollOptionBuilder {
  inner: PollOption,
}

impl RTDPollOptionBuilder {
  pub fn build(&self) -> PollOption {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn media<T: AsRef<PollMedia>>(&mut self, media: T) -> &mut Self {
    self.inner.media = Some(media.as_ref().clone());
    self
  }

  pub fn voter_count(&mut self, voter_count: i64) -> &mut Self {
    self.inner.voter_count = voter_count;
    self
  }

  pub fn vote_percentage(&mut self, vote_percentage: i64) -> &mut Self {
    self.inner.vote_percentage = vote_percentage;
    self
  }

  pub fn recent_voter_ids(&mut self, recent_voter_ids: Vec<MessageSender>) -> &mut Self {
    self.inner.recent_voter_ids = recent_voter_ids;
    self
  }

  pub fn is_chosen(&mut self, is_chosen: bool) -> &mut Self {
    self.inner.is_chosen = is_chosen;
    self
  }

  pub fn is_being_chosen(&mut self, is_being_chosen: bool) -> &mut Self {
    self.inner.is_being_chosen = is_being_chosen;
    self
  }

  pub fn author<T: AsRef<MessageSender>>(&mut self, author: T) -> &mut Self {
    self.inner.author = Some(author.as_ref().clone());
    self
  }

  pub fn addition_date(&mut self, addition_date: i64) -> &mut Self {
    self.inner.addition_date = addition_date;
    self
  }
}

impl AsRef<PollOption> for PollOption {
  fn as_ref(&self) -> &PollOption {
    self
  }
}

impl AsRef<PollOption> for RTDPollOptionBuilder {
  fn as_ref(&self) -> &PollOption {
    &self.inner
  }
}
