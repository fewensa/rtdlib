use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a gift that can be sent to another user or channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Gift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the gift
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Identifier of the chat that published the gift; 0 if none
  publisher_chat_id: i64,
  /// The sticker representing the gift
  sticker: Sticker,
  /// Number of Telegram Stars that must be paid for the gift
  star_count: i64,
  /// Number of Telegram Stars that can be claimed by the receiver instead of the regular gift by default. If the gift was paid with just bought Telegram Stars, then full value can be claimed
  default_sell_star_count: i64,
  /// Number of Telegram Stars that must be paid to upgrade the gift; 0 if upgrade isn't possible
  upgrade_star_count: i64,
  /// Number of unique gift variants that are available for the upgraded gift; 0 if unknown
  upgrade_variant_count: i64,
  /// True, if the gift can be used to customize the user's name, and backgrounds of profile photo, reply header, and link preview
  has_colors: bool,
  /// True, if the gift is a birthday gift
  is_for_birthday: bool,
  /// True, if the gift can be bought only by Telegram Premium subscribers
  is_premium: bool,
  /// Information about the auction on which the gift can be purchased; may be null if the gift can be purchased directly
  auction_info: Option<GiftAuction>,
  /// Point in time (Unix timestamp) when the gift can be sent next time by the current user; may be 0 or a date in the past. If the date is in the future, then call canSendGift to get the reason, why the gift can't be sent now
  next_send_date: i64,
  /// Number of times the gift can be purchased by the current user; may be null if not limited
  user_limits: Option<GiftPurchaseLimits>,
  /// Number of times the gift can be purchased by all users; may be null if not limited
  overall_limits: Option<GiftPurchaseLimits>,
  /// Background of the gift; may be null if the gift has no background
  background: Option<GiftBackground>,
  /// Point in time (Unix timestamp) when the gift was sent for the first time; for sold out gifts only
  first_send_date: i64,
  /// Point in time (Unix timestamp) when the gift was sent for the last time; for sold out gifts only
  last_send_date: i64,
}

impl RObject for Gift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "gift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Gift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftBuilder {
    let inner = Gift {
      td_name: "gift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn publisher_chat_id(&self) -> i64 {
    self.publisher_chat_id
  }

  pub fn sticker(&self) -> &Sticker {
    &self.sticker
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn default_sell_star_count(&self) -> i64 {
    self.default_sell_star_count
  }

  pub fn upgrade_star_count(&self) -> i64 {
    self.upgrade_star_count
  }

  pub fn upgrade_variant_count(&self) -> i64 {
    self.upgrade_variant_count
  }

  pub fn has_colors(&self) -> bool {
    self.has_colors
  }

  pub fn is_for_birthday(&self) -> bool {
    self.is_for_birthday
  }

  pub fn is_premium(&self) -> bool {
    self.is_premium
  }

  pub fn auction_info(&self) -> &Option<GiftAuction> {
    &self.auction_info
  }

  pub fn next_send_date(&self) -> i64 {
    self.next_send_date
  }

  pub fn user_limits(&self) -> &Option<GiftPurchaseLimits> {
    &self.user_limits
  }

  pub fn overall_limits(&self) -> &Option<GiftPurchaseLimits> {
    &self.overall_limits
  }

  pub fn background(&self) -> &Option<GiftBackground> {
    &self.background
  }

  pub fn first_send_date(&self) -> i64 {
    self.first_send_date
  }

  pub fn last_send_date(&self) -> i64 {
    self.last_send_date
  }
}

#[doc(hidden)]
pub struct RTDGiftBuilder {
  inner: Gift,
}

impl RTDGiftBuilder {
  pub fn build(&self) -> Gift {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn publisher_chat_id(&mut self, publisher_chat_id: i64) -> &mut Self {
    self.inner.publisher_chat_id = publisher_chat_id;
    self
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn default_sell_star_count(&mut self, default_sell_star_count: i64) -> &mut Self {
    self.inner.default_sell_star_count = default_sell_star_count;
    self
  }

  pub fn upgrade_star_count(&mut self, upgrade_star_count: i64) -> &mut Self {
    self.inner.upgrade_star_count = upgrade_star_count;
    self
  }

  pub fn upgrade_variant_count(&mut self, upgrade_variant_count: i64) -> &mut Self {
    self.inner.upgrade_variant_count = upgrade_variant_count;
    self
  }

  pub fn has_colors(&mut self, has_colors: bool) -> &mut Self {
    self.inner.has_colors = has_colors;
    self
  }

  pub fn is_for_birthday(&mut self, is_for_birthday: bool) -> &mut Self {
    self.inner.is_for_birthday = is_for_birthday;
    self
  }

  pub fn is_premium(&mut self, is_premium: bool) -> &mut Self {
    self.inner.is_premium = is_premium;
    self
  }

  pub fn auction_info<T: AsRef<GiftAuction>>(&mut self, auction_info: T) -> &mut Self {
    self.inner.auction_info = Some(auction_info.as_ref().clone());
    self
  }

  pub fn next_send_date(&mut self, next_send_date: i64) -> &mut Self {
    self.inner.next_send_date = next_send_date;
    self
  }

  pub fn user_limits<T: AsRef<GiftPurchaseLimits>>(&mut self, user_limits: T) -> &mut Self {
    self.inner.user_limits = Some(user_limits.as_ref().clone());
    self
  }

  pub fn overall_limits<T: AsRef<GiftPurchaseLimits>>(&mut self, overall_limits: T) -> &mut Self {
    self.inner.overall_limits = Some(overall_limits.as_ref().clone());
    self
  }

  pub fn background<T: AsRef<GiftBackground>>(&mut self, background: T) -> &mut Self {
    self.inner.background = Some(background.as_ref().clone());
    self
  }

  pub fn first_send_date(&mut self, first_send_date: i64) -> &mut Self {
    self.inner.first_send_date = first_send_date;
    self
  }

  pub fn last_send_date(&mut self, last_send_date: i64) -> &mut Self {
    self.inner.last_send_date = last_send_date;
    self
  }
}

impl AsRef<Gift> for Gift {
  fn as_ref(&self) -> &Gift {
    self
  }
}

impl AsRef<Gift> for RTDGiftBuilder {
  fn as_ref(&self) -> &Gift {
    &self.inner
  }
}
