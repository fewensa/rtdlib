use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members. Use telegramPaymentPurposePremiumGiftCodes or telegramPaymentPurposePremiumGiveaway for out-of-store payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayPaymentOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// ISO 4217 currency code for Telegram Premium gift code payment
  currency: String,
  /// The amount to pay, in the smallest units of the currency
  amount: i64,
  /// Number of users who will be able to activate the gift codes
  winner_count: i64,
  /// Number of months the Telegram Premium subscription will be active
  month_count: i64,
  /// Identifier of the store product associated with the option; may be empty if none
  store_product_id: String,
  /// Number of times the store product must be paid
  store_product_quantity: i64,
}

impl RObject for PremiumGiveawayPaymentOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumGiveawayPaymentOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumGiveawayPaymentOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumGiveawayPaymentOptionBuilder {
    let inner = PremiumGiveawayPaymentOption {
      td_name: "premiumGiveawayPaymentOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumGiveawayPaymentOptionBuilder { inner }
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn winner_count(&self) -> i64 {
    self.winner_count
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }

  pub fn store_product_id(&self) -> &String {
    &self.store_product_id
  }

  pub fn store_product_quantity(&self) -> i64 {
    self.store_product_quantity
  }
}

#[doc(hidden)]
pub struct RTDPremiumGiveawayPaymentOptionBuilder {
  inner: PremiumGiveawayPaymentOption,
}

impl RTDPremiumGiveawayPaymentOptionBuilder {
  pub fn build(&self) -> PremiumGiveawayPaymentOption {
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

  pub fn winner_count(&mut self, winner_count: i64) -> &mut Self {
    self.inner.winner_count = winner_count;
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

  pub fn store_product_quantity(&mut self, store_product_quantity: i64) -> &mut Self {
    self.inner.store_product_quantity = store_product_quantity;
    self
  }
}

impl AsRef<PremiumGiveawayPaymentOption> for PremiumGiveawayPaymentOption {
  fn as_ref(&self) -> &PremiumGiveawayPaymentOption {
    self
  }
}

impl AsRef<PremiumGiveawayPaymentOption> for RTDPremiumGiveawayPaymentOptionBuilder {
  fn as_ref(&self) -> &PremiumGiveawayPaymentOption {
    &self.inner
  }
}
