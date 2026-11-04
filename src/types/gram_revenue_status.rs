use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about TON Grams earned by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GramRevenueStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total Gram amount earned; in the smallest units of the cryptocurrency
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  total_amount: isize,
  /// The Gram amount that isn't withdrawn yet; in the smallest units of the cryptocurrency
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  balance_amount: isize,
  /// The Gram amount that is available for withdrawal; in the smallest units of the cryptocurrency
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  available_amount: isize,
  /// True, if Grams can be withdrawn
  withdrawal_enabled: bool,
}

impl RObject for GramRevenueStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "gramRevenueStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GramRevenueStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGramRevenueStatusBuilder {
    let inner = GramRevenueStatus {
      td_name: "gramRevenueStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGramRevenueStatusBuilder { inner }
  }

  pub fn total_amount(&self) -> isize {
    self.total_amount
  }

  pub fn balance_amount(&self) -> isize {
    self.balance_amount
  }

  pub fn available_amount(&self) -> isize {
    self.available_amount
  }

  pub fn withdrawal_enabled(&self) -> bool {
    self.withdrawal_enabled
  }
}

#[doc(hidden)]
pub struct RTDGramRevenueStatusBuilder {
  inner: GramRevenueStatus,
}

impl RTDGramRevenueStatusBuilder {
  pub fn build(&self) -> GramRevenueStatus {
    self.inner.clone()
  }

  pub fn total_amount(&mut self, total_amount: isize) -> &mut Self {
    self.inner.total_amount = total_amount;
    self
  }

  pub fn balance_amount(&mut self, balance_amount: isize) -> &mut Self {
    self.inner.balance_amount = balance_amount;
    self
  }

  pub fn available_amount(&mut self, available_amount: isize) -> &mut Self {
    self.inner.available_amount = available_amount;
    self
  }

  pub fn withdrawal_enabled(&mut self, withdrawal_enabled: bool) -> &mut Self {
    self.inner.withdrawal_enabled = withdrawal_enabled;
    self
  }
}

impl AsRef<GramRevenueStatus> for GramRevenueStatus {
  fn as_ref(&self) -> &GramRevenueStatus {
    self
  }
}

impl AsRef<GramRevenueStatus> for RTDGramRevenueStatusBuilder {
  fn as_ref(&self) -> &GramRevenueStatus {
    &self.inner
  }
}
