use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a purpose of an in-store payment
pub trait TDStorePaymentPurpose: Debug + RObject {}

/// Describes a purpose of an in-store payment
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StorePaymentPurpose {
  #[doc(hidden)]
  _Default(()),
  /// The user buying Telegram Stars for other users
  GiftedStars(StorePaymentPurposeGiftedStars),
  /// The user gifting Telegram Premium to another user
  PremiumGift(StorePaymentPurposePremiumGift),
  /// The user boosting a chat by creating Telegram Premium gift codes for other users
  PremiumGiftCodes(StorePaymentPurposePremiumGiftCodes),
  /// The user creating a Telegram Premium giveaway
  PremiumGiveaway(StorePaymentPurposePremiumGiveaway),
  /// The user subscribing to Telegram Premium
  PremiumSubscription(StorePaymentPurposePremiumSubscription),
  /// The user creating a Telegram Star giveaway
  StarGiveaway(StorePaymentPurposeStarGiveaway),
  /// The user buying Telegram Stars
  Stars(StorePaymentPurposeStars),
}

impl Default for StorePaymentPurpose {
  fn default() -> Self {
    StorePaymentPurpose::_Default(())
  }
}

impl<'de> Deserialize<'de> for StorePaymentPurpose {
  fn deserialize<D>(deserializer: D) -> Result<StorePaymentPurpose, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StorePaymentPurpose,
      (storePaymentPurposeGiftedStars, GiftedStars);
      (storePaymentPurposePremiumGift, PremiumGift);
      (storePaymentPurposePremiumGiftCodes, PremiumGiftCodes);
      (storePaymentPurposePremiumGiveaway, PremiumGiveaway);
      (storePaymentPurposePremiumSubscription, PremiumSubscription);
      (storePaymentPurposeStarGiveaway, StarGiveaway);
      (storePaymentPurposeStars, Stars);

    )(deserializer)
  }
}

