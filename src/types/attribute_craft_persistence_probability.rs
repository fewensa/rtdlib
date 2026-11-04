use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes chance of the crafted gift to have the backdrop or symbol of one of the original gifts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeCraftPersistenceProbability {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The 4 numbers that describe probability of the craft result to have the same attribute as one of the original gifts if 1, 2, 3, or 4 gifts with the attribute are used in the craft. Each number represents the number of crafted gifts with the original attribute per 1000 successful crafts
  persistence_chance_per_mille: Vec<i64>,
}

impl RObject for AttributeCraftPersistenceProbability {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "attributeCraftPersistenceProbability"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AttributeCraftPersistenceProbability {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAttributeCraftPersistenceProbabilityBuilder {
    let inner = AttributeCraftPersistenceProbability {
      td_name: "attributeCraftPersistenceProbability".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAttributeCraftPersistenceProbabilityBuilder { inner }
  }

  pub fn persistence_chance_per_mille(&self) -> &Vec<i64> {
    &self.persistence_chance_per_mille
  }
}

#[doc(hidden)]
pub struct RTDAttributeCraftPersistenceProbabilityBuilder {
  inner: AttributeCraftPersistenceProbability,
}

impl RTDAttributeCraftPersistenceProbabilityBuilder {
  pub fn build(&self) -> AttributeCraftPersistenceProbability {
    self.inner.clone()
  }

  pub fn persistence_chance_per_mille(
    &mut self,
    persistence_chance_per_mille: Vec<i64>,
  ) -> &mut Self {
    self.inner.persistence_chance_per_mille = persistence_chance_per_mille;
    self
  }
}

impl AsRef<AttributeCraftPersistenceProbability> for AttributeCraftPersistenceProbability {
  fn as_ref(&self) -> &AttributeCraftPersistenceProbability {
    self
  }
}

impl AsRef<AttributeCraftPersistenceProbability>
  for RTDAttributeCraftPersistenceProbabilityBuilder
{
  fn as_ref(&self) -> &AttributeCraftPersistenceProbability {
    &self.inner
  }
}
