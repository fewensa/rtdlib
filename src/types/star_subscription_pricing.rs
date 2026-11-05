use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes subscription plan paid in Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarSubscriptionPricing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of seconds between consecutive Telegram Star debiting
  period: i64,
  /// The Telegram Star amount that must be paid for each period
  star_count: i64,
}

impl RObject for StarSubscriptionPricing {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starSubscriptionPricing"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarSubscriptionPricing {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarSubscriptionPricingBuilder {
    let inner = StarSubscriptionPricing {
      td_name: "starSubscriptionPricing".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarSubscriptionPricingBuilder { inner }
  }

  pub fn period(&self) -> i64 {
    self.period
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDStarSubscriptionPricingBuilder {
  inner: StarSubscriptionPricing,
}

impl RTDStarSubscriptionPricingBuilder {
  pub fn build(&self) -> StarSubscriptionPricing {
    self.inner.clone()
  }

  pub fn period(&mut self, period: i64) -> &mut Self {
    self.inner.period = period;
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<StarSubscriptionPricing> for StarSubscriptionPricing {
  fn as_ref(&self) -> &StarSubscriptionPricing {
    self
  }
}

impl AsRef<StarSubscriptionPricing> for RTDStarSubscriptionPricingBuilder {
  fn as_ref(&self) -> &StarSubscriptionPricing {
    &self.inner
  }
}
