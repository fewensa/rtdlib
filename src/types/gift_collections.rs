use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of gift collections
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftCollections {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of gift collections
  collections: Vec<GiftCollection>,
}

impl RObject for GiftCollections {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftCollections"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftCollections {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftCollectionsBuilder {
    let inner = GiftCollections {
      td_name: "giftCollections".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftCollectionsBuilder { inner }
  }

  pub fn collections(&self) -> &Vec<GiftCollection> {
    &self.collections
  }
}

#[doc(hidden)]
pub struct RTDGiftCollectionsBuilder {
  inner: GiftCollections,
}

impl RTDGiftCollectionsBuilder {
  pub fn build(&self) -> GiftCollections {
    self.inner.clone()
  }

  pub fn collections(&mut self, collections: Vec<GiftCollection>) -> &mut Self {
    self.inner.collections = collections;
    self
  }
}

impl AsRef<GiftCollections> for GiftCollections {
  fn as_ref(&self) -> &GiftCollections {
    self
  }
}

impl AsRef<GiftCollections> for RTDGiftCollectionsBuilder {
  fn as_ref(&self) -> &GiftCollections {
    &self.inner
  }
}
