use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for gifting Telegram Premium to a user. Use telegramPaymentPurposePremiumGift for out-of-store payments or payments in Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiftPaymentOption {
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
  /// The alternative Telegram Star amount to pay; 0 if payment in Telegram Stars is not possible
  star_count: i64,
  /// The discount associated with this option, as a percentage
  discount_percentage: i64,
  /// Number of months the Telegram Premium subscription will be active
  month_count: i64,
  /// Identifier of the store product associated with the option
  store_product_id: String,
  /// A sticker to be shown along with the option; may be null if unknown
  sticker: Option<Sticker>,
}

impl RObject for PremiumGiftPaymentOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumGiftPaymentOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumGiftPaymentOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumGiftPaymentOptionBuilder {
    let inner = PremiumGiftPaymentOption {
      td_name: "premiumGiftPaymentOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumGiftPaymentOptionBuilder { inner }
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

  pub fn discount_percentage(&self) -> i64 {
    self.discount_percentage
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }

  pub fn store_product_id(&self) -> &String {
    &self.store_product_id
  }

  pub fn sticker(&self) -> &Option<Sticker> {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDPremiumGiftPaymentOptionBuilder {
  inner: PremiumGiftPaymentOption,
}

impl RTDPremiumGiftPaymentOptionBuilder {
  pub fn build(&self) -> PremiumGiftPaymentOption {
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

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }
}

impl AsRef<PremiumGiftPaymentOption> for PremiumGiftPaymentOption {
  fn as_ref(&self) -> &PremiumGiftPaymentOption {
    self
  }
}

impl AsRef<PremiumGiftPaymentOption> for RTDPremiumGiftPaymentOptionBuilder {
  fn as_ref(&self) -> &PremiumGiftPaymentOption {
    &self.inner
  }
}
