use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes collection of gifts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftCollection {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the collection
  id: i64,
  /// Name of the collection
  name: String,
  /// Icon of the collection; may be null if none
  icon: Option<Sticker>,
  /// Total number of gifts in the collection
  gift_count: i64,
}

impl RObject for GiftCollection {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftCollection"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftCollection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftCollectionBuilder {
    let inner = GiftCollection {
      td_name: "giftCollection".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftCollectionBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn icon(&self) -> &Option<Sticker> {
    &self.icon
  }

  pub fn gift_count(&self) -> i64 {
    self.gift_count
  }
}

#[doc(hidden)]
pub struct RTDGiftCollectionBuilder {
  inner: GiftCollection,
}

impl RTDGiftCollectionBuilder {
  pub fn build(&self) -> GiftCollection {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn icon<T: AsRef<Sticker>>(&mut self, icon: T) -> &mut Self {
    self.inner.icon = Some(icon.as_ref().clone());
    self
  }

  pub fn gift_count(&mut self, gift_count: i64) -> &mut Self {
    self.inner.gift_count = gift_count;
    self
  }
}

impl AsRef<GiftCollection> for GiftCollection {
  fn as_ref(&self) -> &GiftCollection {
    self
  }
}

impl AsRef<GiftCollection> for RTDGiftCollectionBuilder {
  fn as_ref(&self) -> &GiftCollection {
    &self.inner
  }
}
