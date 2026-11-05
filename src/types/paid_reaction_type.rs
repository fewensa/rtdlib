use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of paid message reaction
pub trait TDPaidReactionType: Debug + RObject {}

/// Describes type of paid message reaction
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PaidReactionType {
  #[doc(hidden)]
  _Default(()),
  /// An anonymous paid reaction
  Anonymous(PaidReactionTypeAnonymous),
  /// A paid reaction on behalf of an owned chat
  Chat(PaidReactionTypeChat),
  /// A paid reaction on behalf of the current user
  Regular(PaidReactionTypeRegular),
}

impl Default for PaidReactionType {
  fn default() -> Self {
    PaidReactionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for PaidReactionType {
  fn deserialize<D>(deserializer: D) -> Result<PaidReactionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PaidReactionType,
      (paidReactionTypeAnonymous, Anonymous);
      (paidReactionTypeChat, Chat);
      (paidReactionTypeRegular, Regular);

    )(deserializer)
  }
}

impl RObject for PaidReactionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PaidReactionType::Anonymous(t) => t.td_name(),
      PaidReactionType::Chat(t) => t.td_name(),
      PaidReactionType::Regular(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PaidReactionType::Anonymous(t) => t.extra(),
      PaidReactionType::Chat(t) => t.extra(),
      PaidReactionType::Regular(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaidReactionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PaidReactionType::_Default(_))
  }

  pub fn is_anonymous(&self) -> bool {
    matches!(self, PaidReactionType::Anonymous(_))
  }
  pub fn is_chat(&self) -> bool {
    matches!(self, PaidReactionType::Chat(_))
  }
  pub fn is_regular(&self) -> bool {
    matches!(self, PaidReactionType::Regular(_))
  }

  pub fn on_anonymous<F: FnOnce(&PaidReactionTypeAnonymous)>(&self, fnc: F) -> &Self {
    if let PaidReactionType::Anonymous(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat<F: FnOnce(&PaidReactionTypeChat)>(&self, fnc: F) -> &Self {
    if let PaidReactionType::Chat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_regular<F: FnOnce(&PaidReactionTypeRegular)>(&self, fnc: F) -> &Self {
    if let PaidReactionType::Regular(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_anonymous(&self) -> Option<&PaidReactionTypeAnonymous> {
    if let PaidReactionType::Anonymous(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat(&self) -> Option<&PaidReactionTypeChat> {
    if let PaidReactionType::Chat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_regular(&self) -> Option<&PaidReactionTypeRegular> {
    if let PaidReactionType::Regular(t) = self {
      return Some(t);
    }
    None
  }

  pub fn anonymous<T: AsRef<PaidReactionTypeAnonymous>>(t: T) -> Self {
    PaidReactionType::Anonymous(t.as_ref().clone())
  }

  pub fn chat<T: AsRef<PaidReactionTypeChat>>(t: T) -> Self {
    PaidReactionType::Chat(t.as_ref().clone())
  }

  pub fn regular<T: AsRef<PaidReactionTypeRegular>>(t: T) -> Self {
    PaidReactionType::Regular(t.as_ref().clone())
  }
}

impl AsRef<PaidReactionType> for PaidReactionType {
  fn as_ref(&self) -> &PaidReactionType {
    self
  }
}

/// An anonymous paid reaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaidReactionTypeAnonymous {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PaidReactionTypeAnonymous {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paidReactionTypeAnonymous"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaidReactionType for PaidReactionTypeAnonymous {}

impl PaidReactionTypeAnonymous {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaidReactionTypeAnonymousBuilder {
    let inner = PaidReactionTypeAnonymous {
      td_name: "paidReactionTypeAnonymous".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPaidReactionTypeAnonymousBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPaidReactionTypeAnonymousBuilder {
  inner: PaidReactionTypeAnonymous,
}

impl RTDPaidReactionTypeAnonymousBuilder {
  pub fn build(&self) -> PaidReactionTypeAnonymous {
    self.inner.clone()
  }
}

impl AsRef<PaidReactionTypeAnonymous> for PaidReactionTypeAnonymous {
  fn as_ref(&self) -> &PaidReactionTypeAnonymous {
    self
  }
}

impl AsRef<PaidReactionTypeAnonymous> for RTDPaidReactionTypeAnonymousBuilder {
  fn as_ref(&self) -> &PaidReactionTypeAnonymous {
    &self.inner
  }
}

/// A paid reaction on behalf of an owned chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaidReactionTypeChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat
  chat_id: i64,
}

impl RObject for PaidReactionTypeChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paidReactionTypeChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaidReactionType for PaidReactionTypeChat {}

impl PaidReactionTypeChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaidReactionTypeChatBuilder {
    let inner = PaidReactionTypeChat {
      td_name: "paidReactionTypeChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaidReactionTypeChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDPaidReactionTypeChatBuilder {
  inner: PaidReactionTypeChat,
}

impl RTDPaidReactionTypeChatBuilder {
  pub fn build(&self) -> PaidReactionTypeChat {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<PaidReactionTypeChat> for PaidReactionTypeChat {
  fn as_ref(&self) -> &PaidReactionTypeChat {
    self
  }
}

impl AsRef<PaidReactionTypeChat> for RTDPaidReactionTypeChatBuilder {
  fn as_ref(&self) -> &PaidReactionTypeChat {
    &self.inner
  }
}

/// A paid reaction on behalf of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaidReactionTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PaidReactionTypeRegular {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paidReactionTypeRegular"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPaidReactionType for PaidReactionTypeRegular {}

impl PaidReactionTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaidReactionTypeRegularBuilder {
    let inner = PaidReactionTypeRegular {
      td_name: "paidReactionTypeRegular".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPaidReactionTypeRegularBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPaidReactionTypeRegularBuilder {
  inner: PaidReactionTypeRegular,
}

impl RTDPaidReactionTypeRegularBuilder {
  pub fn build(&self) -> PaidReactionTypeRegular {
    self.inner.clone()
  }
}

impl AsRef<PaidReactionTypeRegular> for PaidReactionTypeRegular {
  fn as_ref(&self) -> &PaidReactionTypeRegular {
    self
  }
}

impl AsRef<PaidReactionTypeRegular> for RTDPaidReactionTypeRegularBuilder {
  fn as_ref(&self) -> &PaidReactionTypeRegular {
    &self.inner
  }
}
