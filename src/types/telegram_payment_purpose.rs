use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a purpose of a payment toward Telegram
pub trait TDTelegramPaymentPurpose: Debug + RObject {}

/// Describes a purpose of a payment toward Telegram
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TelegramPaymentPurpose {
  #[doc(hidden)]
  _Default(()),
  /// The user buying Telegram Stars for other users
  GiftedStars(TelegramPaymentPurposeGiftedStars),
  /// The user joins a chat and subscribes to regular payments in Telegram Stars
  JoinChat(TelegramPaymentPurposeJoinChat),
  /// The user gifting Telegram Premium to another user
  PremiumGift(TelegramPaymentPurposePremiumGift),
  /// The user boosting a chat by creating Telegram Premium gift codes for other users
  PremiumGiftCodes(TelegramPaymentPurposePremiumGiftCodes),
  /// The user creating a Telegram Premium giveaway
  PremiumGiveaway(TelegramPaymentPurposePremiumGiveaway),
  /// The user creating a Telegram Star giveaway
  StarGiveaway(TelegramPaymentPurposeStarGiveaway),
  /// The user buying Telegram Stars
  Stars(TelegramPaymentPurposeStars),
}

impl Default for TelegramPaymentPurpose {
  fn default() -> Self {
    TelegramPaymentPurpose::_Default(())
  }
}

impl<'de> Deserialize<'de> for TelegramPaymentPurpose {
  fn deserialize<D>(deserializer: D) -> Result<TelegramPaymentPurpose, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TelegramPaymentPurpose,
      (telegramPaymentPurposeGiftedStars, GiftedStars);
      (telegramPaymentPurposeJoinChat, JoinChat);
      (telegramPaymentPurposePremiumGift, PremiumGift);
      (telegramPaymentPurposePremiumGiftCodes, PremiumGiftCodes);
      (telegramPaymentPurposePremiumGiveaway, PremiumGiveaway);
      (telegramPaymentPurposeStarGiveaway, StarGiveaway);
      (telegramPaymentPurposeStars, Stars);

    )(deserializer)
  }
}

