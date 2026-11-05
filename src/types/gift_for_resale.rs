use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a gift available for resale
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftForResale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: UpgradedGift,
  /// Unique identifier of the received gift for the current user; only for the gifts owned by the current user
  received_gift_id: String,
}

impl RObject for GiftForResale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftForResale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftForResale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftForResaleBuilder {
    let inner = GiftForResale {
      td_name: "giftForResale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftForResaleBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }

  pub fn received_gift_id(&self) -> &String {
    &self.received_gift_id
  }
}

#[doc(hidden)]
pub struct RTDGiftForResaleBuilder {
  inner: GiftForResale,
}

impl RTDGiftForResaleBuilder {
  pub fn build(&self) -> GiftForResale {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn received_gift_id<T: AsRef<str>>(&mut self, received_gift_id: T) -> &mut Self {
    self.inner.received_gift_id = received_gift_id.as_ref().to_string();
    self
  }
}

impl AsRef<GiftForResale> for GiftForResale {
  fn as_ref(&self) -> &GiftForResale {
    self
  }
}

impl AsRef<GiftForResale> for RTDGiftForResaleBuilder {
  fn as_ref(&self) -> &GiftForResale {
    &self.inner
  }
}
