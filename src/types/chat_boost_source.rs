use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes source of a chat boost
pub trait TDChatBoostSource: Debug + RObject {}

/// Describes source of a chat boost
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatBoostSource {
  #[doc(hidden)]
  _Default(()),
  /// The chat created a Telegram Premium gift code for a user
  GiftCode(ChatBoostSourceGiftCode),
  /// The chat created a giveaway
  Giveaway(ChatBoostSourceGiveaway),
  /// A user with Telegram Premium subscription or gifted Telegram Premium boosted the chat
  Premium(ChatBoostSourcePremium),
}

impl Default for ChatBoostSource {
  fn default() -> Self {
    ChatBoostSource::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatBoostSource {
  fn deserialize<D>(deserializer: D) -> Result<ChatBoostSource, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatBoostSource,
      (chatBoostSourceGiftCode, GiftCode);
      (chatBoostSourceGiveaway, Giveaway);
      (chatBoostSourcePremium, Premium);

    )(deserializer)
  }
}

impl RObject for ChatBoostSource {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatBoostSource::GiftCode(t) => t.td_name(),
      ChatBoostSource::Giveaway(t) => t.td_name(),
      ChatBoostSource::Premium(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatBoostSource::GiftCode(t) => t.extra(),
      ChatBoostSource::Giveaway(t) => t.extra(),
      ChatBoostSource::Premium(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoostSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatBoostSource::_Default(_))
  }

  pub fn is_gift_code(&self) -> bool {
    matches!(self, ChatBoostSource::GiftCode(_))
  }
  pub fn is_giveaway(&self) -> bool {
    matches!(self, ChatBoostSource::Giveaway(_))
  }
  pub fn is_premium(&self) -> bool {
    matches!(self, ChatBoostSource::Premium(_))
  }

