use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes parameters of a giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the supergroup or channel chat, which will be automatically boosted by the winners of the giveaway for duration of the Telegram Premium subscription, or for the specified time. If the chat is a channel, then can_post_messages administrator right is required in the channel, otherwise, the user must be an administrator in the supergroup
  boosted_chat_id: i64,
  /// Identifiers of other supergroup or channel chats that must be subscribed by the users to be eligible for the giveaway. There can be up to getOption("giveaway_additional_chat_count_max") additional chats
  additional_chat_ids: Vec<i64>,
  /// Point in time (Unix timestamp) when the giveaway is expected to be performed; must be 60-getOption("giveaway_duration_max") seconds in the future in scheduled giveaways
  winners_selection_date: i64,
  /// True, if only new members of the chats will be eligible for the giveaway
  only_new_members: bool,
  /// True, if the list of winners of the giveaway will be available to everyone
  has_public_winners: bool,
  /// The list of two-letter ISO 3166-1 alpha-2 codes of countries, users from which will be eligible for the giveaway. If empty, then all users can participate in the giveaway. There can be up to getOption("giveaway_country_count_max") chosen countries. Users with phone number that was bought at https://fragment.com can participate in any giveaway and the country code "FT" must not be specified in the list
  country_codes: Vec<String>,
  /// Additional description of the giveaway prize; 0-128 characters
  prize_description: String,
}

impl RObject for GiveawayParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiveawayParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayParametersBuilder {
    let inner = GiveawayParameters {
      td_name: "giveawayParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiveawayParametersBuilder { inner }
  }

  pub fn boosted_chat_id(&self) -> i64 {
    self.boosted_chat_id
  }

  pub fn additional_chat_ids(&self) -> &Vec<i64> {
    &self.additional_chat_ids
  }

  pub fn winners_selection_date(&self) -> i64 {
    self.winners_selection_date
  }

  pub fn only_new_members(&self) -> bool {
    self.only_new_members
  }

  pub fn has_public_winners(&self) -> bool {
    self.has_public_winners
  }

  pub fn country_codes(&self) -> &Vec<String> {
    &self.country_codes
  }

  pub fn prize_description(&self) -> &String {
    &self.prize_description
  }
}

#[doc(hidden)]
pub struct RTDGiveawayParametersBuilder {
  inner: GiveawayParameters,
}

impl RTDGiveawayParametersBuilder {
  pub fn build(&self) -> GiveawayParameters {
    self.inner.clone()
  }

  pub fn boosted_chat_id(&mut self, boosted_chat_id: i64) -> &mut Self {
    self.inner.boosted_chat_id = boosted_chat_id;
    self
  }

  pub fn additional_chat_ids(&mut self, additional_chat_ids: Vec<i64>) -> &mut Self {
    self.inner.additional_chat_ids = additional_chat_ids;
    self
  }

  pub fn winners_selection_date(&mut self, winners_selection_date: i64) -> &mut Self {
    self.inner.winners_selection_date = winners_selection_date;
    self
  }

  pub fn only_new_members(&mut self, only_new_members: bool) -> &mut Self {
    self.inner.only_new_members = only_new_members;
    self
  }

  pub fn has_public_winners(&mut self, has_public_winners: bool) -> &mut Self {
    self.inner.has_public_winners = has_public_winners;
    self
  }

  pub fn country_codes(&mut self, country_codes: Vec<String>) -> &mut Self {
    self.inner.country_codes = country_codes;
    self
  }

  pub fn prize_description<T: AsRef<str>>(&mut self, prize_description: T) -> &mut Self {
    self.inner.prize_description = prize_description.as_ref().to_string();
    self
  }
}

impl AsRef<GiveawayParameters> for GiveawayParameters {
  fn as_ref(&self) -> &GiveawayParameters {
    self
  }
}

impl AsRef<GiveawayParameters> for RTDGiveawayParametersBuilder {
  fn as_ref(&self) -> &GiveawayParameters {
    &self.inner
  }
}
