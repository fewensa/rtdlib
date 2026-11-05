use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of subscription paid in Telegram Stars
pub trait TDStarSubscriptionType: Debug + RObject {}

/// Describes type of subscription paid in Telegram Stars
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StarSubscriptionType {
  #[doc(hidden)]
  _Default(()),
  /// Describes a subscription in a bot or a business account
  Bot(StarSubscriptionTypeBot),
  /// Describes a subscription to a channel chat
  Channel(StarSubscriptionTypeChannel),
}

impl Default for StarSubscriptionType {
  fn default() -> Self {
    StarSubscriptionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for StarSubscriptionType {
  fn deserialize<D>(deserializer: D) -> Result<StarSubscriptionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StarSubscriptionType,
      (starSubscriptionTypeBot, Bot);
      (starSubscriptionTypeChannel, Channel);

    )(deserializer)
  }
}

impl RObject for StarSubscriptionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StarSubscriptionType::Bot(t) => t.td_name(),
      StarSubscriptionType::Channel(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StarSubscriptionType::Bot(t) => t.extra(),
      StarSubscriptionType::Channel(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarSubscriptionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StarSubscriptionType::_Default(_))
  }

  pub fn is_bot(&self) -> bool {
    matches!(self, StarSubscriptionType::Bot(_))
  }
  pub fn is_channel(&self) -> bool {
    matches!(self, StarSubscriptionType::Channel(_))
  }

  pub fn on_bot<F: FnOnce(&StarSubscriptionTypeBot)>(&self, fnc: F) -> &Self {
    if let StarSubscriptionType::Bot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel<F: FnOnce(&StarSubscriptionTypeChannel)>(&self, fnc: F) -> &Self {
    if let StarSubscriptionType::Channel(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_bot(&self) -> Option<&StarSubscriptionTypeBot> {
    if let StarSubscriptionType::Bot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel(&self) -> Option<&StarSubscriptionTypeChannel> {
    if let StarSubscriptionType::Channel(t) = self {
      return Some(t);
    }
    None
  }

  pub fn bot<T: AsRef<StarSubscriptionTypeBot>>(t: T) -> Self {
    StarSubscriptionType::Bot(t.as_ref().clone())
  }

  pub fn channel<T: AsRef<StarSubscriptionTypeChannel>>(t: T) -> Self {
    StarSubscriptionType::Channel(t.as_ref().clone())
  }
}

impl AsRef<StarSubscriptionType> for StarSubscriptionType {
  fn as_ref(&self) -> &StarSubscriptionType {
    self
  }
}

/// Describes a subscription in a bot or a business account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarSubscriptionTypeBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the subscription was canceled by the bot and can't be extended
  is_canceled_by_bot: bool,
  /// Subscription invoice title
  title: String,
  /// Subscription invoice photo
  photo: Photo,
  /// The link to the subscription invoice
  invoice_link: String,
}

impl RObject for StarSubscriptionTypeBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starSubscriptionTypeBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarSubscriptionType for StarSubscriptionTypeBot {}

impl StarSubscriptionTypeBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarSubscriptionTypeBotBuilder {
    let inner = StarSubscriptionTypeBot {
      td_name: "starSubscriptionTypeBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarSubscriptionTypeBotBuilder { inner }
  }

  pub fn is_canceled_by_bot(&self) -> bool {
    self.is_canceled_by_bot
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn photo(&self) -> &Photo {
    &self.photo
  }

  pub fn invoice_link(&self) -> &String {
    &self.invoice_link
  }
}

#[doc(hidden)]
pub struct RTDStarSubscriptionTypeBotBuilder {
  inner: StarSubscriptionTypeBot,
}

impl RTDStarSubscriptionTypeBotBuilder {
  pub fn build(&self) -> StarSubscriptionTypeBot {
    self.inner.clone()
  }

  pub fn is_canceled_by_bot(&mut self, is_canceled_by_bot: bool) -> &mut Self {
    self.inner.is_canceled_by_bot = is_canceled_by_bot;
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

  pub fn invoice_link<T: AsRef<str>>(&mut self, invoice_link: T) -> &mut Self {
    self.inner.invoice_link = invoice_link.as_ref().to_string();
    self
  }
}

impl AsRef<StarSubscriptionTypeBot> for StarSubscriptionTypeBot {
  fn as_ref(&self) -> &StarSubscriptionTypeBot {
    self
  }
}

impl AsRef<StarSubscriptionTypeBot> for RTDStarSubscriptionTypeBotBuilder {
  fn as_ref(&self) -> &StarSubscriptionTypeBot {
    &self.inner
  }
}

/// Describes a subscription to a channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarSubscriptionTypeChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the subscription is active and the user can use the method reuseStarSubscription to join the subscribed chat again
  can_reuse: bool,
  /// The invite link that can be used to renew the subscription if it has expired; may be empty if the link isn't available anymore
  invite_link: String,
}

impl RObject for StarSubscriptionTypeChannel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starSubscriptionTypeChannel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarSubscriptionType for StarSubscriptionTypeChannel {}

impl StarSubscriptionTypeChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarSubscriptionTypeChannelBuilder {
    let inner = StarSubscriptionTypeChannel {
      td_name: "starSubscriptionTypeChannel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarSubscriptionTypeChannelBuilder { inner }
  }

  pub fn can_reuse(&self) -> bool {
    self.can_reuse
  }

  pub fn invite_link(&self) -> &String {
    &self.invite_link
  }
}

#[doc(hidden)]
pub struct RTDStarSubscriptionTypeChannelBuilder {
  inner: StarSubscriptionTypeChannel,
}

impl RTDStarSubscriptionTypeChannelBuilder {
  pub fn build(&self) -> StarSubscriptionTypeChannel {
    self.inner.clone()
  }

  pub fn can_reuse(&mut self, can_reuse: bool) -> &mut Self {
    self.inner.can_reuse = can_reuse;
    self
  }

  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }
}

impl AsRef<StarSubscriptionTypeChannel> for StarSubscriptionTypeChannel {
  fn as_ref(&self) -> &StarSubscriptionTypeChannel {
    self
  }
}

impl AsRef<StarSubscriptionTypeChannel> for RTDStarSubscriptionTypeChannelBuilder {
  fn as_ref(&self) -> &StarSubscriptionTypeChannel {
    &self.inner
  }
}
