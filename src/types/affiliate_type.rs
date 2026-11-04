use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of affiliate for an affiliate program
pub trait TDAffiliateType: Debug + RObject {}

/// Describes type of affiliate for an affiliate program
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AffiliateType {
  #[doc(hidden)]
  _Default(()),
  /// The affiliate is a bot owned by the current user
  Bot(AffiliateTypeBot),
  /// The affiliate is a channel chat where the current user has can_post_messages administrator right
  Channel(AffiliateTypeChannel),
  /// The affiliate is the current user
  CurrentUser(AffiliateTypeCurrentUser),
}

impl Default for AffiliateType {
  fn default() -> Self {
    AffiliateType::_Default(())
  }
}

impl<'de> Deserialize<'de> for AffiliateType {
  fn deserialize<D>(deserializer: D) -> Result<AffiliateType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AffiliateType,
      (affiliateTypeBot, Bot);
      (affiliateTypeChannel, Channel);
      (affiliateTypeCurrentUser, CurrentUser);

    )(deserializer)
  }
}

impl RObject for AffiliateType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      AffiliateType::Bot(t) => t.td_name(),
      AffiliateType::Channel(t) => t.td_name(),
      AffiliateType::CurrentUser(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      AffiliateType::Bot(t) => t.extra(),
      AffiliateType::Channel(t) => t.extra(),
      AffiliateType::CurrentUser(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AffiliateType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, AffiliateType::_Default(_))
  }

  pub fn is_bot(&self) -> bool {
    matches!(self, AffiliateType::Bot(_))
  }
  pub fn is_channel(&self) -> bool {
    matches!(self, AffiliateType::Channel(_))
  }
  pub fn is_current_user(&self) -> bool {
    matches!(self, AffiliateType::CurrentUser(_))
  }

  pub fn on_bot<F: FnOnce(&AffiliateTypeBot)>(&self, fnc: F) -> &Self {
    if let AffiliateType::Bot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel<F: FnOnce(&AffiliateTypeChannel)>(&self, fnc: F) -> &Self {
    if let AffiliateType::Channel(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_current_user<F: FnOnce(&AffiliateTypeCurrentUser)>(&self, fnc: F) -> &Self {
    if let AffiliateType::CurrentUser(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_bot(&self) -> Option<&AffiliateTypeBot> {
    if let AffiliateType::Bot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel(&self) -> Option<&AffiliateTypeChannel> {
    if let AffiliateType::Channel(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_current_user(&self) -> Option<&AffiliateTypeCurrentUser> {
    if let AffiliateType::CurrentUser(t) = self {
      return Some(t);
    }
    None
  }

  pub fn bot<T: AsRef<AffiliateTypeBot>>(t: T) -> Self {
    AffiliateType::Bot(t.as_ref().clone())
  }

  pub fn channel<T: AsRef<AffiliateTypeChannel>>(t: T) -> Self {
    AffiliateType::Channel(t.as_ref().clone())
  }

  pub fn current_user<T: AsRef<AffiliateTypeCurrentUser>>(t: T) -> Self {
    AffiliateType::CurrentUser(t.as_ref().clone())
  }
}

impl AsRef<AffiliateType> for AffiliateType {
  fn as_ref(&self) -> &AffiliateType {
    self
  }
}

/// The affiliate is a bot owned by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateTypeBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the bot
  user_id: i64,
}

impl RObject for AffiliateTypeBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateTypeBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAffiliateType for AffiliateTypeBot {}

impl AffiliateTypeBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateTypeBotBuilder {
    let inner = AffiliateTypeBot {
      td_name: "affiliateTypeBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAffiliateTypeBotBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}

#[doc(hidden)]
pub struct RTDAffiliateTypeBotBuilder {
  inner: AffiliateTypeBot,
}

impl RTDAffiliateTypeBotBuilder {
  pub fn build(&self) -> AffiliateTypeBot {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }
}

impl AsRef<AffiliateTypeBot> for AffiliateTypeBot {
  fn as_ref(&self) -> &AffiliateTypeBot {
    self
  }
}

impl AsRef<AffiliateTypeBot> for RTDAffiliateTypeBotBuilder {
  fn as_ref(&self) -> &AffiliateTypeBot {
    &self.inner
  }
}

/// The affiliate is a channel chat where the current user has can_post_messages administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateTypeChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel chat
  chat_id: i64,
}

impl RObject for AffiliateTypeChannel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateTypeChannel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAffiliateType for AffiliateTypeChannel {}

impl AffiliateTypeChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateTypeChannelBuilder {
    let inner = AffiliateTypeChannel {
      td_name: "affiliateTypeChannel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAffiliateTypeChannelBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDAffiliateTypeChannelBuilder {
  inner: AffiliateTypeChannel,
}

impl RTDAffiliateTypeChannelBuilder {
  pub fn build(&self) -> AffiliateTypeChannel {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<AffiliateTypeChannel> for AffiliateTypeChannel {
  fn as_ref(&self) -> &AffiliateTypeChannel {
    self
  }
}

impl AsRef<AffiliateTypeChannel> for RTDAffiliateTypeChannelBuilder {
  fn as_ref(&self) -> &AffiliateTypeChannel {
    &self.inner
  }
}

/// The affiliate is the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateTypeCurrentUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AffiliateTypeCurrentUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateTypeCurrentUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAffiliateType for AffiliateTypeCurrentUser {}

impl AffiliateTypeCurrentUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateTypeCurrentUserBuilder {
    let inner = AffiliateTypeCurrentUser {
      td_name: "affiliateTypeCurrentUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAffiliateTypeCurrentUserBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAffiliateTypeCurrentUserBuilder {
  inner: AffiliateTypeCurrentUser,
}

impl RTDAffiliateTypeCurrentUserBuilder {
  pub fn build(&self) -> AffiliateTypeCurrentUser {
    self.inner.clone()
  }
}

impl AsRef<AffiliateTypeCurrentUser> for AffiliateTypeCurrentUser {
  fn as_ref(&self) -> &AffiliateTypeCurrentUser {
    self
  }
}

impl AsRef<AffiliateTypeCurrentUser> for RTDAffiliateTypeCurrentUserBuilder {
  fn as_ref(&self) -> &AffiliateTypeCurrentUser {
    &self.inner
  }
}
