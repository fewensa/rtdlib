use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes state of a suggested post
pub trait TDSuggestedPostState: Debug + RObject {}

/// Describes state of a suggested post
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SuggestedPostState {
  #[doc(hidden)]
  _Default(()),
  /// The post was approved
  Approved(SuggestedPostStateApproved),
  /// The post was declined
  Declined(SuggestedPostStateDeclined),
  /// The post must be approved or declined
  Pending(SuggestedPostStatePending),
}

impl Default for SuggestedPostState {
  fn default() -> Self {
    SuggestedPostState::_Default(())
  }
}

impl<'de> Deserialize<'de> for SuggestedPostState {
  fn deserialize<D>(deserializer: D) -> Result<SuggestedPostState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SuggestedPostState,
      (suggestedPostStateApproved, Approved);
      (suggestedPostStateDeclined, Declined);
      (suggestedPostStatePending, Pending);

    )(deserializer)
  }
}

impl RObject for SuggestedPostState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SuggestedPostState::Approved(t) => t.td_name(),
      SuggestedPostState::Declined(t) => t.td_name(),
      SuggestedPostState::Pending(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SuggestedPostState::Approved(t) => t.extra(),
      SuggestedPostState::Declined(t) => t.extra(),
      SuggestedPostState::Pending(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SuggestedPostState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SuggestedPostState::_Default(_))
  }

  pub fn is_approved(&self) -> bool {
    matches!(self, SuggestedPostState::Approved(_))
  }
  pub fn is_declined(&self) -> bool {
    matches!(self, SuggestedPostState::Declined(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, SuggestedPostState::Pending(_))
  }

  pub fn on_approved<F: FnOnce(&SuggestedPostStateApproved)>(&self, fnc: F) -> &Self {
    if let SuggestedPostState::Approved(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_declined<F: FnOnce(&SuggestedPostStateDeclined)>(&self, fnc: F) -> &Self {
    if let SuggestedPostState::Declined(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&SuggestedPostStatePending)>(&self, fnc: F) -> &Self {
    if let SuggestedPostState::Pending(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_approved(&self) -> Option<&SuggestedPostStateApproved> {
    if let SuggestedPostState::Approved(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_declined(&self) -> Option<&SuggestedPostStateDeclined> {
    if let SuggestedPostState::Declined(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&SuggestedPostStatePending> {
    if let SuggestedPostState::Pending(t) = self {
      return Some(t);
    }
    None
  }

  pub fn approved<T: AsRef<SuggestedPostStateApproved>>(t: T) -> Self {
    SuggestedPostState::Approved(t.as_ref().clone())
  }

  pub fn declined<T: AsRef<SuggestedPostStateDeclined>>(t: T) -> Self {
    SuggestedPostState::Declined(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<SuggestedPostStatePending>>(t: T) -> Self {
    SuggestedPostState::Pending(t.as_ref().clone())
  }
}

impl AsRef<SuggestedPostState> for SuggestedPostState {
  fn as_ref(&self) -> &SuggestedPostState {
    self
  }
}

/// The post was approved
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedPostStateApproved {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedPostStateApproved {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedPostStateApproved"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedPostState for SuggestedPostStateApproved {}

impl SuggestedPostStateApproved {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedPostStateApprovedBuilder {
    let inner = SuggestedPostStateApproved {
      td_name: "suggestedPostStateApproved".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedPostStateApprovedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedPostStateApprovedBuilder {
  inner: SuggestedPostStateApproved,
}

impl RTDSuggestedPostStateApprovedBuilder {
  pub fn build(&self) -> SuggestedPostStateApproved {
    self.inner.clone()
  }
}

impl AsRef<SuggestedPostStateApproved> for SuggestedPostStateApproved {
  fn as_ref(&self) -> &SuggestedPostStateApproved {
    self
  }
}

impl AsRef<SuggestedPostStateApproved> for RTDSuggestedPostStateApprovedBuilder {
  fn as_ref(&self) -> &SuggestedPostStateApproved {
    &self.inner
  }
}

/// The post was declined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedPostStateDeclined {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedPostStateDeclined {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedPostStateDeclined"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedPostState for SuggestedPostStateDeclined {}

impl SuggestedPostStateDeclined {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedPostStateDeclinedBuilder {
    let inner = SuggestedPostStateDeclined {
      td_name: "suggestedPostStateDeclined".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedPostStateDeclinedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedPostStateDeclinedBuilder {
  inner: SuggestedPostStateDeclined,
}

impl RTDSuggestedPostStateDeclinedBuilder {
  pub fn build(&self) -> SuggestedPostStateDeclined {
    self.inner.clone()
  }
}

impl AsRef<SuggestedPostStateDeclined> for SuggestedPostStateDeclined {
  fn as_ref(&self) -> &SuggestedPostStateDeclined {
    self
  }
}

impl AsRef<SuggestedPostStateDeclined> for RTDSuggestedPostStateDeclinedBuilder {
  fn as_ref(&self) -> &SuggestedPostStateDeclined {
    &self.inner
  }
}

/// The post must be approved or declined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedPostStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedPostStatePending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedPostStatePending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedPostState for SuggestedPostStatePending {}

impl SuggestedPostStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedPostStatePendingBuilder {
    let inner = SuggestedPostStatePending {
      td_name: "suggestedPostStatePending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedPostStatePendingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedPostStatePendingBuilder {
  inner: SuggestedPostStatePending,
}

impl RTDSuggestedPostStatePendingBuilder {
  pub fn build(&self) -> SuggestedPostStatePending {
    self.inner.clone()
  }
}

impl AsRef<SuggestedPostStatePending> for SuggestedPostStatePending {
  fn as_ref(&self) -> &SuggestedPostStatePending {
    self
  }
}

impl AsRef<SuggestedPostStatePending> for RTDSuggestedPostStatePendingBuilder {
  fn as_ref(&self) -> &SuggestedPostStatePending {
    &self.inner
  }
}