impl RObject for StorePaymentPurpose {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StorePaymentPurpose::GiftedStars(t) => t.td_name(),
      StorePaymentPurpose::PremiumGift(t) => t.td_name(),
      StorePaymentPurpose::PremiumGiftCodes(t) => t.td_name(),
      StorePaymentPurpose::PremiumGiveaway(t) => t.td_name(),
      StorePaymentPurpose::PremiumSubscription(t) => t.td_name(),
      StorePaymentPurpose::StarGiveaway(t) => t.td_name(),
      StorePaymentPurpose::Stars(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StorePaymentPurpose::GiftedStars(t) => t.extra(),
      StorePaymentPurpose::PremiumGift(t) => t.extra(),
      StorePaymentPurpose::PremiumGiftCodes(t) => t.extra(),
      StorePaymentPurpose::PremiumGiveaway(t) => t.extra(),
      StorePaymentPurpose::PremiumSubscription(t) => t.extra(),
      StorePaymentPurpose::StarGiveaway(t) => t.extra(),
      StorePaymentPurpose::Stars(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StorePaymentPurpose {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StorePaymentPurpose::_Default(_))
  }

  pub fn is_gifted_stars(&self) -> bool {
    matches!(self, StorePaymentPurpose::GiftedStars(_))
  }
  pub fn is_premium_gift(&self) -> bool {
    matches!(self, StorePaymentPurpose::PremiumGift(_))
  }
  pub fn is_premium_gift_codes(&self) -> bool {
    matches!(self, StorePaymentPurpose::PremiumGiftCodes(_))
  }
  pub fn is_premium_giveaway(&self) -> bool {
    matches!(self, StorePaymentPurpose::PremiumGiveaway(_))
  }
  pub fn is_premium_subscription(&self) -> bool {
    matches!(self, StorePaymentPurpose::PremiumSubscription(_))
  }
  pub fn is_star_giveaway(&self) -> bool {
    matches!(self, StorePaymentPurpose::StarGiveaway(_))
  }
  pub fn is_stars(&self) -> bool {
    matches!(self, StorePaymentPurpose::Stars(_))
  }

  pub fn on_gifted_stars<F: FnOnce(&StorePaymentPurposeGiftedStars)>(&self, fnc: F) -> &Self {
    if let StorePaymentPurpose::GiftedStars(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_gift<F: FnOnce(&StorePaymentPurposePremiumGift)>(&self, fnc: F) -> &Self {
    if let StorePaymentPurpose::PremiumGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_gift_codes<F: FnOnce(&StorePaymentPurposePremiumGiftCodes)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StorePaymentPurpose::PremiumGiftCodes(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_giveaway<F: FnOnce(&StorePaymentPurposePremiumGiveaway)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StorePaymentPurpose::PremiumGiveaway(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_subscription<F: FnOnce(&StorePaymentPurposePremiumSubscription)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StorePaymentPurpose::PremiumSubscription(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_star_giveaway<F: FnOnce(&StorePaymentPurposeStarGiveaway)>(&self, fnc: F) -> &Self {
    if let StorePaymentPurpose::StarGiveaway(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stars<F: FnOnce(&StorePaymentPurposeStars)>(&self, fnc: F) -> &Self {
    if let StorePaymentPurpose::Stars(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_gifted_stars(&self) -> Option<&StorePaymentPurposeGiftedStars> {
    if let StorePaymentPurpose::GiftedStars(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_gift(&self) -> Option<&StorePaymentPurposePremiumGift> {
    if let StorePaymentPurpose::PremiumGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_gift_codes(&self) -> Option<&StorePaymentPurposePremiumGiftCodes> {
    if let StorePaymentPurpose::PremiumGiftCodes(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_giveaway(&self) -> Option<&StorePaymentPurposePremiumGiveaway> {
    if let StorePaymentPurpose::PremiumGiveaway(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_subscription(&self) -> Option<&StorePaymentPurposePremiumSubscription> {
    if let StorePaymentPurpose::PremiumSubscription(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_star_giveaway(&self) -> Option<&StorePaymentPurposeStarGiveaway> {
    if let StorePaymentPurpose::StarGiveaway(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stars(&self) -> Option<&StorePaymentPurposeStars> {
    if let StorePaymentPurpose::Stars(t) = self {
      return Some(t);
    }
    None
  }

  pub fn gifted_stars<T: AsRef<StorePaymentPurposeGiftedStars>>(t: T) -> Self {
    StorePaymentPurpose::GiftedStars(t.as_ref().clone())
  }

  pub fn premium_gift<T: AsRef<StorePaymentPurposePremiumGift>>(t: T) -> Self {
    StorePaymentPurpose::PremiumGift(t.as_ref().clone())
  }

  pub fn premium_gift_codes<T: AsRef<StorePaymentPurposePremiumGiftCodes>>(t: T) -> Self {
    StorePaymentPurpose::PremiumGiftCodes(t.as_ref().clone())
  }

  pub fn premium_giveaway<T: AsRef<StorePaymentPurposePremiumGiveaway>>(t: T) -> Self {
    StorePaymentPurpose::PremiumGiveaway(t.as_ref().clone())
  }

  pub fn premium_subscription<T: AsRef<StorePaymentPurposePremiumSubscription>>(t: T) -> Self {
    StorePaymentPurpose::PremiumSubscription(t.as_ref().clone())
  }

  pub fn star_giveaway<T: AsRef<StorePaymentPurposeStarGiveaway>>(t: T) -> Self {
    StorePaymentPurpose::StarGiveaway(t.as_ref().clone())
  }

  pub fn stars<T: AsRef<StorePaymentPurposeStars>>(t: T) -> Self {
    StorePaymentPurpose::Stars(t.as_ref().clone())
  }
}

impl AsRef<StorePaymentPurpose> for StorePaymentPurpose {
  fn as_ref(&self) -> &StorePaymentPurpose {
    self
  }
}

/// The user buying Telegram Stars for other users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposeGiftedStars {
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

impl RObject for StorePaymentPurposeGiftedStars {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storePaymentPurposeGiftedStars"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStorePaymentPurpose for StorePaymentPurposeGiftedStars {}

impl StorePaymentPurposeGiftedStars {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStorePaymentPurposeGiftedStarsBuilder {
    let inner = StorePaymentPurposeGiftedStars {
      td_name: "storePaymentPurposeGiftedStars".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStorePaymentPurposeGiftedStarsBuilder { inner }
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
pub struct RTDStorePaymentPurposeGiftedStarsBuilder {
  inner: StorePaymentPurposeGiftedStars,
}

impl RTDStorePaymentPurposeGiftedStarsBuilder {
  pub fn build(&self) -> StorePaymentPurposeGiftedStars {
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

impl AsRef<StorePaymentPurposeGiftedStars> for StorePaymentPurposeGiftedStars {
  fn as_ref(&self) -> &StorePaymentPurposeGiftedStars {
    self
  }
}

impl AsRef<StorePaymentPurposeGiftedStars> for RTDStorePaymentPurposeGiftedStarsBuilder {
  fn as_ref(&self) -> &StorePaymentPurposeGiftedStars {
    &self.inner
  }
}

/// The user gifting Telegram Premium to another user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposePremiumGift {
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
  /// Identifier of the user who will receive Telegram Premium
  user_id: i64,
  /// Text to show along with the gift codes; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
  text: FormattedText,
}

impl RObject for StorePaymentPurposePremiumGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storePaymentPurposePremiumGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStorePaymentPurpose for StorePaymentPurposePremiumGift {}

impl StorePaymentPurposePremiumGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStorePaymentPurposePremiumGiftBuilder {
    let inner = StorePaymentPurposePremiumGift {
      td_name: "storePaymentPurposePremiumGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStorePaymentPurposePremiumGiftBuilder { inner }
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

  pub fn text(&self) -> &FormattedText {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDStorePaymentPurposePremiumGiftBuilder {
  inner: StorePaymentPurposePremiumGift,
}

impl RTDStorePaymentPurposePremiumGiftBuilder {
  pub fn build(&self) -> StorePaymentPurposePremiumGift {
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

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<StorePaymentPurposePremiumGift> for StorePaymentPurposePremiumGift {
  fn as_ref(&self) -> &StorePaymentPurposePremiumGift {
    self
  }
}

impl AsRef<StorePaymentPurposePremiumGift> for RTDStorePaymentPurposePremiumGiftBuilder {
  fn as_ref(&self) -> &StorePaymentPurposePremiumGift {
    &self.inner
  }
}

/// The user boosting a chat by creating Telegram Premium gift codes for other users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposePremiumGiftCodes {
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
  /// Text to show along with the gift codes; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed
  text: FormattedText,
}

impl RObject for StorePaymentPurposePremiumGiftCodes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storePaymentPurposePremiumGiftCodes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStorePaymentPurpose for StorePaymentPurposePremiumGiftCodes {}

impl StorePaymentPurposePremiumGiftCodes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStorePaymentPurposePremiumGiftCodesBuilder {
    let inner = StorePaymentPurposePremiumGiftCodes {
      td_name: "storePaymentPurposePremiumGiftCodes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStorePaymentPurposePremiumGiftCodesBuilder { inner }
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

  pub fn text(&self) -> &FormattedText {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDStorePaymentPurposePremiumGiftCodesBuilder {
  inner: StorePaymentPurposePremiumGiftCodes,
}

impl RTDStorePaymentPurposePremiumGiftCodesBuilder {
  pub fn build(&self) -> StorePaymentPurposePremiumGiftCodes {
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

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<StorePaymentPurposePremiumGiftCodes> for StorePaymentPurposePremiumGiftCodes {
  fn as_ref(&self) -> &StorePaymentPurposePremiumGiftCodes {
    self
  }
}

impl AsRef<StorePaymentPurposePremiumGiftCodes> for RTDStorePaymentPurposePremiumGiftCodesBuilder {
  fn as_ref(&self) -> &StorePaymentPurposePremiumGiftCodes {
    &self.inner
  }
}

/// The user creating a Telegram Premium giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposePremiumGiveaway {
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
}

impl RObject for StorePaymentPurposePremiumGiveaway {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storePaymentPurposePremiumGiveaway"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStorePaymentPurpose for StorePaymentPurposePremiumGiveaway {}

impl StorePaymentPurposePremiumGiveaway {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStorePaymentPurposePremiumGiveawayBuilder {
    let inner = StorePaymentPurposePremiumGiveaway {
      td_name: "storePaymentPurposePremiumGiveaway".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStorePaymentPurposePremiumGiveawayBuilder { inner }
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
}

#[doc(hidden)]
pub struct RTDStorePaymentPurposePremiumGiveawayBuilder {
  inner: StorePaymentPurposePremiumGiveaway,
}

impl RTDStorePaymentPurposePremiumGiveawayBuilder {
  pub fn build(&self) -> StorePaymentPurposePremiumGiveaway {
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
}

impl AsRef<StorePaymentPurposePremiumGiveaway> for StorePaymentPurposePremiumGiveaway {
  fn as_ref(&self) -> &StorePaymentPurposePremiumGiveaway {
    self
  }
}

impl AsRef<StorePaymentPurposePremiumGiveaway> for RTDStorePaymentPurposePremiumGiveawayBuilder {
  fn as_ref(&self) -> &StorePaymentPurposePremiumGiveaway {
    &self.inner
  }
}

/// The user subscribing to Telegram Premium
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposePremiumSubscription {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true if this is a restore of a Telegram Premium purchase; only for App Store
  is_restore: bool,
  /// Pass true if this is an upgrade from a monthly subscription to early subscription; only for App Store
  is_upgrade: bool,
}

impl RObject for StorePaymentPurposePremiumSubscription {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storePaymentPurposePremiumSubscription"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStorePaymentPurpose for StorePaymentPurposePremiumSubscription {}

impl StorePaymentPurposePremiumSubscription {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStorePaymentPurposePremiumSubscriptionBuilder {
    let inner = StorePaymentPurposePremiumSubscription {
      td_name: "storePaymentPurposePremiumSubscription".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStorePaymentPurposePremiumSubscriptionBuilder { inner }
  }

  pub fn is_restore(&self) -> bool {
    self.is_restore
  }

  pub fn is_upgrade(&self) -> bool {
    self.is_upgrade
  }
}

#[doc(hidden)]
pub struct RTDStorePaymentPurposePremiumSubscriptionBuilder {
  inner: StorePaymentPurposePremiumSubscription,
}

impl RTDStorePaymentPurposePremiumSubscriptionBuilder {
  pub fn build(&self) -> StorePaymentPurposePremiumSubscription {
    self.inner.clone()
  }

  pub fn is_restore(&mut self, is_restore: bool) -> &mut Self {
    self.inner.is_restore = is_restore;
    self
  }

  pub fn is_upgrade(&mut self, is_upgrade: bool) -> &mut Self {
    self.inner.is_upgrade = is_upgrade;
    self
  }
}

impl AsRef<StorePaymentPurposePremiumSubscription> for StorePaymentPurposePremiumSubscription {
  fn as_ref(&self) -> &StorePaymentPurposePremiumSubscription {
    self
  }
}

impl AsRef<StorePaymentPurposePremiumSubscription>
  for RTDStorePaymentPurposePremiumSubscriptionBuilder
{
  fn as_ref(&self) -> &StorePaymentPurposePremiumSubscription {
    &self.inner
  }
}

/// The user creating a Telegram Star giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposeStarGiveaway {
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

impl RObject for StorePaymentPurposeStarGiveaway {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storePaymentPurposeStarGiveaway"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStorePaymentPurpose for StorePaymentPurposeStarGiveaway {}

impl StorePaymentPurposeStarGiveaway {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStorePaymentPurposeStarGiveawayBuilder {
    let inner = StorePaymentPurposeStarGiveaway {
      td_name: "storePaymentPurposeStarGiveaway".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStorePaymentPurposeStarGiveawayBuilder { inner }
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
pub struct RTDStorePaymentPurposeStarGiveawayBuilder {
  inner: StorePaymentPurposeStarGiveaway,
}

impl RTDStorePaymentPurposeStarGiveawayBuilder {
  pub fn build(&self) -> StorePaymentPurposeStarGiveaway {
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

impl AsRef<StorePaymentPurposeStarGiveaway> for StorePaymentPurposeStarGiveaway {
  fn as_ref(&self) -> &StorePaymentPurposeStarGiveaway {
    self
  }
}

impl AsRef<StorePaymentPurposeStarGiveaway> for RTDStorePaymentPurposeStarGiveawayBuilder {
  fn as_ref(&self) -> &StorePaymentPurposeStarGiveaway {
    &self.inner
  }
}

/// The user buying Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposeStars {
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
  /// Identifier of the chat that is supposed to receive the Telegram Stars; pass 0 if none
  chat_id: i64,
}

impl RObject for StorePaymentPurposeStars {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storePaymentPurposeStars"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStorePaymentPurpose for StorePaymentPurposeStars {}

impl StorePaymentPurposeStars {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStorePaymentPurposeStarsBuilder {
    let inner = StorePaymentPurposeStars {
      td_name: "storePaymentPurposeStars".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStorePaymentPurposeStarsBuilder { inner }
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
pub struct RTDStorePaymentPurposeStarsBuilder {
  inner: StorePaymentPurposeStars,
}

impl RTDStorePaymentPurposeStarsBuilder {
  pub fn build(&self) -> StorePaymentPurposeStars {
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

impl AsRef<StorePaymentPurposeStars> for StorePaymentPurposeStars {
  fn as_ref(&self) -> &StorePaymentPurposeStars {
    self
  }
}

impl AsRef<StorePaymentPurposeStars> for RTDStorePaymentPurposeStarsBuilder {
  fn as_ref(&self) -> &StorePaymentPurposeStars {
    &self.inner
  }
}
