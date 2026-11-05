use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about Telegram Stars earned by a user or a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarRevenueStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total Telegram Star amount earned
  total_amount: StarAmount,
  /// The Telegram Star amount that isn't withdrawn yet
  current_amount: StarAmount,
  /// The Telegram Star amount that is available for withdrawal
  available_amount: StarAmount,
  /// True, if Telegram Stars can be withdrawn now or later
  withdrawal_enabled: bool,
  /// Time left before the next withdrawal can be started, in seconds; 0 if withdrawal can be started now
  next_withdrawal_in: i64,
}

impl RObject for StarRevenueStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starRevenueStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarRevenueStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarRevenueStatusBuilder {
    let inner = StarRevenueStatus {
      td_name: "starRevenueStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarRevenueStatusBuilder { inner }
  }

  pub fn total_amount(&self) -> &StarAmount {
    &self.total_amount
  }

  pub fn current_amount(&self) -> &StarAmount {
    &self.current_amount
  }

  pub fn available_amount(&self) -> &StarAmount {
    &self.available_amount
  }

  pub fn withdrawal_enabled(&self) -> bool {
    self.withdrawal_enabled
  }

  pub fn next_withdrawal_in(&self) -> i64 {
    self.next_withdrawal_in
  }
}

#[doc(hidden)]
pub struct RTDStarRevenueStatusBuilder {
  inner: StarRevenueStatus,
}

impl RTDStarRevenueStatusBuilder {
  pub fn build(&self) -> StarRevenueStatus {
    self.inner.clone()
  }

  pub fn total_amount<T: AsRef<StarAmount>>(&mut self, total_amount: T) -> &mut Self {
    self.inner.total_amount = total_amount.as_ref().clone();
    self
  }

  pub fn current_amount<T: AsRef<StarAmount>>(&mut self, current_amount: T) -> &mut Self {
    self.inner.current_amount = current_amount.as_ref().clone();
    self
  }

  pub fn available_amount<T: AsRef<StarAmount>>(&mut self, available_amount: T) -> &mut Self {
    self.inner.available_amount = available_amount.as_ref().clone();
    self
  }

  pub fn withdrawal_enabled(&mut self, withdrawal_enabled: bool) -> &mut Self {
    self.inner.withdrawal_enabled = withdrawal_enabled;
    self
  }

  pub fn next_withdrawal_in(&mut self, next_withdrawal_in: i64) -> &mut Self {
    self.inner.next_withdrawal_in = next_withdrawal_in;
    self
  }
}

impl AsRef<StarRevenueStatus> for StarRevenueStatus {
  fn as_ref(&self) -> &StarRevenueStatus {
    self
  }
}

impl AsRef<StarRevenueStatus> for RTDStarRevenueStatusBuilder {
  fn as_ref(&self) -> &StarRevenueStatus {
    &self.inner
  }
}
