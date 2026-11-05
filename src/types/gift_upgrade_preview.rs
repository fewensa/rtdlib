use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains examples of possible upgraded gifts for the given regular gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftUpgradePreview {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Examples of possible models that can be chosen for the gift after upgrade
  models: Vec<UpgradedGiftModel>,
  /// Examples of possible symbols that can be chosen for the gift after upgrade
  symbols: Vec<UpgradedGiftSymbol>,
  /// Examples of possible backdrops that can be chosen for the gift after upgrade
  backdrops: Vec<UpgradedGiftBackdrop>,
  /// Examples of price for gift upgrade from the maximum price to the minimum price
  prices: Vec<GiftUpgradePrice>,
  /// Next changes for the price for gift upgrade with more granularity than in prices
  next_prices: Vec<GiftUpgradePrice>,
}

impl RObject for GiftUpgradePreview {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftUpgradePreview"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftUpgradePreview {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftUpgradePreviewBuilder {
    let inner = GiftUpgradePreview {
      td_name: "giftUpgradePreview".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftUpgradePreviewBuilder { inner }
  }

  pub fn models(&self) -> &Vec<UpgradedGiftModel> {
    &self.models
  }

  pub fn symbols(&self) -> &Vec<UpgradedGiftSymbol> {
    &self.symbols
  }

  pub fn backdrops(&self) -> &Vec<UpgradedGiftBackdrop> {
    &self.backdrops
  }

  pub fn prices(&self) -> &Vec<GiftUpgradePrice> {
    &self.prices
  }

  pub fn next_prices(&self) -> &Vec<GiftUpgradePrice> {
    &self.next_prices
  }
}

#[doc(hidden)]
pub struct RTDGiftUpgradePreviewBuilder {
  inner: GiftUpgradePreview,
}

impl RTDGiftUpgradePreviewBuilder {
  pub fn build(&self) -> GiftUpgradePreview {
    self.inner.clone()
  }

  pub fn models(&mut self, models: Vec<UpgradedGiftModel>) -> &mut Self {
    self.inner.models = models;
    self
  }

  pub fn symbols(&mut self, symbols: Vec<UpgradedGiftSymbol>) -> &mut Self {
    self.inner.symbols = symbols;
    self
  }

  pub fn backdrops(&mut self, backdrops: Vec<UpgradedGiftBackdrop>) -> &mut Self {
    self.inner.backdrops = backdrops;
    self
  }

  pub fn prices(&mut self, prices: Vec<GiftUpgradePrice>) -> &mut Self {
    self.inner.prices = prices;
    self
  }

  pub fn next_prices(&mut self, next_prices: Vec<GiftUpgradePrice>) -> &mut Self {
    self.inner.next_prices = next_prices;
    self
  }
}

impl AsRef<GiftUpgradePreview> for GiftUpgradePreview {
  fn as_ref(&self) -> &GiftUpgradePreview {
    self
  }
}

impl AsRef<GiftUpgradePreview> for RTDGiftUpgradePreviewBuilder {
  fn as_ref(&self) -> &GiftUpgradePreview {
    &self.inner
  }
}
