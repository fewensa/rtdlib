use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a prepaid giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrepaidGiveaway {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the prepaid giveaway
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Number of users who will receive giveaway prize
  winner_count: i64,
  /// Prize of the giveaway
  prize: GiveawayPrize,
  /// The number of boosts received by the chat from the giveaway; for Telegram Star giveaways only
  boost_count: i64,
  /// Point in time (Unix timestamp) when the giveaway was paid
  payment_date: i64,
}

impl RObject for PrepaidGiveaway {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "prepaidGiveaway"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PrepaidGiveaway {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPrepaidGiveawayBuilder {
    let inner = PrepaidGiveaway {
      td_name: "prepaidGiveaway".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPrepaidGiveawayBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn winner_count(&self) -> i64 {
    self.winner_count
  }

  pub fn prize(&self) -> &GiveawayPrize {
    &self.prize
  }

  pub fn boost_count(&self) -> i64 {
    self.boost_count
  }

  pub fn payment_date(&self) -> i64 {
    self.payment_date
  }
}

#[doc(hidden)]
pub struct RTDPrepaidGiveawayBuilder {
  inner: PrepaidGiveaway,
}

impl RTDPrepaidGiveawayBuilder {
  pub fn build(&self) -> PrepaidGiveaway {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn winner_count(&mut self, winner_count: i64) -> &mut Self {
    self.inner.winner_count = winner_count;
    self
  }

  pub fn prize<T: AsRef<GiveawayPrize>>(&mut self, prize: T) -> &mut Self {
    self.inner.prize = prize.as_ref().clone();
    self
  }

  pub fn boost_count(&mut self, boost_count: i64) -> &mut Self {
    self.inner.boost_count = boost_count;
    self
  }

  pub fn payment_date(&mut self, payment_date: i64) -> &mut Self {
    self.inner.payment_date = payment_date;
    self
  }
}

impl AsRef<PrepaidGiveaway> for PrepaidGiveaway {
  fn as_ref(&self) -> &PrepaidGiveaway {
    self
  }
}

impl AsRef<PrepaidGiveaway> for RTDPrepaidGiveawayBuilder {
  fn as_ref(&self) -> &PrepaidGiveaway {
    &self.inner
  }
}
