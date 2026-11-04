use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes the maximum number of times that a specific gift can be purchased
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftPurchaseLimits {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The maximum number of times the gifts can be purchased
  total_count: i64,
  /// Number of remaining times the gift can be purchased
  remaining_count: i64,
}

impl RObject for GiftPurchaseLimits {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftPurchaseLimits"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftPurchaseLimits {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftPurchaseLimitsBuilder {
    let inner = GiftPurchaseLimits {
      td_name: "giftPurchaseLimits".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftPurchaseLimitsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn remaining_count(&self) -> i64 {
    self.remaining_count
  }
}

#[doc(hidden)]
pub struct RTDGiftPurchaseLimitsBuilder {
  inner: GiftPurchaseLimits,
}

impl RTDGiftPurchaseLimitsBuilder {
  pub fn build(&self) -> GiftPurchaseLimits {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn remaining_count(&mut self, remaining_count: i64) -> &mut Self {
    self.inner.remaining_count = remaining_count;
    self
  }
}

impl AsRef<GiftPurchaseLimits> for GiftPurchaseLimits {
  fn as_ref(&self) -> &GiftPurchaseLimits {
    self
  }
}

impl AsRef<GiftPurchaseLimits> for RTDGiftPurchaseLimitsBuilder {
  fn as_ref(&self) -> &GiftPurchaseLimits {
    &self.inner
  }
}
