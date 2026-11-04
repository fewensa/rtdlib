use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a backdrop of an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftBackdrop {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the backdrop
  id: i64,
  /// Name of the backdrop
  name: String,
  /// Colors of the backdrop
  colors: UpgradedGiftBackdropColors,
  /// The rarity of the backdrop
  rarity: UpgradedGiftAttributeRarity,
}

impl RObject for UpgradedGiftBackdrop {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftBackdrop"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftBackdrop {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftBackdropBuilder {
    let inner = UpgradedGiftBackdrop {
      td_name: "upgradedGiftBackdrop".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftBackdropBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn colors(&self) -> &UpgradedGiftBackdropColors {
    &self.colors
  }

  pub fn rarity(&self) -> &UpgradedGiftAttributeRarity {
    &self.rarity
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftBackdropBuilder {
  inner: UpgradedGiftBackdrop,
}

impl RTDUpgradedGiftBackdropBuilder {
  pub fn build(&self) -> UpgradedGiftBackdrop {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn colors<T: AsRef<UpgradedGiftBackdropColors>>(&mut self, colors: T) -> &mut Self {
    self.inner.colors = colors.as_ref().clone();
    self
  }

  pub fn rarity<T: AsRef<UpgradedGiftAttributeRarity>>(&mut self, rarity: T) -> &mut Self {
    self.inner.rarity = rarity.as_ref().clone();
    self
  }
}

impl AsRef<UpgradedGiftBackdrop> for UpgradedGiftBackdrop {
  fn as_ref(&self) -> &UpgradedGiftBackdrop {
    self
  }
}

impl AsRef<UpgradedGiftBackdrop> for RTDUpgradedGiftBackdropBuilder {
  fn as_ref(&self) -> &UpgradedGiftBackdrop {
    &self.inner
  }
}
