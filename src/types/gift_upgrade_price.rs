use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a price required to pay to upgrade a gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftUpgradePrice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the price will be in effect
  date: i64,
  /// The Telegram Star amount required to pay to upgrade the gift
  star_count: i64,
}

impl RObject for GiftUpgradePrice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftUpgradePrice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftUpgradePrice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftUpgradePriceBuilder {
    let inner = GiftUpgradePrice {
      td_name: "giftUpgradePrice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftUpgradePriceBuilder { inner }
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDGiftUpgradePriceBuilder {
  inner: GiftUpgradePrice,
}

impl RTDGiftUpgradePriceBuilder {
  pub fn build(&self) -> GiftUpgradePrice {
    self.inner.clone()
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<GiftUpgradePrice> for GiftUpgradePrice {
  fn as_ref(&self) -> &GiftUpgradePrice {
    self
  }
}

impl AsRef<GiftUpgradePrice> for RTDGiftUpgradePriceBuilder {
  fn as_ref(&self) -> &GiftUpgradePrice {
    &self.inner
  }
}
