use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains all possible variants of upgraded gifts for the given regular gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftUpgradeVariants {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Models that can be chosen for the gift after upgrade
  models: Vec<UpgradedGiftModel>,
  /// Symbols that can be chosen for the gift after upgrade
  symbols: Vec<UpgradedGiftSymbol>,
  /// Backdrops that can be chosen for the gift after upgrade
  backdrops: Vec<UpgradedGiftBackdrop>,
}

impl RObject for GiftUpgradeVariants {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftUpgradeVariants"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftUpgradeVariants {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftUpgradeVariantsBuilder {
    let inner = GiftUpgradeVariants {
      td_name: "giftUpgradeVariants".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftUpgradeVariantsBuilder { inner }
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
}

#[doc(hidden)]
pub struct RTDGiftUpgradeVariantsBuilder {
  inner: GiftUpgradeVariants,
}

impl RTDGiftUpgradeVariantsBuilder {
  pub fn build(&self) -> GiftUpgradeVariants {
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
}

impl AsRef<GiftUpgradeVariants> for GiftUpgradeVariants {
  fn as_ref(&self) -> &GiftUpgradeVariants {
    self
  }
}

impl AsRef<GiftUpgradeVariants> for RTDGiftUpgradeVariantsBuilder {
  fn as_ref(&self) -> &GiftUpgradeVariants {
    &self.inner
  }
}