  pub fn on_gift_code<F: FnOnce(&ChatBoostSourceGiftCode)>(&self, fnc: F) -> &Self {
    if let ChatBoostSource::GiftCode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_giveaway<F: FnOnce(&ChatBoostSourceGiveaway)>(&self, fnc: F) -> &Self {
    if let ChatBoostSource::Giveaway(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium<F: FnOnce(&ChatBoostSourcePremium)>(&self, fnc: F) -> &Self {
    if let ChatBoostSource::Premium(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_gift_code(&self) -> Option<&ChatBoostSourceGiftCode> {
    if let ChatBoostSource::GiftCode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_giveaway(&self) -> Option<&ChatBoostSourceGiveaway> {
    if let ChatBoostSource::Giveaway(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium(&self) -> Option<&ChatBoostSourcePremium> {
    if let ChatBoostSource::Premium(t) = self {
      return Some(t);
    }
    None
  }

  pub fn gift_code<T: AsRef<ChatBoostSourceGiftCode>>(t: T) -> Self {
    ChatBoostSource::GiftCode(t.as_ref().clone())
  }

  pub fn giveaway<T: AsRef<ChatBoostSourceGiveaway>>(t: T) -> Self {
    ChatBoostSource::Giveaway(t.as_ref().clone())
  }

  pub fn premium<T: AsRef<ChatBoostSourcePremium>>(t: T) -> Self {
    ChatBoostSource::Premium(t.as_ref().clone())
  }
}

impl AsRef<ChatBoostSource> for ChatBoostSource {
  fn as_ref(&self) -> &ChatBoostSource {
    self
  }
}

/// The chat created a Telegram Premium gift code for a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSourceGiftCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a user, for which the gift code was created
  user_id: i64,
  /// The created Telegram Premium gift code, which is known only if this is a gift code for the current user, or it has already been claimed
  gift_code: String,
}

impl RObject for ChatBoostSourceGiftCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostSourceGiftCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatBoostSource for ChatBoostSourceGiftCode {}

impl ChatBoostSourceGiftCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostSourceGiftCodeBuilder {
    let inner = ChatBoostSourceGiftCode {
      td_name: "chatBoostSourceGiftCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostSourceGiftCodeBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn gift_code(&self) -> &String {
    &self.gift_code
  }
}

#[doc(hidden)]
pub struct RTDChatBoostSourceGiftCodeBuilder {
  inner: ChatBoostSourceGiftCode,
}

impl RTDChatBoostSourceGiftCodeBuilder {
  pub fn build(&self) -> ChatBoostSourceGiftCode {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn gift_code<T: AsRef<str>>(&mut self, gift_code: T) -> &mut Self {
    self.inner.gift_code = gift_code.as_ref().to_string();
    self
  }
}

impl AsRef<ChatBoostSourceGiftCode> for ChatBoostSourceGiftCode {
  fn as_ref(&self) -> &ChatBoostSourceGiftCode {
    self
  }
}

impl AsRef<ChatBoostSourceGiftCode> for RTDChatBoostSourceGiftCodeBuilder {
  fn as_ref(&self) -> &ChatBoostSourceGiftCode {
    &self.inner
  }
}

/// The chat created a giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSourceGiveaway {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a user who won in the giveaway; 0 if none
  user_id: i64,
  /// The created Telegram Premium gift code if it was used by the user or can be claimed by the current user; an empty string otherwise; for Telegram Premium giveaways only
  gift_code: String,
  /// Number of Telegram Stars distributed among winners of the giveaway
  star_count: i64,
  /// Identifier of the corresponding giveaway message; can be an identifier of a deleted message
  giveaway_message_id: i64,
  /// True, if the winner for the corresponding giveaway prize wasn't chosen, because there were not enough participants
  is_unclaimed: bool,
}

impl RObject for ChatBoostSourceGiveaway {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostSourceGiveaway"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatBoostSource for ChatBoostSourceGiveaway {}

impl ChatBoostSourceGiveaway {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostSourceGiveawayBuilder {
    let inner = ChatBoostSourceGiveaway {
      td_name: "chatBoostSourceGiveaway".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostSourceGiveawayBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn gift_code(&self) -> &String {
    &self.gift_code
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn giveaway_message_id(&self) -> i64 {
    self.giveaway_message_id
  }

  pub fn is_unclaimed(&self) -> bool {
    self.is_unclaimed
  }
}

#[doc(hidden)]
pub struct RTDChatBoostSourceGiveawayBuilder {
  inner: ChatBoostSourceGiveaway,
}

impl RTDChatBoostSourceGiveawayBuilder {
  pub fn build(&self) -> ChatBoostSourceGiveaway {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn gift_code<T: AsRef<str>>(&mut self, gift_code: T) -> &mut Self {
    self.inner.gift_code = gift_code.as_ref().to_string();
    self
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn giveaway_message_id(&mut self, giveaway_message_id: i64) -> &mut Self {
    self.inner.giveaway_message_id = giveaway_message_id;
    self
  }

  pub fn is_unclaimed(&mut self, is_unclaimed: bool) -> &mut Self {
    self.inner.is_unclaimed = is_unclaimed;
    self
  }
}

impl AsRef<ChatBoostSourceGiveaway> for ChatBoostSourceGiveaway {
  fn as_ref(&self) -> &ChatBoostSourceGiveaway {
    self
  }
}

impl AsRef<ChatBoostSourceGiveaway> for RTDChatBoostSourceGiveawayBuilder {
  fn as_ref(&self) -> &ChatBoostSourceGiveaway {
    &self.inner
  }
}

/// A user with Telegram Premium subscription or gifted Telegram Premium boosted the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSourcePremium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user
  user_id: i64,
}

impl RObject for ChatBoostSourcePremium {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostSourcePremium"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatBoostSource for ChatBoostSourcePremium {}

impl ChatBoostSourcePremium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostSourcePremiumBuilder {
    let inner = ChatBoostSourcePremium {
      td_name: "chatBoostSourcePremium".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostSourcePremiumBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}

#[doc(hidden)]
pub struct RTDChatBoostSourcePremiumBuilder {
  inner: ChatBoostSourcePremium,
}

impl RTDChatBoostSourcePremiumBuilder {
  pub fn build(&self) -> ChatBoostSourcePremium {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }
}

impl AsRef<ChatBoostSourcePremium> for ChatBoostSourcePremium {
  fn as_ref(&self) -> &ChatBoostSourcePremium {
    self
  }
}

impl AsRef<ChatBoostSourcePremium> for RTDChatBoostSourcePremiumBuilder {
  fn as_ref(&self) -> &ChatBoostSourcePremium {
    &self.inner
  }
}
