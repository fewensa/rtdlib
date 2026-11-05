use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a symbol shown on the pattern of an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftSymbol {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the symbol
  name: String,
  /// The sticker representing the symbol
  sticker: Sticker,
  /// The rarity of the symbol
  rarity: UpgradedGiftAttributeRarity,
}

impl RObject for UpgradedGiftSymbol {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftSymbol"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftSymbol {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftSymbolBuilder {
    let inner = UpgradedGiftSymbol {
      td_name: "upgradedGiftSymbol".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftSymbolBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn sticker(&self) -> &Sticker {
    &self.sticker
  }

  pub fn rarity(&self) -> &UpgradedGiftAttributeRarity {
    &self.rarity
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftSymbolBuilder {
  inner: UpgradedGiftSymbol,
}

impl RTDUpgradedGiftSymbolBuilder {
  pub fn build(&self) -> UpgradedGiftSymbol {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

  pub fn rarity<T: AsRef<UpgradedGiftAttributeRarity>>(&mut self, rarity: T) -> &mut Self {
    self.inner.rarity = rarity.as_ref().clone();
    self
  }
}

impl AsRef<UpgradedGiftSymbol> for UpgradedGiftSymbol {
  fn as_ref(&self) -> &UpgradedGiftSymbol {
    self
  }
}

impl AsRef<UpgradedGiftSymbol> for RTDUpgradedGiftSymbolBuilder {
  fn as_ref(&self) -> &UpgradedGiftSymbol {
    &self.inner
  }
}
