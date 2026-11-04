use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a bid in an auction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuctionBid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of Telegram Stars that were put in the bid
  star_count: i64,
  /// Point in time (Unix timestamp) when the bid was made
  bid_date: i64,
  /// Position of the bid in the list of all bids
  position: i64,
}

impl RObject for AuctionBid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "auctionBid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AuctionBid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuctionBidBuilder {
    let inner = AuctionBid {
      td_name: "auctionBid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuctionBidBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn bid_date(&self) -> i64 {
    self.bid_date
  }

  pub fn position(&self) -> i64 {
    self.position
  }
}

#[doc(hidden)]
pub struct RTDAuctionBidBuilder {
  inner: AuctionBid,
}

impl RTDAuctionBidBuilder {
  pub fn build(&self) -> AuctionBid {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn bid_date(&mut self, bid_date: i64) -> &mut Self {
    self.inner.bid_date = bid_date;
    self
  }

  pub fn position(&mut self, position: i64) -> &mut Self {
    self.inner.position = position;
    self
  }
}

impl AsRef<AuctionBid> for AuctionBid {
  fn as_ref(&self) -> &AuctionBid {
    self
  }
}

impl AsRef<AuctionBid> for RTDAuctionBidBuilder {
  fn as_ref(&self) -> &AuctionBid {
    &self.inner
  }
}
