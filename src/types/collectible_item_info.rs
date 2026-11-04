use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a collectible item and its last purchase
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectibleItemInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the item was purchased
  purchase_date: i64,
  /// Currency for the paid amount
  currency: String,
  /// The paid amount, in the smallest units of the currency
  amount: i64,
  /// Cryptocurrency used to pay for the item
  cryptocurrency: String,
  /// The paid amount, in the smallest units of the cryptocurrency
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  cryptocurrency_amount: isize,
  /// Individual URL for the item on https://fragment.com
  url: String,
}

impl RObject for CollectibleItemInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "collectibleItemInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CollectibleItemInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCollectibleItemInfoBuilder {
    let inner = CollectibleItemInfo {
      td_name: "collectibleItemInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCollectibleItemInfoBuilder { inner }
  }

  pub fn purchase_date(&self) -> i64 {
    self.purchase_date
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn cryptocurrency(&self) -> &String {
    &self.cryptocurrency
  }

  pub fn cryptocurrency_amount(&self) -> isize {
    self.cryptocurrency_amount
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDCollectibleItemInfoBuilder {
  inner: CollectibleItemInfo,
}

impl RTDCollectibleItemInfoBuilder {
  pub fn build(&self) -> CollectibleItemInfo {
    self.inner.clone()
  }

  pub fn purchase_date(&mut self, purchase_date: i64) -> &mut Self {
    self.inner.purchase_date = purchase_date;
    self
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn cryptocurrency<T: AsRef<str>>(&mut self, cryptocurrency: T) -> &mut Self {
    self.inner.cryptocurrency = cryptocurrency.as_ref().to_string();
    self
  }

  pub fn cryptocurrency_amount(&mut self, cryptocurrency_amount: isize) -> &mut Self {
    self.inner.cryptocurrency_amount = cryptocurrency_amount;
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<CollectibleItemInfo> for CollectibleItemInfo {
  fn as_ref(&self) -> &CollectibleItemInfo {
    self
  }
}

impl AsRef<CollectibleItemInfo> for RTDCollectibleItemInfoBuilder {
  fn as_ref(&self) -> &CollectibleItemInfo {
    &self.inner
  }
}
