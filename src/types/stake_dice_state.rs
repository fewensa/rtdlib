use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes state of the stake dice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StakeDiceState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hash of the state to use for sending the next dice; may be empty if the stake dice can't be sent by the current user
  state_hash: String,
  /// The amount of TON Grams staked in the previous roll; in the smallest units of the currency
  stake_gram_amount: i64,
  /// The amounts of Grams that are suggested to be staked; in the smallest units of the currency
  suggested_stake_gram_amounts: Vec<i64>,
  /// The number of rolled sixes towards the streak; 0-2
  current_streak: i64,
  /// The number of Grams received by the user for each 1000 Grams staked if the dice outcome is 1-6 correspondingly; may be empty if the stake dice can't be sent by the current user
  prize_per_mille: Vec<i64>,
  /// The number of Grams received by the user for each 1000 Grams staked if the dice outcome is 6 three times in a row with the same stake
  streak_prize_per_mille: i64,
}

impl RObject for StakeDiceState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "stakeDiceState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StakeDiceState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStakeDiceStateBuilder {
    let inner = StakeDiceState {
      td_name: "stakeDiceState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStakeDiceStateBuilder { inner }
  }

  pub fn state_hash(&self) -> &String {
    &self.state_hash
  }

  pub fn stake_gram_amount(&self) -> i64 {
    self.stake_gram_amount
  }

  pub fn suggested_stake_gram_amounts(&self) -> &Vec<i64> {
    &self.suggested_stake_gram_amounts
  }

  pub fn current_streak(&self) -> i64 {
    self.current_streak
  }

  pub fn prize_per_mille(&self) -> &Vec<i64> {
    &self.prize_per_mille
  }

  pub fn streak_prize_per_mille(&self) -> i64 {
    self.streak_prize_per_mille
  }
}

#[doc(hidden)]
pub struct RTDStakeDiceStateBuilder {
  inner: StakeDiceState,
}

impl RTDStakeDiceStateBuilder {
  pub fn build(&self) -> StakeDiceState {
    self.inner.clone()
  }

  pub fn state_hash<T: AsRef<str>>(&mut self, state_hash: T) -> &mut Self {
    self.inner.state_hash = state_hash.as_ref().to_string();
    self
  }

  pub fn stake_gram_amount(&mut self, stake_gram_amount: i64) -> &mut Self {
    self.inner.stake_gram_amount = stake_gram_amount;
    self
  }

  pub fn suggested_stake_gram_amounts(
    &mut self,
    suggested_stake_gram_amounts: Vec<i64>,
  ) -> &mut Self {
    self.inner.suggested_stake_gram_amounts = suggested_stake_gram_amounts;
    self
  }

  pub fn current_streak(&mut self, current_streak: i64) -> &mut Self {
    self.inner.current_streak = current_streak;
    self
  }

  pub fn prize_per_mille(&mut self, prize_per_mille: Vec<i64>) -> &mut Self {
    self.inner.prize_per_mille = prize_per_mille;
    self
  }

  pub fn streak_prize_per_mille(&mut self, streak_prize_per_mille: i64) -> &mut Self {
    self.inner.streak_prize_per_mille = streak_prize_per_mille;
    self
  }
}

impl AsRef<StakeDiceState> for StakeDiceState {
  fn as_ref(&self) -> &StakeDiceState {
    self
  }
}

impl AsRef<StakeDiceState> for RTDStakeDiceStateBuilder {
  fn as_ref(&self) -> &StakeDiceState {
    &self.inner
  }
}
