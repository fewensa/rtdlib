use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for the number of winners of a Telegram Star giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarGiveawayWinnerOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of users that will be chosen as winners
  winner_count: i64,
  /// The number of Telegram Stars that will be won by the winners of the giveaway
  won_star_count: i64,
  /// True, if the option must be chosen by default
  is_default: bool,
}

impl RObject for StarGiveawayWinnerOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starGiveawayWinnerOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarGiveawayWinnerOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarGiveawayWinnerOptionBuilder {
    let inner = StarGiveawayWinnerOption {
      td_name: "starGiveawayWinnerOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarGiveawayWinnerOptionBuilder { inner }
  }

  pub fn winner_count(&self) -> i64 {
    self.winner_count
  }

  pub fn won_star_count(&self) -> i64 {
    self.won_star_count
  }

  pub fn is_default(&self) -> bool {
    self.is_default
  }
}

#[doc(hidden)]
pub struct RTDStarGiveawayWinnerOptionBuilder {
  inner: StarGiveawayWinnerOption,
}

impl RTDStarGiveawayWinnerOptionBuilder {
  pub fn build(&self) -> StarGiveawayWinnerOption {
    self.inner.clone()
  }

  pub fn winner_count(&mut self, winner_count: i64) -> &mut Self {
    self.inner.winner_count = winner_count;
    self
  }

  pub fn won_star_count(&mut self, won_star_count: i64) -> &mut Self {
    self.inner.won_star_count = won_star_count;
    self
  }

  pub fn is_default(&mut self, is_default: bool) -> &mut Self {
    self.inner.is_default = is_default;
    self
  }
}

impl AsRef<StarGiveawayWinnerOption> for StarGiveawayWinnerOption {
  fn as_ref(&self) -> &StarGiveawayWinnerOption {
    self
  }
}

impl AsRef<StarGiveawayWinnerOption> for RTDStarGiveawayWinnerOptionBuilder {
  fn as_ref(&self) -> &StarGiveawayWinnerOption {
    &self.inner
  }
}
