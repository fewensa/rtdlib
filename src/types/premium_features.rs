use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about features, available to Premium users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatures {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of available features
  features: Vec<PremiumFeature>,
  /// The list of limits, increased for Premium users
  limits: Vec<PremiumLimit>,
  /// An internal link to be opened to pay for Telegram Premium if store payment isn't possible; may be null if direct payment isn't available
  payment_link: Option<InternalLinkType>,
}

impl RObject for PremiumFeatures {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatures"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumFeatures {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeaturesBuilder {
    let inner = PremiumFeatures {
      td_name: "premiumFeatures".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumFeaturesBuilder { inner }
  }

  pub fn features(&self) -> &Vec<PremiumFeature> {
    &self.features
  }

  pub fn limits(&self) -> &Vec<PremiumLimit> {
    &self.limits
  }

  pub fn payment_link(&self) -> &Option<InternalLinkType> {
    &self.payment_link
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeaturesBuilder {
  inner: PremiumFeatures,
}

impl RTDPremiumFeaturesBuilder {
  pub fn build(&self) -> PremiumFeatures {
    self.inner.clone()
  }

  pub fn features(&mut self, features: Vec<PremiumFeature>) -> &mut Self {
    self.inner.features = features;
    self
  }

  pub fn limits(&mut self, limits: Vec<PremiumLimit>) -> &mut Self {
    self.inner.limits = limits;
    self
  }

  pub fn payment_link<T: AsRef<InternalLinkType>>(&mut self, payment_link: T) -> &mut Self {
    self.inner.payment_link = Some(payment_link.as_ref().clone());
    self
  }
}

impl AsRef<PremiumFeatures> for PremiumFeatures {
  fn as_ref(&self) -> &PremiumFeatures {
    self
  }
}

impl AsRef<PremiumFeatures> for RTDPremiumFeaturesBuilder {
  fn as_ref(&self) -> &PremiumFeatures {
    &self.inner
  }
}
