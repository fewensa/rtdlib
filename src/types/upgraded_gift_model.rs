use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a model of an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftModel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the model
  name: String,
  /// The sticker representing the upgraded gift
  sticker: Sticker,
  /// The rarity of the model
  rarity: UpgradedGiftAttributeRarity,
  /// True, if the model can be obtained only through gift crafting
  is_crafted: bool,
}

impl RObject for UpgradedGiftModel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftModel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftModel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftModelBuilder {
    let inner = UpgradedGiftModel {
      td_name: "upgradedGiftModel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftModelBuilder { inner }
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

  pub fn is_crafted(&self) -> bool {
    self.is_crafted
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftModelBuilder {
  inner: UpgradedGiftModel,
}

impl RTDUpgradedGiftModelBuilder {
  pub fn build(&self) -> UpgradedGiftModel {
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

  pub fn is_crafted(&mut self, is_crafted: bool) -> &mut Self {
    self.inner.is_crafted = is_crafted;
    self
  }
}

impl AsRef<UpgradedGiftModel> for UpgradedGiftModel {
  fn as_ref(&self) -> &UpgradedGiftModel {
    self
  }
}

impl AsRef<UpgradedGiftModel> for RTDUpgradedGiftModelBuilder {
  fn as_ref(&self) -> &UpgradedGiftModel {
    &self.inner
  }
}
