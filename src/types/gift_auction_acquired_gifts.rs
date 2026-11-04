use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of gifts that were acquired by the current user on an auction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftAuctionAcquiredGifts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of acquired gifts
  gifts: Vec<GiftAuctionAcquiredGift>,
}

impl RObject for GiftAuctionAcquiredGifts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftAuctionAcquiredGifts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftAuctionAcquiredGifts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftAuctionAcquiredGiftsBuilder {
    let inner = GiftAuctionAcquiredGifts {
      td_name: "giftAuctionAcquiredGifts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftAuctionAcquiredGiftsBuilder { inner }
  }

  pub fn gifts(&self) -> &Vec<GiftAuctionAcquiredGift> {
    &self.gifts
  }
}

#[doc(hidden)]
pub struct RTDGiftAuctionAcquiredGiftsBuilder {
  inner: GiftAuctionAcquiredGifts,
}

impl RTDGiftAuctionAcquiredGiftsBuilder {
  pub fn build(&self) -> GiftAuctionAcquiredGifts {
    self.inner.clone()
  }

  pub fn gifts(&mut self, gifts: Vec<GiftAuctionAcquiredGift>) -> &mut Self {
    self.inner.gifts = gifts;
    self
  }
}

impl AsRef<GiftAuctionAcquiredGifts> for GiftAuctionAcquiredGifts {
  fn as_ref(&self) -> &GiftAuctionAcquiredGifts {
    self
  }
}

impl AsRef<GiftAuctionAcquiredGifts> for RTDGiftAuctionAcquiredGiftsBuilder {
  fn as_ref(&self) -> &GiftAuctionAcquiredGifts {
    &self.inner
  }
}
