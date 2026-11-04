use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes state of a gift purchase offer
pub trait TDGiftPurchaseOfferState: Debug + RObject {}

/// Describes state of a gift purchase offer
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiftPurchaseOfferState {
  #[doc(hidden)]
  _Default(()),
  /// The offer was accepted
  Accepted(GiftPurchaseOfferStateAccepted),
  /// The offer must be accepted or rejected
  Pending(GiftPurchaseOfferStatePending),
  /// The offer was rejected
  Rejected(GiftPurchaseOfferStateRejected),
}

impl Default for GiftPurchaseOfferState {
  fn default() -> Self {
    GiftPurchaseOfferState::_Default(())
  }
}

impl<'de> Deserialize<'de> for GiftPurchaseOfferState {
  fn deserialize<D>(deserializer: D) -> Result<GiftPurchaseOfferState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GiftPurchaseOfferState,
      (giftPurchaseOfferStateAccepted, Accepted);
      (giftPurchaseOfferStatePending, Pending);
      (giftPurchaseOfferStateRejected, Rejected);

    )(deserializer)
  }
}

impl RObject for GiftPurchaseOfferState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      GiftPurchaseOfferState::Accepted(t) => t.td_name(),
      GiftPurchaseOfferState::Pending(t) => t.td_name(),
      GiftPurchaseOfferState::Rejected(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      GiftPurchaseOfferState::Accepted(t) => t.extra(),
      GiftPurchaseOfferState::Pending(t) => t.extra(),
      GiftPurchaseOfferState::Rejected(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftPurchaseOfferState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, GiftPurchaseOfferState::_Default(_))
  }

  pub fn is_accepted(&self) -> bool {
    matches!(self, GiftPurchaseOfferState::Accepted(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, GiftPurchaseOfferState::Pending(_))
  }
  pub fn is_rejected(&self) -> bool {
    matches!(self, GiftPurchaseOfferState::Rejected(_))
  }

  pub fn on_accepted<F: FnOnce(&GiftPurchaseOfferStateAccepted)>(&self, fnc: F) -> &Self {
    if let GiftPurchaseOfferState::Accepted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&GiftPurchaseOfferStatePending)>(&self, fnc: F) -> &Self {
    if let GiftPurchaseOfferState::Pending(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_rejected<F: FnOnce(&GiftPurchaseOfferStateRejected)>(&self, fnc: F) -> &Self {
    if let GiftPurchaseOfferState::Rejected(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_accepted(&self) -> Option<&GiftPurchaseOfferStateAccepted> {
    if let GiftPurchaseOfferState::Accepted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&GiftPurchaseOfferStatePending> {
    if let GiftPurchaseOfferState::Pending(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_rejected(&self) -> Option<&GiftPurchaseOfferStateRejected> {
    if let GiftPurchaseOfferState::Rejected(t) = self {
      return Some(t);
    }
    None
  }

  pub fn accepted<T: AsRef<GiftPurchaseOfferStateAccepted>>(t: T) -> Self {
    GiftPurchaseOfferState::Accepted(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<GiftPurchaseOfferStatePending>>(t: T) -> Self {
    GiftPurchaseOfferState::Pending(t.as_ref().clone())
  }

  pub fn rejected<T: AsRef<GiftPurchaseOfferStateRejected>>(t: T) -> Self {
    GiftPurchaseOfferState::Rejected(t.as_ref().clone())
  }
}

impl AsRef<GiftPurchaseOfferState> for GiftPurchaseOfferState {
  fn as_ref(&self) -> &GiftPurchaseOfferState {
    self
  }
}

/// The offer was accepted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftPurchaseOfferStateAccepted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GiftPurchaseOfferStateAccepted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftPurchaseOfferStateAccepted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftPurchaseOfferState for GiftPurchaseOfferStateAccepted {}

impl GiftPurchaseOfferStateAccepted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftPurchaseOfferStateAcceptedBuilder {
    let inner = GiftPurchaseOfferStateAccepted {
      td_name: "giftPurchaseOfferStateAccepted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGiftPurchaseOfferStateAcceptedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGiftPurchaseOfferStateAcceptedBuilder {
  inner: GiftPurchaseOfferStateAccepted,
}

impl RTDGiftPurchaseOfferStateAcceptedBuilder {
  pub fn build(&self) -> GiftPurchaseOfferStateAccepted {
    self.inner.clone()
  }
}

impl AsRef<GiftPurchaseOfferStateAccepted> for GiftPurchaseOfferStateAccepted {
  fn as_ref(&self) -> &GiftPurchaseOfferStateAccepted {
    self
  }
}

impl AsRef<GiftPurchaseOfferStateAccepted> for RTDGiftPurchaseOfferStateAcceptedBuilder {
  fn as_ref(&self) -> &GiftPurchaseOfferStateAccepted {
    &self.inner
  }
}

/// The offer must be accepted or rejected
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftPurchaseOfferStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GiftPurchaseOfferStatePending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftPurchaseOfferStatePending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftPurchaseOfferState for GiftPurchaseOfferStatePending {}

impl GiftPurchaseOfferStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftPurchaseOfferStatePendingBuilder {
    let inner = GiftPurchaseOfferStatePending {
      td_name: "giftPurchaseOfferStatePending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGiftPurchaseOfferStatePendingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGiftPurchaseOfferStatePendingBuilder {
  inner: GiftPurchaseOfferStatePending,
}

impl RTDGiftPurchaseOfferStatePendingBuilder {
  pub fn build(&self) -> GiftPurchaseOfferStatePending {
    self.inner.clone()
  }
}

impl AsRef<GiftPurchaseOfferStatePending> for GiftPurchaseOfferStatePending {
  fn as_ref(&self) -> &GiftPurchaseOfferStatePending {
    self
  }
}

impl AsRef<GiftPurchaseOfferStatePending> for RTDGiftPurchaseOfferStatePendingBuilder {
  fn as_ref(&self) -> &GiftPurchaseOfferStatePending {
    &self.inner
  }
}

/// The offer was rejected
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftPurchaseOfferStateRejected {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for GiftPurchaseOfferStateRejected {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftPurchaseOfferStateRejected"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftPurchaseOfferState for GiftPurchaseOfferStateRejected {}

impl GiftPurchaseOfferStateRejected {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftPurchaseOfferStateRejectedBuilder {
    let inner = GiftPurchaseOfferStateRejected {
      td_name: "giftPurchaseOfferStateRejected".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDGiftPurchaseOfferStateRejectedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDGiftPurchaseOfferStateRejectedBuilder {
  inner: GiftPurchaseOfferStateRejected,
}

impl RTDGiftPurchaseOfferStateRejectedBuilder {
  pub fn build(&self) -> GiftPurchaseOfferStateRejected {
    self.inner.clone()
  }
}

impl AsRef<GiftPurchaseOfferStateRejected> for GiftPurchaseOfferStateRejected {
  fn as_ref(&self) -> &GiftPurchaseOfferStateRejected {
    self
  }
}

impl AsRef<GiftPurchaseOfferStateRejected> for RTDGiftPurchaseOfferStateRejectedBuilder {
  fn as_ref(&self) -> &GiftPurchaseOfferStateRejected {
    &self.inner
  }
}