impl RObject for TelegramPaymentPurpose {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TelegramPaymentPurpose::GiftedStars(t) => t.td_name(),
      TelegramPaymentPurpose::JoinChat(t) => t.td_name(),
      TelegramPaymentPurpose::PremiumGift(t) => t.td_name(),
      TelegramPaymentPurpose::PremiumGiftCodes(t) => t.td_name(),
      TelegramPaymentPurpose::PremiumGiveaway(t) => t.td_name(),
      TelegramPaymentPurpose::StarGiveaway(t) => t.td_name(),
      TelegramPaymentPurpose::Stars(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TelegramPaymentPurpose::GiftedStars(t) => t.extra(),
      TelegramPaymentPurpose::JoinChat(t) => t.extra(),
      TelegramPaymentPurpose::PremiumGift(t) => t.extra(),
      TelegramPaymentPurpose::PremiumGiftCodes(t) => t.extra(),
      TelegramPaymentPurpose::PremiumGiveaway(t) => t.extra(),
      TelegramPaymentPurpose::StarGiveaway(t) => t.extra(),
      TelegramPaymentPurpose::Stars(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TelegramPaymentPurpose {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TelegramPaymentPurpose::_Default(_))
  }

  pub fn is_gifted_stars(&self) -> bool {
    matches!(self, TelegramPaymentPurpose::GiftedStars(_))
  }
  pub fn is_join_chat(&self) -> bool {
    matches!(self, TelegramPaymentPurpose::JoinChat(_))
  }
  pub fn is_premium_gift(&self) -> bool {
    matches!(self, TelegramPaymentPurpose::PremiumGift(_))
  }
  pub fn is_premium_gift_codes(&self) -> bool {
    matches!(self, TelegramPaymentPurpose::PremiumGiftCodes(_))
  }
  pub fn is_premium_giveaway(&self) -> bool {
    matches!(self, TelegramPaymentPurpose::PremiumGiveaway(_))
  }
  pub fn is_star_giveaway(&self) -> bool {
    matches!(self, TelegramPaymentPurpose::StarGiveaway(_))
  }
  pub fn is_stars(&self) -> bool {
    matches!(self, TelegramPaymentPurpose::Stars(_))
  }

  pub fn on_gifted_stars<F: FnOnce(&TelegramPaymentPurposeGiftedStars)>(&self, fnc: F) -> &Self {
    if let TelegramPaymentPurpose::GiftedStars(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_join_chat<F: FnOnce(&TelegramPaymentPurposeJoinChat)>(&self, fnc: F) -> &Self {
    if let TelegramPaymentPurpose::JoinChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_gift<F: FnOnce(&TelegramPaymentPurposePremiumGift)>(&self, fnc: F) -> &Self {
    if let TelegramPaymentPurpose::PremiumGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_gift_codes<F: FnOnce(&TelegramPaymentPurposePremiumGiftCodes)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TelegramPaymentPurpose::PremiumGiftCodes(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_giveaway<F: FnOnce(&TelegramPaymentPurposePremiumGiveaway)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TelegramPaymentPurpose::PremiumGiveaway(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_star_giveaway<F: FnOnce(&TelegramPaymentPurposeStarGiveaway)>(&self, fnc: F) -> &Self {
    if let TelegramPaymentPurpose::StarGiveaway(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stars<F: FnOnce(&TelegramPaymentPurposeStars)>(&self, fnc: F) -> &Self {
    if let TelegramPaymentPurpose::Stars(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_gifted_stars(&self) -> Option<&TelegramPaymentPurposeGiftedStars> {
    if let TelegramPaymentPurpose::GiftedStars(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_join_chat(&self) -> Option<&TelegramPaymentPurposeJoinChat> {
    if let TelegramPaymentPurpose::JoinChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_gift(&self) -> Option<&TelegramPaymentPurposePremiumGift> {
    if let TelegramPaymentPurpose::PremiumGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_gift_codes(&self) -> Option<&TelegramPaymentPurposePremiumGiftCodes> {
    if let TelegramPaymentPurpose::PremiumGiftCodes(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_giveaway(&self) -> Option<&TelegramPaymentPurposePremiumGiveaway> {
    if let TelegramPaymentPurpose::PremiumGiveaway(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_star_giveaway(&self) -> Option<&TelegramPaymentPurposeStarGiveaway> {
    if let TelegramPaymentPurpose::StarGiveaway(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stars(&self) -> Option<&TelegramPaymentPurposeStars> {
    if let TelegramPaymentPurpose::Stars(t) = self {
      return Some(t);
    }
    None
  }

  pub fn gifted_stars<T: AsRef<TelegramPaymentPurposeGiftedStars>>(t: T) -> Self {
    TelegramPaymentPurpose::GiftedStars(t.as_ref().clone())
  }

  pub fn join_chat<T: AsRef<TelegramPaymentPurposeJoinChat>>(t: T) -> Self {
    TelegramPaymentPurpose::JoinChat(t.as_ref().clone())
  }

  pub fn premium_gift<T: AsRef<TelegramPaymentPurposePremiumGift>>(t: T) -> Self {
    TelegramPaymentPurpose::PremiumGift(t.as_ref().clone())
  }

  pub fn premium_gift_codes<T: AsRef<TelegramPaymentPurposePremiumGiftCodes>>(t: T) -> Self {
    TelegramPaymentPurpose::PremiumGiftCodes(t.as_ref().clone())
  }

  pub fn premium_giveaway<T: AsRef<TelegramPaymentPurposePremiumGiveaway>>(t: T) -> Self {
    TelegramPaymentPurpose::PremiumGiveaway(t.as_ref().clone())
  }

  pub fn star_giveaway<T: AsRef<TelegramPaymentPurposeStarGiveaway>>(t: T) -> Self {
    TelegramPaymentPurpose::StarGiveaway(t.as_ref().clone())
  }

  pub fn stars<T: AsRef<TelegramPaymentPurposeStars>>(t: T) -> Self {
    TelegramPaymentPurpose::Stars(t.as_ref().clone())
  }
}

impl AsRef<TelegramPaymentPurpose> for TelegramPaymentPurpose {
  fn as_ref(&self) -> &TelegramPaymentPurpose {
    self
  }
}

/// The user buying Telegram Stars for other users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposeGiftedStars {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user to which Telegram Stars are gifted
  user_id: i64,
  /// ISO 4217 currency code of the payment currency
  currency: String,
  /// Paid amount, in the smallest units of the currency
  amount: i64,
  /// Number of bought Telegram Stars
  star_count: i64,
}

impl RObject for TelegramPaymentPurposeGiftedStars {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "telegramPaymentPurposeGiftedStars"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposeGiftedStars {}

impl TelegramPaymentPurposeGiftedStars {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTelegramPaymentPurposeGiftedStarsBuilder {
    let inner = TelegramPaymentPurposeGiftedStars {
      td_name: "telegramPaymentPurposeGiftedStars".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTelegramPaymentPurposeGiftedStarsBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDTelegramPaymentPurposeGiftedStarsBuilder {
  inner: TelegramPaymentPurposeGiftedStars,
}

impl RTDTelegramPaymentPurposeGiftedStarsBuilder {
  pub fn build(&self) -> TelegramPaymentPurposeGiftedStars {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<TelegramPaymentPurposeGiftedStars> for TelegramPaymentPurposeGiftedStars {
  fn as_ref(&self) -> &TelegramPaymentPurposeGiftedStars {
    self
  }
}

impl AsRef<TelegramPaymentPurposeGiftedStars> for RTDTelegramPaymentPurposeGiftedStarsBuilder {
  fn as_ref(&self) -> &TelegramPaymentPurposeGiftedStars {
    &self.inner
  }
}

/// The user joins a chat and subscribes to regular payments in Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposeJoinChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Invite link to use
  invite_link: String,
}

impl RObject for TelegramPaymentPurposeJoinChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "telegramPaymentPurposeJoinChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposeJoinChat {}

impl TelegramPaymentPurposeJoinChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTelegramPaymentPurposeJoinChatBuilder {
    let inner = TelegramPaymentPurposeJoinChat {
      td_name: "telegramPaymentPurposeJoinChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTelegramPaymentPurposeJoinChatBuilder { inner }
  }

  pub fn invite_link(&self) -> &String {
    &self.invite_link
  }
}

#[doc(hidden)]
pub struct RTDTelegramPaymentPurposeJoinChatBuilder {
  inner: TelegramPaymentPurposeJoinChat,
}

impl RTDTelegramPaymentPurposeJoinChatBuilder {
  pub fn build(&self) -> TelegramPaymentPurposeJoinChat {
    self.inner.clone()
  }

  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }
}

impl AsRef<TelegramPaymentPurposeJoinChat> for TelegramPaymentPurposeJoinChat {
  fn as_ref(&self) -> &TelegramPaymentPurposeJoinChat {
    self
  }
}

impl AsRef<TelegramPaymentPurposeJoinChat> for RTDTelegramPaymentPurposeJoinChatBuilder {
  fn as_ref(&self) -> &TelegramPaymentPurposeJoinChat {
    &self.inner
  }
}

/// The user gifting Telegram Premium to another user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposePremiumGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// ISO 4217 currency code of the payment currency, or "XTR" for payments in Telegram Stars
  currency: String,
  /// Paid amount, in the smallest units of the currency
  amount: i64,
  /// Identifier of the user who will receive Telegram Premium
  user_id: i64,
  /// Number of months the Telegram Premium subscription will be active for the user
  month_count: i64,
  /// Text to show to the user receiving Telegram Premium; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
  text: FormattedText,
}

impl RObject for TelegramPaymentPurposePremiumGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "telegramPaymentPurposePremiumGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposePremiumGift {}

impl TelegramPaymentPurposePremiumGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTelegramPaymentPurposePremiumGiftBuilder {
    let inner = TelegramPaymentPurposePremiumGift {
      td_name: "telegramPaymentPurposePremiumGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTelegramPaymentPurposePremiumGiftBuilder { inner }
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDTelegramPaymentPurposePremiumGiftBuilder {
  inner: TelegramPaymentPurposePremiumGift,
}

impl RTDTelegramPaymentPurposePremiumGiftBuilder {
  pub fn build(&self) -> TelegramPaymentPurposePremiumGift {
    self.inner.clone()
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<TelegramPaymentPurposePremiumGift> for TelegramPaymentPurposePremiumGift {
  fn as_ref(&self) -> &TelegramPaymentPurposePremiumGift {
    self
  }
}

impl AsRef<TelegramPaymentPurposePremiumGift> for RTDTelegramPaymentPurposePremiumGiftBuilder {
  fn as_ref(&self) -> &TelegramPaymentPurposePremiumGift {
    &self.inner
  }
}

/// The user boosting a chat by creating Telegram Premium gift codes for other users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposePremiumGiftCodes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the supergroup or channel chat, which will be automatically boosted by the users for duration of the Premium subscription and which is administered by the user
  boosted_chat_id: i64,
  /// ISO 4217 currency code of the payment currency
  currency: String,
  /// Paid amount, in the smallest units of the currency
  amount: i64,
  /// Identifiers of the users who can activate the gift codes
  user_ids: Vec<i64>,
  /// Number of months the Telegram Premium subscription will be active for the users
  month_count: i64,
  /// Text to show along with the gift codes; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
  text: FormattedText,
}

impl RObject for TelegramPaymentPurposePremiumGiftCodes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "telegramPaymentPurposePremiumGiftCodes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposePremiumGiftCodes {}

impl TelegramPaymentPurposePremiumGiftCodes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTelegramPaymentPurposePremiumGiftCodesBuilder {
    let inner = TelegramPaymentPurposePremiumGiftCodes {
      td_name: "telegramPaymentPurposePremiumGiftCodes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTelegramPaymentPurposePremiumGiftCodesBuilder { inner }
  }

  pub fn boosted_chat_id(&self) -> i64 {
    self.boosted_chat_id
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn user_ids(&self) -> &Vec<i64> {
    &self.user_ids
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDTelegramPaymentPurposePremiumGiftCodesBuilder {
  inner: TelegramPaymentPurposePremiumGiftCodes,
}

impl RTDTelegramPaymentPurposePremiumGiftCodesBuilder {
  pub fn build(&self) -> TelegramPaymentPurposePremiumGiftCodes {
    self.inner.clone()
  }

  pub fn boosted_chat_id(&mut self, boosted_chat_id: i64) -> &mut Self {
    self.inner.boosted_chat_id = boosted_chat_id;
    self
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<TelegramPaymentPurposePremiumGiftCodes> for TelegramPaymentPurposePremiumGiftCodes {
  fn as_ref(&self) -> &TelegramPaymentPurposePremiumGiftCodes {
    self
  }
}

impl AsRef<TelegramPaymentPurposePremiumGiftCodes>
  for RTDTelegramPaymentPurposePremiumGiftCodesBuilder
{
  fn as_ref(&self) -> &TelegramPaymentPurposePremiumGiftCodes {
    &self.inner
  }
}

/// The user creating a Telegram Premium giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposePremiumGiveaway {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Giveaway parameters
  parameters: GiveawayParameters,
  /// ISO 4217 currency code of the payment currency
  currency: String,
  /// Paid amount, in the smallest units of the currency
  amount: i64,
  /// Number of users who will be able to activate the gift codes
  winner_count: i64,
  /// Number of months the Telegram Premium subscription will be active for the users
  month_count: i64,
}

impl RObject for TelegramPaymentPurposePremiumGiveaway {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "telegramPaymentPurposePremiumGiveaway"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposePremiumGiveaway {}

impl TelegramPaymentPurposePremiumGiveaway {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTelegramPaymentPurposePremiumGiveawayBuilder {
    let inner = TelegramPaymentPurposePremiumGiveaway {
      td_name: "telegramPaymentPurposePremiumGiveaway".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTelegramPaymentPurposePremiumGiveawayBuilder { inner }
  }

  pub fn parameters(&self) -> &GiveawayParameters {
    &self.parameters
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn winner_count(&self) -> i64 {
    self.winner_count
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }
}

#[doc(hidden)]
pub struct RTDTelegramPaymentPurposePremiumGiveawayBuilder {
  inner: TelegramPaymentPurposePremiumGiveaway,
}

impl RTDTelegramPaymentPurposePremiumGiveawayBuilder {
  pub fn build(&self) -> TelegramPaymentPurposePremiumGiveaway {
    self.inner.clone()
  }

  pub fn parameters<T: AsRef<GiveawayParameters>>(&mut self, parameters: T) -> &mut Self {
    self.inner.parameters = parameters.as_ref().clone();
    self
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn winner_count(&mut self, winner_count: i64) -> &mut Self {
    self.inner.winner_count = winner_count;
    self
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }
}

impl AsRef<TelegramPaymentPurposePremiumGiveaway> for TelegramPaymentPurposePremiumGiveaway {
  fn as_ref(&self) -> &TelegramPaymentPurposePremiumGiveaway {
    self
  }
}

impl AsRef<TelegramPaymentPurposePremiumGiveaway>
  for RTDTelegramPaymentPurposePremiumGiveawayBuilder
{
  fn as_ref(&self) -> &TelegramPaymentPurposePremiumGiveaway {
    &self.inner
  }
}

/// The user creating a Telegram Star giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposeStarGiveaway {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Giveaway parameters
  parameters: GiveawayParameters,
  /// ISO 4217 currency code of the payment currency
  currency: String,
  /// Paid amount, in the smallest units of the currency
  amount: i64,
  /// The number of users to receive Telegram Stars
  winner_count: i64,
  /// The number of Telegram Stars to be distributed through the giveaway
  star_count: i64,
}

impl RObject for TelegramPaymentPurposeStarGiveaway {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "telegramPaymentPurposeStarGiveaway"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposeStarGiveaway {}

impl TelegramPaymentPurposeStarGiveaway {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTelegramPaymentPurposeStarGiveawayBuilder {
    let inner = TelegramPaymentPurposeStarGiveaway {
      td_name: "telegramPaymentPurposeStarGiveaway".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTelegramPaymentPurposeStarGiveawayBuilder { inner }
  }

  pub fn parameters(&self) -> &GiveawayParameters {
    &self.parameters
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn winner_count(&self) -> i64 {
    self.winner_count
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDTelegramPaymentPurposeStarGiveawayBuilder {
  inner: TelegramPaymentPurposeStarGiveaway,
}

impl RTDTelegramPaymentPurposeStarGiveawayBuilder {
  pub fn build(&self) -> TelegramPaymentPurposeStarGiveaway {
    self.inner.clone()
  }

  pub fn parameters<T: AsRef<GiveawayParameters>>(&mut self, parameters: T) -> &mut Self {
    self.inner.parameters = parameters.as_ref().clone();
    self
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn winner_count(&mut self, winner_count: i64) -> &mut Self {
    self.inner.winner_count = winner_count;
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<TelegramPaymentPurposeStarGiveaway> for TelegramPaymentPurposeStarGiveaway {
  fn as_ref(&self) -> &TelegramPaymentPurposeStarGiveaway {
    self
  }
}

impl AsRef<TelegramPaymentPurposeStarGiveaway> for RTDTelegramPaymentPurposeStarGiveawayBuilder {
  fn as_ref(&self) -> &TelegramPaymentPurposeStarGiveaway {
    &self.inner
  }
}

/// The user buying Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposeStars {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// ISO 4217 currency code of the payment currency
  currency: String,
  /// Paid amount, in the smallest units of the currency
  amount: i64,
  /// Number of bought Telegram Stars
  star_count: i64,
  /// Identifier of the chat that is supposed to receive the Telegram Stars; pass 0 if none. If non-zero and getOption("star_top_up_disabled"), then the payment isn't allowed and the user can't buy Telegram Stars in this place
  chat_id: i64,
}

impl RObject for TelegramPaymentPurposeStars {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "telegramPaymentPurposeStars"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposeStars {}

impl TelegramPaymentPurposeStars {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTelegramPaymentPurposeStarsBuilder {
    let inner = TelegramPaymentPurposeStars {
      td_name: "telegramPaymentPurposeStars".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTelegramPaymentPurposeStarsBuilder { inner }
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDTelegramPaymentPurposeStarsBuilder {
  inner: TelegramPaymentPurposeStars,
}

impl RTDTelegramPaymentPurposeStarsBuilder {
  pub fn build(&self) -> TelegramPaymentPurposeStars {
    self.inner.clone()
  }

  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<TelegramPaymentPurposeStars> for TelegramPaymentPurposeStars {
  fn as_ref(&self) -> &TelegramPaymentPurposeStars {
    self
  }
}

impl AsRef<TelegramPaymentPurposeStars> for RTDTelegramPaymentPurposeStarsBuilder {
  fn as_ref(&self) -> &TelegramPaymentPurposeStars {
    &self.inner
  }
}
