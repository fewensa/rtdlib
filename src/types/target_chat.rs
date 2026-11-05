use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the target chat to be opened
pub trait TDTargetChat: Debug + RObject {}

/// Describes the target chat to be opened
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TargetChat {
  #[doc(hidden)]
  _Default(()),
  /// The chat needs to be chosen by the user among chats of the specified types
  Chosen(TargetChatChosen),
  /// The currently opened chat and forum topic must be kept
  Current(TargetChatCurrent),
  /// The chat needs to be open with the provided internal link
  InternalLink(TargetChatInternalLink),
}

impl Default for TargetChat {
  fn default() -> Self {
    TargetChat::_Default(())
  }
}

impl<'de> Deserialize<'de> for TargetChat {
  fn deserialize<D>(deserializer: D) -> Result<TargetChat, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TargetChat,
      (targetChatChosen, Chosen);
      (targetChatCurrent, Current);
      (targetChatInternalLink, InternalLink);

    )(deserializer)
  }
}

impl RObject for TargetChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TargetChat::Chosen(t) => t.td_name(),
      TargetChat::Current(t) => t.td_name(),
      TargetChat::InternalLink(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TargetChat::Chosen(t) => t.extra(),
      TargetChat::Current(t) => t.extra(),
      TargetChat::InternalLink(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TargetChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TargetChat::_Default(_))
  }

  pub fn is_chosen(&self) -> bool {
    matches!(self, TargetChat::Chosen(_))
  }
  pub fn is_current(&self) -> bool {
    matches!(self, TargetChat::Current(_))
  }
  pub fn is_internal_link(&self) -> bool {
    matches!(self, TargetChat::InternalLink(_))
  }

  pub fn on_chosen<F: FnOnce(&TargetChatChosen)>(&self, fnc: F) -> &Self {
    if let TargetChat::Chosen(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_current<F: FnOnce(&TargetChatCurrent)>(&self, fnc: F) -> &Self {
    if let TargetChat::Current(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_internal_link<F: FnOnce(&TargetChatInternalLink)>(&self, fnc: F) -> &Self {
    if let TargetChat::InternalLink(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_chosen(&self) -> Option<&TargetChatChosen> {
    if let TargetChat::Chosen(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_current(&self) -> Option<&TargetChatCurrent> {
    if let TargetChat::Current(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_internal_link(&self) -> Option<&TargetChatInternalLink> {
    if let TargetChat::InternalLink(t) = self {
      return Some(t);
    }
    None
  }

  pub fn chosen<T: AsRef<TargetChatChosen>>(t: T) -> Self {
    TargetChat::Chosen(t.as_ref().clone())
  }

  pub fn current<T: AsRef<TargetChatCurrent>>(t: T) -> Self {
    TargetChat::Current(t.as_ref().clone())
  }

  pub fn internal_link<T: AsRef<TargetChatInternalLink>>(t: T) -> Self {
    TargetChat::InternalLink(t.as_ref().clone())
  }
}

impl AsRef<TargetChat> for TargetChat {
  fn as_ref(&self) -> &TargetChat {
    self
  }
}

/// The chat needs to be chosen by the user among chats of the specified types
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetChatChosen {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Allowed types for the chat
  types: TargetChatTypes,
}

impl RObject for TargetChatChosen {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "targetChatChosen"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTargetChat for TargetChatChosen {}

impl TargetChatChosen {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTargetChatChosenBuilder {
    let inner = TargetChatChosen {
      td_name: "targetChatChosen".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTargetChatChosenBuilder { inner }
  }

  pub fn types(&self) -> &TargetChatTypes {
    &self.types
  }
}

#[doc(hidden)]
pub struct RTDTargetChatChosenBuilder {
  inner: TargetChatChosen,
}

impl RTDTargetChatChosenBuilder {
  pub fn build(&self) -> TargetChatChosen {
    self.inner.clone()
  }

  pub fn types<T: AsRef<TargetChatTypes>>(&mut self, types: T) -> &mut Self {
    self.inner.types = types.as_ref().clone();
    self
  }
}

impl AsRef<TargetChatChosen> for TargetChatChosen {
  fn as_ref(&self) -> &TargetChatChosen {
    self
  }
}

impl AsRef<TargetChatChosen> for RTDTargetChatChosenBuilder {
  fn as_ref(&self) -> &TargetChatChosen {
    &self.inner
  }
}

/// The currently opened chat and forum topic must be kept
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetChatCurrent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TargetChatCurrent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "targetChatCurrent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTargetChat for TargetChatCurrent {}

impl TargetChatCurrent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTargetChatCurrentBuilder {
    let inner = TargetChatCurrent {
      td_name: "targetChatCurrent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTargetChatCurrentBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTargetChatCurrentBuilder {
  inner: TargetChatCurrent,
}

impl RTDTargetChatCurrentBuilder {
  pub fn build(&self) -> TargetChatCurrent {
    self.inner.clone()
  }
}

impl AsRef<TargetChatCurrent> for TargetChatCurrent {
  fn as_ref(&self) -> &TargetChatCurrent {
    self
  }
}

impl AsRef<TargetChatCurrent> for RTDTargetChatCurrentBuilder {
  fn as_ref(&self) -> &TargetChatCurrent {
    &self.inner
  }
}

/// The chat needs to be open with the provided internal link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetChatInternalLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// An internal link pointing to the chat
  link: Box<InternalLinkType>,
}

impl RObject for TargetChatInternalLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "targetChatInternalLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTargetChat for TargetChatInternalLink {}

impl TargetChatInternalLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTargetChatInternalLinkBuilder {
    let inner = TargetChatInternalLink {
      td_name: "targetChatInternalLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTargetChatInternalLinkBuilder { inner }
  }

  pub fn link(&self) -> &Box<InternalLinkType> {
    &self.link
  }
}

#[doc(hidden)]
pub struct RTDTargetChatInternalLinkBuilder {
  inner: TargetChatInternalLink,
}

impl RTDTargetChatInternalLinkBuilder {
  pub fn build(&self) -> TargetChatInternalLink {
    self.inner.clone()
  }

  pub fn link<T: AsRef<Box<InternalLinkType>>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().clone();
    self
  }
}

impl AsRef<TargetChatInternalLink> for TargetChatInternalLink {
  fn as_ref(&self) -> &TargetChatInternalLink {
    self
  }
}

impl AsRef<TargetChatInternalLink> for RTDTargetChatInternalLinkBuilder {
  fn as_ref(&self) -> &TargetChatInternalLink {
    &self.inner
  }
}
