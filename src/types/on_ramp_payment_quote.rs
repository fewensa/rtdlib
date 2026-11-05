use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a quote for an on-ramp purchase
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnRampPaymentQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The payment method to be used
  payment_method: String,
  /// Point in time (Unix timestamp) when the quote will expire
  expiration_date: i64,
  /// Paid amount of the base currency
  base_currency_amount: String,
  /// Purchased amount of the cryptocurrency
  cryptocurrency_amount: String,
  /// The price of the cryptocurrency
  cryptocurrency_price: String,
  /// Fee amount
  fee_amount: String,
  /// Extra fee amount
  extra_fee_amount: String,
  /// Network fee amount
  network_fee_amount: String,
  /// Total paid amount
  total_amount: String,
}

impl RObject for OnRampPaymentQuote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "onRampPaymentQuote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl OnRampPaymentQuote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOnRampPaymentQuoteBuilder {
    let inner = OnRampPaymentQuote {
      td_name: "onRampPaymentQuote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOnRampPaymentQuoteBuilder { inner }
  }

  pub fn payment_method(&self) -> &String {
    &self.payment_method
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }

  pub fn base_currency_amount(&self) -> &String {
    &self.base_currency_amount
  }

  pub fn cryptocurrency_amount(&self) -> &String {
    &self.cryptocurrency_amount
  }

  pub fn cryptocurrency_price(&self) -> &String {
    &self.cryptocurrency_price
  }

  pub fn fee_amount(&self) -> &String {
    &self.fee_amount
  }

  pub fn extra_fee_amount(&self) -> &String {
    &self.extra_fee_amount
  }

  pub fn network_fee_amount(&self) -> &String {
    &self.network_fee_amount
  }

  pub fn total_amount(&self) -> &String {
    &self.total_amount
  }
}

#[doc(hidden)]
pub struct RTDOnRampPaymentQuoteBuilder {
  inner: OnRampPaymentQuote,
}

impl RTDOnRampPaymentQuoteBuilder {
  pub fn build(&self) -> OnRampPaymentQuote {
    self.inner.clone()
  }

  pub fn payment_method<T: AsRef<str>>(&mut self, payment_method: T) -> &mut Self {
    self.inner.payment_method = payment_method.as_ref().to_string();
    self
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }

  pub fn base_currency_amount<T: AsRef<str>>(&mut self, base_currency_amount: T) -> &mut Self {
    self.inner.base_currency_amount = base_currency_amount.as_ref().to_string();
    self
  }

  pub fn cryptocurrency_amount<T: AsRef<str>>(&mut self, cryptocurrency_amount: T) -> &mut Self {
    self.inner.cryptocurrency_amount = cryptocurrency_amount.as_ref().to_string();
    self
  }

  pub fn cryptocurrency_price<T: AsRef<str>>(&mut self, cryptocurrency_price: T) -> &mut Self {
    self.inner.cryptocurrency_price = cryptocurrency_price.as_ref().to_string();
    self
  }

  pub fn fee_amount<T: AsRef<str>>(&mut self, fee_amount: T) -> &mut Self {
    self.inner.fee_amount = fee_amount.as_ref().to_string();
    self
  }

  pub fn extra_fee_amount<T: AsRef<str>>(&mut self, extra_fee_amount: T) -> &mut Self {
    self.inner.extra_fee_amount = extra_fee_amount.as_ref().to_string();
    self
  }

  pub fn network_fee_amount<T: AsRef<str>>(&mut self, network_fee_amount: T) -> &mut Self {
    self.inner.network_fee_amount = network_fee_amount.as_ref().to_string();
    self
  }

  pub fn total_amount<T: AsRef<str>>(&mut self, total_amount: T) -> &mut Self {
    self.inner.total_amount = total_amount.as_ref().to_string();
    self
  }
}

impl AsRef<OnRampPaymentQuote> for OnRampPaymentQuote {
  fn as_ref(&self) -> &OnRampPaymentQuote {
    self
  }
}

impl AsRef<OnRampPaymentQuote> for RTDOnRampPaymentQuoteBuilder {
  fn as_ref(&self) -> &OnRampPaymentQuote {
    &self.inner
  }
}
