use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of names of upgraded gift collections. The name of the collection of an upgraded gift is the part of upgradedGift.name before the last "-"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftCollectionNames {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The names
  names: Vec<String>,
}

impl RObject for UpgradedGiftCollectionNames {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftCollectionNames"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftCollectionNames {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftCollectionNamesBuilder {
    let inner = UpgradedGiftCollectionNames {
      td_name: "upgradedGiftCollectionNames".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftCollectionNamesBuilder { inner }
  }

  pub fn names(&self) -> &Vec<String> {
    &self.names
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftCollectionNamesBuilder {
  inner: UpgradedGiftCollectionNames,
}

impl RTDUpgradedGiftCollectionNamesBuilder {
  pub fn build(&self) -> UpgradedGiftCollectionNames {
    self.inner.clone()
  }

  pub fn names(&mut self, names: Vec<String>) -> &mut Self {
    self.inner.names = names;
    self
  }
}

impl AsRef<UpgradedGiftCollectionNames> for UpgradedGiftCollectionNames {
  fn as_ref(&self) -> &UpgradedGiftCollectionNames {
    self
  }
}

impl AsRef<UpgradedGiftCollectionNames> for RTDUpgradedGiftCollectionNamesBuilder {
  fn as_ref(&self) -> &UpgradedGiftCollectionNames {
    &self.inner
  }
}
