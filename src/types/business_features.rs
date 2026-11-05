use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about features, available to Business user accounts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessFeatures {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of available business features
  features: Vec<BusinessFeature>,
}

impl RObject for BusinessFeatures {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessFeatures"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessFeatures {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessFeaturesBuilder {
    let inner = BusinessFeatures {
      td_name: "businessFeatures".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessFeaturesBuilder { inner }
  }

  pub fn features(&self) -> &Vec<BusinessFeature> {
    &self.features
  }
}

#[doc(hidden)]
pub struct RTDBusinessFeaturesBuilder {
  inner: BusinessFeatures,
}

impl RTDBusinessFeaturesBuilder {
  pub fn build(&self) -> BusinessFeatures {
    self.inner.clone()
  }

  pub fn features(&mut self, features: Vec<BusinessFeature>) -> &mut Self {
    self.inner.features = features;
    self
  }
}

impl AsRef<BusinessFeatures> for BusinessFeatures {
  fn as_ref(&self) -> &BusinessFeatures {
    self
  }
}

impl AsRef<BusinessFeatures> for RTDBusinessFeaturesBuilder {
  fn as_ref(&self) -> &BusinessFeatures {
    &self.inner
  }
}
