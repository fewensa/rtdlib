use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a bid of the current user in an auction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserAuctionBid {
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
  /// The minimum number of Telegram Stars that can be put for the next bid
  next_bid_star_count: i64,
  /// Identifier of the user or the chat that will receive the auctioned item. If the auction is opened in context of another user or chat, then a warning is supposed to be shown to the current user
  owner_id: MessageSender,
  /// True, if the bid was returned to the user, because it was outbid and can't win anymore
  was_returned: bool,
}

impl RObject for UserAuctionBid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userAuctionBid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserAuctionBid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserAuctionBidBuilder {
    let inner = UserAuctionBid {
      td_name: "userAuctionBid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserAuctionBidBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn bid_date(&self) -> i64 {
    self.bid_date
  }

  pub fn next_bid_star_count(&self) -> i64 {
    self.next_bid_star_count
  }

  pub fn owner_id(&self) -> &MessageSender {
    &self.owner_id
  }

  pub fn was_returned(&self) -> bool {
    self.was_returned
  }
}

#[doc(hidden)]
pub struct RTDUserAuctionBidBuilder {
  inner: UserAuctionBid,
}

impl RTDUserAuctionBidBuilder {
  pub fn build(&self) -> UserAuctionBid {
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

  pub fn next_bid_star_count(&mut self, next_bid_star_count: i64) -> &mut Self {
    self.inner.next_bid_star_count = next_bid_star_count;
    self
  }

  pub fn owner_id<T: AsRef<MessageSender>>(&mut self, owner_id: T) -> &mut Self {
    self.inner.owner_id = owner_id.as_ref().clone();
    self
  }

  pub fn was_returned(&mut self, was_returned: bool) -> &mut Self {
    self.inner.was_returned = was_returned;
    self
  }
}

impl AsRef<UserAuctionBid> for UserAuctionBid {
  fn as_ref(&self) -> &UserAuctionBid {
    self
  }
}

impl AsRef<UserAuctionBid> for RTDUserAuctionBidBuilder {
  fn as_ref(&self) -> &UserAuctionBid {
    &self.inner
  }
}
