use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of gifts received by a user or a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftsForCrafting {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The total number of received gifts
  total_count: i64,
  /// The list of gifts
  gifts: Vec<ReceivedGift>,
  /// The 4 objects that describe probabilities of the crafted gift to have the backdrop or symbol of one of the original gifts for the cases when 1-4 gifts are used in the craft correspondingly
  attribute_persistence_probabilities: Vec<AttributeCraftPersistenceProbability>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for GiftsForCrafting {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftsForCrafting"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftsForCrafting {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftsForCraftingBuilder {
    let inner = GiftsForCrafting {
      td_name: "giftsForCrafting".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftsForCraftingBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn gifts(&self) -> &Vec<ReceivedGift> {
    &self.gifts
  }

  pub fn attribute_persistence_probabilities(&self) -> &Vec<AttributeCraftPersistenceProbability> {
    &self.attribute_persistence_probabilities
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDGiftsForCraftingBuilder {
  inner: GiftsForCrafting,
}

impl RTDGiftsForCraftingBuilder {
  pub fn build(&self) -> GiftsForCrafting {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn gifts(&mut self, gifts: Vec<ReceivedGift>) -> &mut Self {
    self.inner.gifts = gifts;
    self
  }

  pub fn attribute_persistence_probabilities(
    &mut self,
    attribute_persistence_probabilities: Vec<AttributeCraftPersistenceProbability>,
  ) -> &mut Self {
    self.inner.attribute_persistence_probabilities = attribute_persistence_probabilities;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<GiftsForCrafting> for GiftsForCrafting {
  fn as_ref(&self) -> &GiftsForCrafting {
    self
  }
}

impl AsRef<GiftsForCrafting> for RTDGiftsForCraftingBuilder {
  fn as_ref(&self) -> &GiftsForCrafting {
    &self.inner
  }
}
