use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for creating of Telegram Star giveaway. Use telegramPaymentPurposeStarGiveaway for out-of-store payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarGiveawayPaymentOption {
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
  /// Number of Telegram Stars that will be distributed among winners
  star_count: i64,
  /// Identifier of the store product associated with the option; may be empty if none
  store_product_id: String,
  /// Number of times the chat will be boosted for one year if the option is chosen
  yearly_boost_count: i64,
  /// Allowed options for the number of giveaway winners
  winner_options: Vec<StarGiveawayWinnerOption>,
  /// True, if the option must be chosen by default
  is_default: bool,
  /// True, if the option must be shown only in the full list of payment options
  is_additional: bool,
}

impl RObject for StarGiveawayPaymentOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starGiveawayPaymentOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarGiveawayPaymentOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarGiveawayPaymentOptionBuilder {
    let inner = StarGiveawayPaymentOption {
      td_name: "starGiveawayPaymentOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarGiveawayPaymentOptionBuilder { inner }
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

  pub fn yearly_boost_count(&self) -> i64 {
    self.yearly_boost_count
  }

  pub fn winner_options(&self) -> &Vec<StarGiveawayWinnerOption> {
    &self.winner_options
  }

  pub fn is_default(&self) -> bool {
    self.is_default
  }

  pub fn is_additional(&self) -> bool {
    self.is_additional
  }
}

#[doc(hidden)]
pub struct RTDStarGiveawayPaymentOptionBuilder {
  inner: StarGiveawayPaymentOption,
}

impl RTDStarGiveawayPaymentOptionBuilder {
  pub fn build(&self) -> StarGiveawayPaymentOption {
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

  pub fn yearly_boost_count(&mut self, yearly_boost_count: i64) -> &mut Self {
    self.inner.yearly_boost_count = yearly_boost_count;
    self
  }

  pub fn winner_options(&mut self, winner_options: Vec<StarGiveawayWinnerOption>) -> &mut Self {
    self.inner.winner_options = winner_options;
    self
  }

  pub fn is_default(&mut self, is_default: bool) -> &mut Self {
    self.inner.is_default = is_default;
    self
  }

  pub fn is_additional(&mut self, is_additional: bool) -> &mut Self {
    self.inner.is_additional = is_additional;
    self
  }
}

impl AsRef<StarGiveawayPaymentOption> for StarGiveawayPaymentOption {
  fn as_ref(&self) -> &StarGiveawayPaymentOption {
    self
  }
}

impl AsRef<StarGiveawayPaymentOption> for RTDStarGiveawayPaymentOptionBuilder {
  fn as_ref(&self) -> &StarGiveawayPaymentOption {
    &self.inner
  }
}
