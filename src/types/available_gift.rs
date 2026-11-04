use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a gift that is available for purchase
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: Gift,
  /// Number of gifts that are available for resale
  resale_count: i64,
  /// The minimum price for the gifts available for resale in Telegram Star equivalent; 0 if there are no such gifts
  min_resale_star_count: i64,
  /// The title of the upgraded gift; empty if the gift isn't available for resale
  title: String,
}

impl RObject for AvailableGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "availableGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AvailableGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAvailableGiftBuilder {
    let inner = AvailableGift {
      td_name: "availableGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAvailableGiftBuilder { inner }
  }

  pub fn gift(&self) -> &Gift {
    &self.gift
  }

  pub fn resale_count(&self) -> i64 {
    self.resale_count
  }

  pub fn min_resale_star_count(&self) -> i64 {
    self.min_resale_star_count
  }

  pub fn title(&self) -> &String {
    &self.title
  }
}

#[doc(hidden)]
pub struct RTDAvailableGiftBuilder {
  inner: AvailableGift,
}

impl RTDAvailableGiftBuilder {
  pub fn build(&self) -> AvailableGift {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<Gift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn resale_count(&mut self, resale_count: i64) -> &mut Self {
    self.inner.resale_count = resale_count;
    self
  }

  pub fn min_resale_star_count(&mut self, min_resale_star_count: i64) -> &mut Self {
    self.inner.min_resale_star_count = min_resale_star_count;
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }
}

impl AsRef<AvailableGift> for AvailableGift {
  fn as_ref(&self) -> &AvailableGift {
    self
  }
}

impl AsRef<AvailableGift> for RTDAvailableGiftBuilder {
  fn as_ref(&self) -> &AvailableGift {
    &self.inner
  }
}
