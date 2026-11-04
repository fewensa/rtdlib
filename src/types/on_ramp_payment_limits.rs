use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes limits for an on-ramp purchase
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnRampPaymentLimits {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The payment method to be used
  payment_method: String,
  /// The minimum allowed amount of the base currency
  base_currency_min_amount: String,
  /// The maximum allowed amount of the base currency
  base_currency_max_amount: String,
  /// The minimum allowed amount of the bought cryptocurrency
  cryptocurrency_min_amount: String,
  /// The maximum allowed amount of the bought cryptocurrency
  cryptocurrency_max_amount: String,
}

impl RObject for OnRampPaymentLimits {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "onRampPaymentLimits"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl OnRampPaymentLimits {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOnRampPaymentLimitsBuilder {
    let inner = OnRampPaymentLimits {
      td_name: "onRampPaymentLimits".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOnRampPaymentLimitsBuilder { inner }
  }

  pub fn payment_method(&self) -> &String {
    &self.payment_method
  }

  pub fn base_currency_min_amount(&self) -> &String {
    &self.base_currency_min_amount
  }

  pub fn base_currency_max_amount(&self) -> &String {
    &self.base_currency_max_amount
  }

  pub fn cryptocurrency_min_amount(&self) -> &String {
    &self.cryptocurrency_min_amount
  }

  pub fn cryptocurrency_max_amount(&self) -> &String {
    &self.cryptocurrency_max_amount
  }
}

#[doc(hidden)]
pub struct RTDOnRampPaymentLimitsBuilder {
  inner: OnRampPaymentLimits,
}

impl RTDOnRampPaymentLimitsBuilder {
  pub fn build(&self) -> OnRampPaymentLimits {
    self.inner.clone()
  }

  pub fn payment_method<T: AsRef<str>>(&mut self, payment_method: T) -> &mut Self {
    self.inner.payment_method = payment_method.as_ref().to_string();
    self
  }

  pub fn base_currency_min_amount<T: AsRef<str>>(
    &mut self,
    base_currency_min_amount: T,
  ) -> &mut Self {
    self.inner.base_currency_min_amount = base_currency_min_amount.as_ref().to_string();
    self
  }

  pub fn base_currency_max_amount<T: AsRef<str>>(
    &mut self,
    base_currency_max_amount: T,
  ) -> &mut Self {
    self.inner.base_currency_max_amount = base_currency_max_amount.as_ref().to_string();
    self
  }

  pub fn cryptocurrency_min_amount<T: AsRef<str>>(
    &mut self,
    cryptocurrency_min_amount: T,
  ) -> &mut Self {
    self.inner.cryptocurrency_min_amount = cryptocurrency_min_amount.as_ref().to_string();
    self
  }

  pub fn cryptocurrency_max_amount<T: AsRef<str>>(
    &mut self,
    cryptocurrency_max_amount: T,
  ) -> &mut Self {
    self.inner.cryptocurrency_max_amount = cryptocurrency_max_amount.as_ref().to_string();
    self
  }
}

impl AsRef<OnRampPaymentLimits> for OnRampPaymentLimits {
  fn as_ref(&self) -> &OnRampPaymentLimits {
    self
  }
}

impl AsRef<OnRampPaymentLimits> for RTDOnRampPaymentLimitsBuilder {
  fn as_ref(&self) -> &OnRampPaymentLimits {
    &self.inner
  }
}
