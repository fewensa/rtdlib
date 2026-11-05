use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about value of an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftValueInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// ISO 4217 currency code of the currency in which the prices are represented
  currency: String,
  /// Estimated value of the gift; in the smallest units of the currency
  value: i64,
  /// True, if the value is calculated as average value of similar sold gifts. Otherwise, it is based on the sale price of the gift
  is_value_average: bool,
  /// Point in time (Unix timestamp) when the corresponding regular gift was originally purchased
  initial_sale_date: i64,
  /// The Telegram Star amount that was paid for the gift
  initial_sale_star_count: i64,
  /// Initial price of the gift; in the smallest units of the currency
  initial_sale_price: i64,
  /// Point in time (Unix timestamp) when the upgraded gift was purchased last time; 0 if never
  last_sale_date: i64,
  /// Last purchase price of the gift; in the smallest units of the currency; 0 if the gift has never been resold
  last_sale_price: i64,
  /// True, if the last sale was completed on Fragment
  is_last_sale_on_fragment: bool,
  /// The current minimum price of gifts upgraded from the same gift; in the smallest units of the currency; 0 if there are no such gifts
  minimum_price: i64,
  /// The average sale price in the last month of gifts upgraded from the same gift; in the smallest units of the currency; 0 if there were no such sales
  average_sale_price: i64,
  /// Number of gifts upgraded from the same gift being resold on Telegram
  telegram_listed_gift_count: i64,
  /// Number of gifts upgraded from the same gift being resold on Fragment
  fragment_listed_gift_count: i64,
  /// The HTTPS link to the Fragment for the gift; may be empty if there are no such gifts being sold on Fragment
  fragment_url: String,
}

impl RObject for UpgradedGiftValueInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftValueInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftValueInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftValueInfoBuilder {
    let inner = UpgradedGiftValueInfo {
      td_name: "upgradedGiftValueInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftValueInfoBuilder { inner }
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn value(&self) -> i64 {
    self.value
  }

  pub fn is_value_average(&self) -> bool {
    self.is_value_average
  }

  pub fn initial_sale_date(&self) -> i64 {
    self.initial_sale_date
  }

  pub fn initial_sale_star_count(&self) -> i64 {
    self.initial_sale_star_count
  }

  pub fn initial_sale_price(&self) -> i64 {
    self.initial_sale_price
  }

  pub fn last_sale_date(&self) -> i64 {
    self.last_sale_date
  }

  pub fn last_sale_price(&self) -> i64 {
    self.last_sale_price
  }

  pub fn is_last_sale_on_fragment(&self) -> bool {
    self.is_last_sale_on_fragment
  }

  pub fn minimum_price(&self) -> i64 {
    self.minimum_price
  }

  pub fn average_sale_price(&self) -> i64 {
    self.average_sale_price
  }

  pub fn telegram_listed_gift_count(&self) -> i64 {
    self.telegram_listed_gift_count
  }

  pub fn fragment_listed_gift_count(&self) -> i64 {
    self.fragment_listed_gift_count
  }

  pub fn fragment_url(&self) -> &String {
    &self.fragment_url
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftValueInfoBuilder {
  inner: UpgradedGiftValueInfo,
}

impl RTDUpgradedGiftValueInfoBuilder {
  pub fn build(&self) -> UpgradedGiftValueInfo {
    self.inner.clone()
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn value(&mut self, value: i64) -> &mut Self {
    self.inner.value = value;
    self
  }

  pub fn is_value_average(&mut self, is_value_average: bool) -> &mut Self {
    self.inner.is_value_average = is_value_average;
    self
  }

  pub fn initial_sale_date(&mut self, initial_sale_date: i64) -> &mut Self {
    self.inner.initial_sale_date = initial_sale_date;
    self
  }

  pub fn initial_sale_star_count(&mut self, initial_sale_star_count: i64) -> &mut Self {
    self.inner.initial_sale_star_count = initial_sale_star_count;
    self
  }

  pub fn initial_sale_price(&mut self, initial_sale_price: i64) -> &mut Self {
    self.inner.initial_sale_price = initial_sale_price;
    self
  }

  pub fn last_sale_date(&mut self, last_sale_date: i64) -> &mut Self {
    self.inner.last_sale_date = last_sale_date;
    self
  }

  pub fn last_sale_price(&mut self, last_sale_price: i64) -> &mut Self {
    self.inner.last_sale_price = last_sale_price;
    self
  }

  pub fn is_last_sale_on_fragment(&mut self, is_last_sale_on_fragment: bool) -> &mut Self {
    self.inner.is_last_sale_on_fragment = is_last_sale_on_fragment;
    self
  }

  pub fn minimum_price(&mut self, minimum_price: i64) -> &mut Self {
    self.inner.minimum_price = minimum_price;
    self
  }

  pub fn average_sale_price(&mut self, average_sale_price: i64) -> &mut Self {
    self.inner.average_sale_price = average_sale_price;
    self
  }

  pub fn telegram_listed_gift_count(&mut self, telegram_listed_gift_count: i64) -> &mut Self {
    self.inner.telegram_listed_gift_count = telegram_listed_gift_count;
    self
  }

  pub fn fragment_listed_gift_count(&mut self, fragment_listed_gift_count: i64) -> &mut Self {
    self.inner.fragment_listed_gift_count = fragment_listed_gift_count;
    self
  }

  pub fn fragment_url<T: AsRef<str>>(&mut self, fragment_url: T) -> &mut Self {
    self.inner.fragment_url = fragment_url.as_ref().to_string();
    self
  }
}

impl AsRef<UpgradedGiftValueInfo> for UpgradedGiftValueInfo {
  fn as_ref(&self) -> &UpgradedGiftValueInfo {
    self
  }
}

impl AsRef<UpgradedGiftValueInfo> for RTDUpgradedGiftValueInfoBuilder {
  fn as_ref(&self) -> &UpgradedGiftValueInfo {
    &self.inner
  }
}
