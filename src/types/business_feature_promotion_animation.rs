use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a promotion animation for a Business feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeaturePromotionAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Business feature
  feature: BusinessFeature,
  /// Promotion animation for the feature
  animation: Animation,
}

impl RObject for BusinessFeaturePromotionAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeaturePromotionAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessFeaturePromotionAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeaturePromotionAnimationBuilder {
    let inner = BusinessFeaturePromotionAnimation {
      td_name: "businessFeaturePromotionAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessFeaturePromotionAnimationBuilder { inner }
  }

  pub fn feature(&self) -> &BusinessFeature {
    &self.feature
  }

  pub fn animation(&self) -> &Animation {
    &self.animation
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeaturePromotionAnimationBuilder {
  inner: BusinessFeaturePromotionAnimation,
}

impl RTDBusinessFeaturePromotionAnimationBuilder {
  pub fn build(&self) -> BusinessFeaturePromotionAnimation {
    self.inner.clone()
  }

  pub fn feature<T: AsRef<BusinessFeature>>(&mut self, feature: T) -> &mut Self {
    self.inner.feature = feature.as_ref().clone();
    self
  }

  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }
}

impl AsRef<BusinessFeaturePromotionAnimation> for BusinessFeaturePromotionAnimation {
  fn as_ref(&self) -> &BusinessFeaturePromotionAnimation {
    self
  }
}

impl AsRef<BusinessFeaturePromotionAnimation> for RTDBusinessFeaturePromotionAnimationBuilder {
  fn as_ref(&self) -> &BusinessFeaturePromotionAnimation {
    &self.inner
  }
}
