use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for buying Telegram Premium to a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumPaymentOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// ISO 4217 currency code for Telegram Premium subscription payment
  currency: String,
  /// The amount to pay, in the smallest units of the currency
  amount: i64,
  /// The discount associated with this option, as a percentage
  discount_percentage: i64,
  /// Number of months the Telegram Premium subscription will be active. Use getPremiumInfoSticker to get the sticker to be used as representation of the Telegram Premium subscription
  month_count: i64,
  /// Identifier of the store product associated with the option
  store_product_id: String,
  /// An internal link to be opened for buying Telegram Premium to the user if store payment isn't possible; may be null if direct payment isn't available
  payment_link: Option<InternalLinkType>,
}

impl RObject for PremiumPaymentOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumPaymentOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumPaymentOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumPaymentOptionBuilder {
    let inner = PremiumPaymentOption {
      td_name: "premiumPaymentOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumPaymentOptionBuilder { inner }
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn discount_percentage(&self) -> i64 {
    self.discount_percentage
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }

  pub fn store_product_id(&self) -> &String {
    &self.store_product_id
  }

  pub fn payment_link(&self) -> &Option<InternalLinkType> {
    &self.payment_link
  }
}

#[doc(hidden)]
pub struct RTDPremiumPaymentOptionBuilder {
  inner: PremiumPaymentOption,
}

impl RTDPremiumPaymentOptionBuilder {
  pub fn build(&self) -> PremiumPaymentOption {
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

  pub fn discount_percentage(&mut self, discount_percentage: i64) -> &mut Self {
    self.inner.discount_percentage = discount_percentage;
    self
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }

  pub fn store_product_id<T: AsRef<str>>(&mut self, store_product_id: T) -> &mut Self {
    self.inner.store_product_id = store_product_id.as_ref().to_string();
    self
  }

  pub fn payment_link<T: AsRef<InternalLinkType>>(&mut self, payment_link: T) -> &mut Self {
    self.inner.payment_link = Some(payment_link.as_ref().clone());
    self
  }
}

impl AsRef<PremiumPaymentOption> for PremiumPaymentOption {
  fn as_ref(&self) -> &PremiumPaymentOption {
    self
  }
}

impl AsRef<PremiumPaymentOption> for RTDPremiumPaymentOptionBuilder {
  fn as_ref(&self) -> &PremiumPaymentOption {
    &self.inner
  }
}
