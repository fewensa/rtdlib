use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a model of an upgraded gift with the number of gifts found
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftModelCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The model
  model: UpgradedGiftModel,
  /// Total number of gifts with the model
  total_count: i64,
}

impl RObject for UpgradedGiftModelCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftModelCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftModelCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftModelCountBuilder {
    let inner = UpgradedGiftModelCount {
      td_name: "upgradedGiftModelCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftModelCountBuilder { inner }
  }

  pub fn model(&self) -> &UpgradedGiftModel {
    &self.model
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftModelCountBuilder {
  inner: UpgradedGiftModelCount,
}

impl RTDUpgradedGiftModelCountBuilder {
  pub fn build(&self) -> UpgradedGiftModelCount {
    self.inner.clone()
  }

  pub fn model<T: AsRef<UpgradedGiftModel>>(&mut self, model: T) -> &mut Self {
    self.inner.model = model.as_ref().clone();
    self
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }
}

impl AsRef<UpgradedGiftModelCount> for UpgradedGiftModelCount {
  fn as_ref(&self) -> &UpgradedGiftModelCount {
    self
  }
}

impl AsRef<UpgradedGiftModelCount> for RTDUpgradedGiftModelCountBuilder {
  fn as_ref(&self) -> &UpgradedGiftModelCount {
    &self.inner
  }
}
