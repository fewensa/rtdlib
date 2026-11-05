use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents auction state of a gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftAuctionState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: Gift,
  /// Auction state of the gift
  state: AuctionState,
}

impl RObject for GiftAuctionState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftAuctionState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftAuctionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftAuctionStateBuilder {
    let inner = GiftAuctionState {
      td_name: "giftAuctionState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftAuctionStateBuilder { inner }
  }

  pub fn gift(&self) -> &Gift {
    &self.gift
  }

  pub fn state(&self) -> &AuctionState {
    &self.state
  }
}

#[doc(hidden)]
pub struct RTDGiftAuctionStateBuilder {
  inner: GiftAuctionState,
}

impl RTDGiftAuctionStateBuilder {
  pub fn build(&self) -> GiftAuctionState {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<Gift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn state<T: AsRef<AuctionState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }
}

impl AsRef<GiftAuctionState> for GiftAuctionState {
  fn as_ref(&self) -> &GiftAuctionState {
    self
  }
}

impl AsRef<GiftAuctionState> for RTDGiftAuctionStateBuilder {
  fn as_ref(&self) -> &GiftAuctionState {
    &self.inner
  }
}
