use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a backdrop of an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftBackdropCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The backdrop
  backdrop: UpgradedGiftBackdrop,
  /// Total number of gifts with the backdrop
  total_count: i64,
}

impl RObject for UpgradedGiftBackdropCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftBackdropCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftBackdropCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftBackdropCountBuilder {
    let inner = UpgradedGiftBackdropCount {
      td_name: "upgradedGiftBackdropCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftBackdropCountBuilder { inner }
  }

  pub fn backdrop(&self) -> &UpgradedGiftBackdrop {
    &self.backdrop
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftBackdropCountBuilder {
  inner: UpgradedGiftBackdropCount,
}

impl RTDUpgradedGiftBackdropCountBuilder {
  pub fn build(&self) -> UpgradedGiftBackdropCount {
    self.inner.clone()
  }

  pub fn backdrop<T: AsRef<UpgradedGiftBackdrop>>(&mut self, backdrop: T) -> &mut Self {
    self.inner.backdrop = backdrop.as_ref().clone();
    self
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }
}

impl AsRef<UpgradedGiftBackdropCount> for UpgradedGiftBackdropCount {
  fn as_ref(&self) -> &UpgradedGiftBackdropCount {
    self
  }
}

impl AsRef<UpgradedGiftBackdropCount> for RTDUpgradedGiftBackdropCountBuilder {
  fn as_ref(&self) -> &UpgradedGiftBackdropCount {
    &self.inner
  }
}
