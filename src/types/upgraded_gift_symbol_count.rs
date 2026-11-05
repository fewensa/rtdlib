use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a symbol shown on the pattern of an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftSymbolCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The symbol
  symbol: UpgradedGiftSymbol,
  /// Total number of gifts with the symbol
  total_count: i64,
}

impl RObject for UpgradedGiftSymbolCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftSymbolCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftSymbolCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftSymbolCountBuilder {
    let inner = UpgradedGiftSymbolCount {
      td_name: "upgradedGiftSymbolCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftSymbolCountBuilder { inner }
  }

  pub fn symbol(&self) -> &UpgradedGiftSymbol {
    &self.symbol
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftSymbolCountBuilder {
  inner: UpgradedGiftSymbolCount,
}

impl RTDUpgradedGiftSymbolCountBuilder {
  pub fn build(&self) -> UpgradedGiftSymbolCount {
    self.inner.clone()
  }

  pub fn symbol<T: AsRef<UpgradedGiftSymbol>>(&mut self, symbol: T) -> &mut Self {
    self.inner.symbol = symbol.as_ref().clone();
    self
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }
}

impl AsRef<UpgradedGiftSymbolCount> for UpgradedGiftSymbolCount {
  fn as_ref(&self) -> &UpgradedGiftSymbolCount {
    self
  }
}

impl AsRef<UpgradedGiftSymbolCount> for RTDUpgradedGiftSymbolCountBuilder {
  fn as_ref(&self) -> &UpgradedGiftSymbolCount {
    &self.inner
  }
}
