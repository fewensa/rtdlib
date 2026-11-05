use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes state of an auction
pub trait TDAuctionState: Debug + RObject {}

/// Describes state of an auction
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AuctionState {
  #[doc(hidden)]
  _Default(()),
  /// Contains information about an ongoing or scheduled auction
  Active(AuctionStateActive),
  /// Contains information about a finished auction
  Finished(AuctionStateFinished),
}

impl Default for AuctionState {
  fn default() -> Self {
    AuctionState::_Default(())
  }
}

impl<'de> Deserialize<'de> for AuctionState {
  fn deserialize<D>(deserializer: D) -> Result<AuctionState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AuctionState,
      (auctionStateActive, Active);
      (auctionStateFinished, Finished);

    )(deserializer)
  }
}

impl RObject for AuctionState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      AuctionState::Active(t) => t.td_name(),
      AuctionState::Finished(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      AuctionState::Active(t) => t.extra(),
      AuctionState::Finished(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AuctionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, AuctionState::_Default(_))
  }

  pub fn is_active(&self) -> bool {
    matches!(self, AuctionState::Active(_))
  }
  pub fn is_finished(&self) -> bool {
    matches!(self, AuctionState::Finished(_))
  }

  pub fn on_active<F: FnOnce(&AuctionStateActive)>(&self, fnc: F) -> &Self {
    if let AuctionState::Active(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_finished<F: FnOnce(&AuctionStateFinished)>(&self, fnc: F) -> &Self {
    if let AuctionState::Finished(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_active(&self) -> Option<&AuctionStateActive> {
    if let AuctionState::Active(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_finished(&self) -> Option<&AuctionStateFinished> {
    if let AuctionState::Finished(t) = self {
      return Some(t);
    }
    None
  }

  pub fn active<T: AsRef<AuctionStateActive>>(t: T) -> Self {
    AuctionState::Active(t.as_ref().clone())
  }

  pub fn finished<T: AsRef<AuctionStateFinished>>(t: T) -> Self {
    AuctionState::Finished(t.as_ref().clone())
  }
}

impl AsRef<AuctionState> for AuctionState {
  fn as_ref(&self) -> &AuctionState {
    self
  }
}

/// Contains information about an ongoing or scheduled auction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuctionStateActive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the auction started or will start
  start_date: i64,
  /// Point in time (Unix timestamp) when the auction will be ended
  end_date: i64,
  /// The minimum possible bid in the auction in Telegram Stars
  min_bid: i64,
  /// A sparse list of bids that were made in the auction
  bid_levels: Vec<AuctionBid>,
  /// User identifiers of at most 3 users with the biggest bids
  top_bidder_user_ids: Vec<i64>,
  /// Rounds of the auction in which their duration or extension rules are changed
  rounds: Vec<AuctionRound>,
  /// Point in time (Unix timestamp) when the current round will end
  current_round_end_date: i64,
  /// 1-based number of the current round
  current_round_number: i64,
  /// The total number of rounds
  total_round_count: i64,
  /// The number of items that were purchased on the auction by all users
  distributed_item_count: i64,
  /// The number of items that have to be distributed on the auction
  left_item_count: i64,
  /// The number of items that were purchased by the current user on the auction
  acquired_item_count: i64,
  /// Bid of the current user in the auction; may be null if none
  user_bid: Option<UserAuctionBid>,
}

impl RObject for AuctionStateActive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "auctionStateActive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuctionState for AuctionStateActive {}

impl AuctionStateActive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuctionStateActiveBuilder {
    let inner = AuctionStateActive {
      td_name: "auctionStateActive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuctionStateActiveBuilder { inner }
  }

  pub fn start_date(&self) -> i64 {
    self.start_date
  }

  pub fn end_date(&self) -> i64 {
    self.end_date
  }

  pub fn min_bid(&self) -> i64 {
    self.min_bid
  }

  pub fn bid_levels(&self) -> &Vec<AuctionBid> {
    &self.bid_levels
  }

  pub fn top_bidder_user_ids(&self) -> &Vec<i64> {
    &self.top_bidder_user_ids
  }

  pub fn rounds(&self) -> &Vec<AuctionRound> {
    &self.rounds
  }

  pub fn current_round_end_date(&self) -> i64 {
    self.current_round_end_date
  }

  pub fn current_round_number(&self) -> i64 {
    self.current_round_number
  }

  pub fn total_round_count(&self) -> i64 {
    self.total_round_count
  }

  pub fn distributed_item_count(&self) -> i64 {
    self.distributed_item_count
  }

  pub fn left_item_count(&self) -> i64 {
    self.left_item_count
  }

  pub fn acquired_item_count(&self) -> i64 {
    self.acquired_item_count
  }

  pub fn user_bid(&self) -> &Option<UserAuctionBid> {
    &self.user_bid
  }
}

#[doc(hidden)]
pub struct RTDAuctionStateActiveBuilder {
  inner: AuctionStateActive,
}

impl RTDAuctionStateActiveBuilder {
  pub fn build(&self) -> AuctionStateActive {
    self.inner.clone()
  }

  pub fn start_date(&mut self, start_date: i64) -> &mut Self {
    self.inner.start_date = start_date;
    self
  }

  pub fn end_date(&mut self, end_date: i64) -> &mut Self {
    self.inner.end_date = end_date;
    self
  }

  pub fn min_bid(&mut self, min_bid: i64) -> &mut Self {
    self.inner.min_bid = min_bid;
    self
  }

  pub fn bid_levels(&mut self, bid_levels: Vec<AuctionBid>) -> &mut Self {
    self.inner.bid_levels = bid_levels;
    self
  }

  pub fn top_bidder_user_ids(&mut self, top_bidder_user_ids: Vec<i64>) -> &mut Self {
    self.inner.top_bidder_user_ids = top_bidder_user_ids;
    self
  }

  pub fn rounds(&mut self, rounds: Vec<AuctionRound>) -> &mut Self {
    self.inner.rounds = rounds;
    self
  }

  pub fn current_round_end_date(&mut self, current_round_end_date: i64) -> &mut Self {
    self.inner.current_round_end_date = current_round_end_date;
    self
  }

  pub fn current_round_number(&mut self, current_round_number: i64) -> &mut Self {
    self.inner.current_round_number = current_round_number;
    self
  }

  pub fn total_round_count(&mut self, total_round_count: i64) -> &mut Self {
    self.inner.total_round_count = total_round_count;
    self
  }

  pub fn distributed_item_count(&mut self, distributed_item_count: i64) -> &mut Self {
    self.inner.distributed_item_count = distributed_item_count;
    self
  }

  pub fn left_item_count(&mut self, left_item_count: i64) -> &mut Self {
    self.inner.left_item_count = left_item_count;
    self
  }

  pub fn acquired_item_count(&mut self, acquired_item_count: i64) -> &mut Self {
    self.inner.acquired_item_count = acquired_item_count;
    self
  }

  pub fn user_bid<T: AsRef<UserAuctionBid>>(&mut self, user_bid: T) -> &mut Self {
    self.inner.user_bid = Some(user_bid.as_ref().clone());
    self
  }
}

impl AsRef<AuctionStateActive> for AuctionStateActive {
  fn as_ref(&self) -> &AuctionStateActive {
    self
  }
}

impl AsRef<AuctionStateActive> for RTDAuctionStateActiveBuilder {
  fn as_ref(&self) -> &AuctionStateActive {
    &self.inner
  }
}

/// Contains information about a finished auction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuctionStateFinished {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the auction started
  start_date: i64,
  /// Point in time (Unix timestamp) when the auction will be ended
  end_date: i64,
  /// Average price of bought items in Telegram Stars
  average_price: i64,
  /// The number of items that were purchased by the current user on the auction
  acquired_item_count: i64,
  /// Number of items from the auction being resold on Telegram
  telegram_listed_item_count: i64,
  /// Number of items from the auction being resold on Fragment
  fragment_listed_item_count: i64,
  /// The HTTPS link to the Fragment for the resold items; may be empty if there are no such items being sold on Fragment
  fragment_url: String,
}

impl RObject for AuctionStateFinished {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "auctionStateFinished"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuctionState for AuctionStateFinished {}

impl AuctionStateFinished {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuctionStateFinishedBuilder {
    let inner = AuctionStateFinished {
      td_name: "auctionStateFinished".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuctionStateFinishedBuilder { inner }
  }

  pub fn start_date(&self) -> i64 {
    self.start_date
  }

  pub fn end_date(&self) -> i64 {
    self.end_date
  }

  pub fn average_price(&self) -> i64 {
    self.average_price
  }

  pub fn acquired_item_count(&self) -> i64 {
    self.acquired_item_count
  }

  pub fn telegram_listed_item_count(&self) -> i64 {
    self.telegram_listed_item_count
  }

  pub fn fragment_listed_item_count(&self) -> i64 {
    self.fragment_listed_item_count
  }

  pub fn fragment_url(&self) -> &String {
    &self.fragment_url
  }
}

#[doc(hidden)]
pub struct RTDAuctionStateFinishedBuilder {
  inner: AuctionStateFinished,
}

impl RTDAuctionStateFinishedBuilder {
  pub fn build(&self) -> AuctionStateFinished {
    self.inner.clone()
  }

  pub fn start_date(&mut self, start_date: i64) -> &mut Self {
    self.inner.start_date = start_date;
    self
  }

  pub fn end_date(&mut self, end_date: i64) -> &mut Self {
    self.inner.end_date = end_date;
    self
  }

  pub fn average_price(&mut self, average_price: i64) -> &mut Self {
    self.inner.average_price = average_price;
    self
  }

  pub fn acquired_item_count(&mut self, acquired_item_count: i64) -> &mut Self {
    self.inner.acquired_item_count = acquired_item_count;
    self
  }

  pub fn telegram_listed_item_count(&mut self, telegram_listed_item_count: i64) -> &mut Self {
    self.inner.telegram_listed_item_count = telegram_listed_item_count;
    self
  }

  pub fn fragment_listed_item_count(&mut self, fragment_listed_item_count: i64) -> &mut Self {
    self.inner.fragment_listed_item_count = fragment_listed_item_count;
    self
  }

  pub fn fragment_url<T: AsRef<str>>(&mut self, fragment_url: T) -> &mut Self {
    self.inner.fragment_url = fragment_url.as_ref().to_string();
    self
  }
}

impl AsRef<AuctionStateFinished> for AuctionStateFinished {
  fn as_ref(&self) -> &AuctionStateFinished {
    self
  }
}

impl AsRef<AuctionStateFinished> for RTDAuctionStateFinishedBuilder {
  fn as_ref(&self) -> &AuctionStateFinished {
    &self.inner
  }
}
