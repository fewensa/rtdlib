use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes colors of a backdrop of an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftBackdropColors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A color in the center of the backdrop in the RGB format
  center_color: i64,
  /// A color on the edges of the backdrop in the RGB format
  edge_color: i64,
  /// A color to be applied for the symbol in the RGB format
  symbol_color: i64,
  /// A color for the text on the backdrop in the RGB format
  text_color: i64,
}

impl RObject for UpgradedGiftBackdropColors {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftBackdropColors"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftBackdropColors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftBackdropColorsBuilder {
    let inner = UpgradedGiftBackdropColors {
      td_name: "upgradedGiftBackdropColors".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftBackdropColorsBuilder { inner }
  }

  pub fn center_color(&self) -> i64 {
    self.center_color
  }

  pub fn edge_color(&self) -> i64 {
    self.edge_color
  }

  pub fn symbol_color(&self) -> i64 {
    self.symbol_color
  }

  pub fn text_color(&self) -> i64 {
    self.text_color
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftBackdropColorsBuilder {
  inner: UpgradedGiftBackdropColors,
}

impl RTDUpgradedGiftBackdropColorsBuilder {
  pub fn build(&self) -> UpgradedGiftBackdropColors {
    self.inner.clone()
  }

  pub fn center_color(&mut self, center_color: i64) -> &mut Self {
    self.inner.center_color = center_color;
    self
  }

  pub fn edge_color(&mut self, edge_color: i64) -> &mut Self {
    self.inner.edge_color = edge_color;
    self
  }

  pub fn symbol_color(&mut self, symbol_color: i64) -> &mut Self {
    self.inner.symbol_color = symbol_color;
    self
  }

  pub fn text_color(&mut self, text_color: i64) -> &mut Self {
    self.inner.text_color = text_color;
    self
  }
}

impl AsRef<UpgradedGiftBackdropColors> for UpgradedGiftBackdropColors {
  fn as_ref(&self) -> &UpgradedGiftBackdropColors {
    self
  }
}

impl AsRef<UpgradedGiftBackdropColors> for RTDUpgradedGiftBackdropColorsBuilder {
  fn as_ref(&self) -> &UpgradedGiftBackdropColors {
    &self.inner
  }
}
