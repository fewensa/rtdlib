use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a gift that was acquired by the current user on an auction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftAuctionAcquiredGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Receiver of the gift
  receiver_id: MessageSender,
  /// Point in time (Unix timestamp) when the gift was acquired
  date: i64,
  /// The number of Telegram Stars that were paid for the gift
  star_count: i64,
  /// Identifier of the auction round in which the gift was acquired
  auction_round_number: i64,
  /// Position of the user in the round among all auction participants
  auction_round_position: i64,
  /// Unique number of the gift among gifts upgraded from the same gift after upgrade; 0 if yet unassigned
  unique_gift_number: i64,
  /// Message added to the gift
  text: FormattedText,
  /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
  is_private: bool,
}

impl RObject for GiftAuctionAcquiredGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftAuctionAcquiredGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftAuctionAcquiredGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftAuctionAcquiredGiftBuilder {
    let inner = GiftAuctionAcquiredGift {
      td_name: "giftAuctionAcquiredGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftAuctionAcquiredGiftBuilder { inner }
  }

  pub fn receiver_id(&self) -> &MessageSender {
    &self.receiver_id
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn auction_round_number(&self) -> i64 {
    self.auction_round_number
  }

  pub fn auction_round_position(&self) -> i64 {
    self.auction_round_position
  }

  pub fn unique_gift_number(&self) -> i64 {
    self.unique_gift_number
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn is_private(&self) -> bool {
    self.is_private
  }
}

#[doc(hidden)]
pub struct RTDGiftAuctionAcquiredGiftBuilder {
  inner: GiftAuctionAcquiredGift,
}

impl RTDGiftAuctionAcquiredGiftBuilder {
  pub fn build(&self) -> GiftAuctionAcquiredGift {
    self.inner.clone()
  }

  pub fn receiver_id<T: AsRef<MessageSender>>(&mut self, receiver_id: T) -> &mut Self {
    self.inner.receiver_id = receiver_id.as_ref().clone();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn auction_round_number(&mut self, auction_round_number: i64) -> &mut Self {
    self.inner.auction_round_number = auction_round_number;
    self
  }

  pub fn auction_round_position(&mut self, auction_round_position: i64) -> &mut Self {
    self.inner.auction_round_position = auction_round_position;
    self
  }

  pub fn unique_gift_number(&mut self, unique_gift_number: i64) -> &mut Self {
    self.inner.unique_gift_number = unique_gift_number;
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn is_private(&mut self, is_private: bool) -> &mut Self {
    self.inner.is_private = is_private;
    self
  }
}

impl AsRef<GiftAuctionAcquiredGift> for GiftAuctionAcquiredGift {
  fn as_ref(&self) -> &GiftAuctionAcquiredGift {
    self
  }
}

impl AsRef<GiftAuctionAcquiredGift> for RTDGiftAuctionAcquiredGiftBuilder {
  fn as_ref(&self) -> &GiftAuctionAcquiredGift {
    &self.inner
  }
}
