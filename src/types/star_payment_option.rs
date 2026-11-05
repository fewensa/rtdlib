use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for buying Telegram Stars. Use telegramPaymentPurposeStars for out-of-store payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarPaymentOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// ISO 4217 currency code for the payment
  currency: String,
  /// The amount to pay, in the smallest units of the currency
  amount: i64,
  /// Number of Telegram Stars that will be purchased
  star_count: i64,
  /// Identifier of the store product associated with the option; may be empty if none
  store_product_id: String,
  /// True, if the option must be shown only in the full list of payment options
  is_additional: bool,
}

impl RObject for StarPaymentOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starPaymentOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarPaymentOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarPaymentOptionBuilder {
    let inner = StarPaymentOption {
      td_name: "starPaymentOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarPaymentOptionBuilder { inner }
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn store_product_id(&self) -> &String {
    &self.store_product_id
  }

  pub fn is_additional(&self) -> bool {
    self.is_additional
  }
}

#[doc(hidden)]
pub struct RTDStarPaymentOptionBuilder {
  inner: StarPaymentOption,
}

impl RTDStarPaymentOptionBuilder {
  pub fn build(&self) -> StarPaymentOption {
    self.inner.clone()
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn store_product_id<T: AsRef<str>>(&mut self, store_product_id: T) -> &mut Self {
    self.inner.store_product_id = store_product_id.as_ref().to_string();
    self
  }

  pub fn is_additional(&mut self, is_additional: bool) -> &mut Self {
    self.inner.is_additional = is_additional;
    self
  }
}

impl AsRef<StarPaymentOption> for StarPaymentOption {
  fn as_ref(&self) -> &StarPaymentOption {
    self
  }
}

impl AsRef<StarPaymentOption> for RTDStarPaymentOptionBuilder {
  fn as_ref(&self) -> &StarPaymentOption {
    &self.inner
  }
}
