use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes reset state of an email address
pub trait TDEmailAddressResetState: Debug + RObject {}

/// Describes reset state of an email address
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EmailAddressResetState {
  #[doc(hidden)]
  _Default(()),
  /// Email address can be reset after the given period. Call resetAuthenticationEmailAddress to reset it and allow the user to authorize with a code sent to the user's phone number
  Available(EmailAddressResetStateAvailable),
  /// Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
  Pending(EmailAddressResetStatePending),
}

impl Default for EmailAddressResetState {
  fn default() -> Self {
    EmailAddressResetState::_Default(())
  }
}

impl<'de> Deserialize<'de> for EmailAddressResetState {
  fn deserialize<D>(deserializer: D) -> Result<EmailAddressResetState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      EmailAddressResetState,
      (emailAddressResetStateAvailable, Available);
      (emailAddressResetStatePending, Pending);

    )(deserializer)
  }
}

impl RObject for EmailAddressResetState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      EmailAddressResetState::Available(t) => t.td_name(),
      EmailAddressResetState::Pending(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      EmailAddressResetState::Available(t) => t.extra(),
      EmailAddressResetState::Pending(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmailAddressResetState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, EmailAddressResetState::_Default(_))
  }

  pub fn is_available(&self) -> bool {
    matches!(self, EmailAddressResetState::Available(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, EmailAddressResetState::Pending(_))
  }

  pub fn on_available<F: FnOnce(&EmailAddressResetStateAvailable)>(&self, fnc: F) -> &Self {
    if let EmailAddressResetState::Available(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&EmailAddressResetStatePending)>(&self, fnc: F) -> &Self {
    if let EmailAddressResetState::Pending(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_available(&self) -> Option<&EmailAddressResetStateAvailable> {
    if let EmailAddressResetState::Available(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&EmailAddressResetStatePending> {
    if let EmailAddressResetState::Pending(t) = self {
      return Some(t);
    }
    None
  }

  pub fn available<T: AsRef<EmailAddressResetStateAvailable>>(t: T) -> Self {
    EmailAddressResetState::Available(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<EmailAddressResetStatePending>>(t: T) -> Self {
    EmailAddressResetState::Pending(t.as_ref().clone())
  }
}

impl AsRef<EmailAddressResetState> for EmailAddressResetState {
  fn as_ref(&self) -> &EmailAddressResetState {
    self
  }
}

/// Email address can be reset after the given period. Call resetAuthenticationEmailAddress to reset it and allow the user to authorize with a code sent to the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressResetStateAvailable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Time required to wait before the email address can be reset; 0 if the user is subscribed to Telegram Premium
  wait_period: i64,
}

impl RObject for EmailAddressResetStateAvailable {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emailAddressResetStateAvailable"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmailAddressResetState for EmailAddressResetStateAvailable {}

impl EmailAddressResetStateAvailable {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmailAddressResetStateAvailableBuilder {
    let inner = EmailAddressResetStateAvailable {
      td_name: "emailAddressResetStateAvailable".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmailAddressResetStateAvailableBuilder { inner }
  }

  pub fn wait_period(&self) -> i64 {
    self.wait_period
  }
}

#[doc(hidden)]
pub struct RTDEmailAddressResetStateAvailableBuilder {
  inner: EmailAddressResetStateAvailable,
}

impl RTDEmailAddressResetStateAvailableBuilder {
  pub fn build(&self) -> EmailAddressResetStateAvailable {
    self.inner.clone()
  }

  pub fn wait_period(&mut self, wait_period: i64) -> &mut Self {
    self.inner.wait_period = wait_period;
    self
  }
}

impl AsRef<EmailAddressResetStateAvailable> for EmailAddressResetStateAvailable {
  fn as_ref(&self) -> &EmailAddressResetStateAvailable {
    self
  }
}

impl AsRef<EmailAddressResetStateAvailable> for RTDEmailAddressResetStateAvailableBuilder {
  fn as_ref(&self) -> &EmailAddressResetStateAvailable {
    &self.inner
  }
}

/// Email address reset has already been requested. Call resetAuthenticationEmailAddress to check whether immediate reset is possible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressResetStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Left time before the email address will be reset, in seconds. updateAuthorizationState is not sent when this field changes
  reset_in: i64,
}

impl RObject for EmailAddressResetStatePending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emailAddressResetStatePending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmailAddressResetState for EmailAddressResetStatePending {}

impl EmailAddressResetStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmailAddressResetStatePendingBuilder {
    let inner = EmailAddressResetStatePending {
      td_name: "emailAddressResetStatePending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmailAddressResetStatePendingBuilder { inner }
  }

  pub fn reset_in(&self) -> i64 {
    self.reset_in
  }
}

#[doc(hidden)]
pub struct RTDEmailAddressResetStatePendingBuilder {
  inner: EmailAddressResetStatePending,
}

impl RTDEmailAddressResetStatePendingBuilder {
  pub fn build(&self) -> EmailAddressResetStatePending {
    self.inner.clone()
  }

  pub fn reset_in(&mut self, reset_in: i64) -> &mut Self {
    self.inner.reset_in = reset_in;
    self
  }
}

impl AsRef<EmailAddressResetStatePending> for EmailAddressResetStatePending {
  fn as_ref(&self) -> &EmailAddressResetStatePending {
    self
  }
}

impl AsRef<EmailAddressResetStatePending> for RTDEmailAddressResetStatePendingBuilder {
  fn as_ref(&self) -> &EmailAddressResetStatePending {
    &self.inner
  }
}
