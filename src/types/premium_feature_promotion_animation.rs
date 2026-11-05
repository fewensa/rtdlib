use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a promotion animation for a Premium feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeaturePromotionAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Premium feature
  feature: PremiumFeature,
  /// Promotion animation for the feature
  animation: Animation,
}

impl RObject for PremiumFeaturePromotionAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeaturePromotionAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumFeaturePromotionAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeaturePromotionAnimationBuilder {
    let inner = PremiumFeaturePromotionAnimation {
      td_name: "premiumFeaturePromotionAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumFeaturePromotionAnimationBuilder { inner }
  }

  pub fn feature(&self) -> &PremiumFeature {
    &self.feature
  }

  pub fn animation(&self) -> &Animation {
    &self.animation
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeaturePromotionAnimationBuilder {
  inner: PremiumFeaturePromotionAnimation,
}

impl RTDPremiumFeaturePromotionAnimationBuilder {
  pub fn build(&self) -> PremiumFeaturePromotionAnimation {
    self.inner.clone()
  }

  pub fn feature<T: AsRef<PremiumFeature>>(&mut self, feature: T) -> &mut Self {
    self.inner.feature = feature.as_ref().clone();
    self
  }

  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }
}

impl AsRef<PremiumFeaturePromotionAnimation> for PremiumFeaturePromotionAnimation {
  fn as_ref(&self) -> &PremiumFeaturePromotionAnimation {
    self
  }
}

impl AsRef<PremiumFeaturePromotionAnimation> for RTDPremiumFeaturePromotionAnimationBuilder {
  fn as_ref(&self) -> &PremiumFeaturePromotionAnimation {
    &self.inner
  }
}
