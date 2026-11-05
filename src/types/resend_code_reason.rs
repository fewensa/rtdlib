use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the reason why a code needs to be re-sent
pub trait TDResendCodeReason: Debug + RObject {}

/// Describes the reason why a code needs to be re-sent
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ResendCodeReason {
  #[doc(hidden)]
  _Default(()),
  /// The user requested to resend the code
  UserRequest(ResendCodeReasonUserRequest),
  /// The code is re-sent, because device verification has failed
  VerificationFailed(ResendCodeReasonVerificationFailed),
}

impl Default for ResendCodeReason {
  fn default() -> Self {
    ResendCodeReason::_Default(())
  }
}

impl<'de> Deserialize<'de> for ResendCodeReason {
  fn deserialize<D>(deserializer: D) -> Result<ResendCodeReason, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ResendCodeReason,
      (resendCodeReasonUserRequest, UserRequest);
      (resendCodeReasonVerificationFailed, VerificationFailed);

    )(deserializer)
  }
}

impl RObject for ResendCodeReason {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ResendCodeReason::UserRequest(t) => t.td_name(),
      ResendCodeReason::VerificationFailed(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ResendCodeReason::UserRequest(t) => t.extra(),
      ResendCodeReason::VerificationFailed(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ResendCodeReason {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ResendCodeReason::_Default(_))
  }

  pub fn is_user_request(&self) -> bool {
    matches!(self, ResendCodeReason::UserRequest(_))
  }
  pub fn is_verification_failed(&self) -> bool {
    matches!(self, ResendCodeReason::VerificationFailed(_))
  }

  pub fn on_user_request<F: FnOnce(&ResendCodeReasonUserRequest)>(&self, fnc: F) -> &Self {
    if let ResendCodeReason::UserRequest(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_verification_failed<F: FnOnce(&ResendCodeReasonVerificationFailed)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ResendCodeReason::VerificationFailed(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_user_request(&self) -> Option<&ResendCodeReasonUserRequest> {
    if let ResendCodeReason::UserRequest(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_verification_failed(&self) -> Option<&ResendCodeReasonVerificationFailed> {
    if let ResendCodeReason::VerificationFailed(t) = self {
      return Some(t);
    }
    None
  }

  pub fn user_request<T: AsRef<ResendCodeReasonUserRequest>>(t: T) -> Self {
    ResendCodeReason::UserRequest(t.as_ref().clone())
  }

  pub fn verification_failed<T: AsRef<ResendCodeReasonVerificationFailed>>(t: T) -> Self {
    ResendCodeReason::VerificationFailed(t.as_ref().clone())
  }
}

impl AsRef<ResendCodeReason> for ResendCodeReason {
  fn as_ref(&self) -> &ResendCodeReason {
    self
  }
}

/// The user requested to resend the code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendCodeReasonUserRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ResendCodeReasonUserRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "resendCodeReasonUserRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDResendCodeReason for ResendCodeReasonUserRequest {}

impl ResendCodeReasonUserRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDResendCodeReasonUserRequestBuilder {
    let inner = ResendCodeReasonUserRequest {
      td_name: "resendCodeReasonUserRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDResendCodeReasonUserRequestBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDResendCodeReasonUserRequestBuilder {
  inner: ResendCodeReasonUserRequest,
}

impl RTDResendCodeReasonUserRequestBuilder {
  pub fn build(&self) -> ResendCodeReasonUserRequest {
    self.inner.clone()
  }
}

impl AsRef<ResendCodeReasonUserRequest> for ResendCodeReasonUserRequest {
  fn as_ref(&self) -> &ResendCodeReasonUserRequest {
    self
  }
}

impl AsRef<ResendCodeReasonUserRequest> for RTDResendCodeReasonUserRequestBuilder {
  fn as_ref(&self) -> &ResendCodeReasonUserRequest {
    &self.inner
  }
}

/// The code is re-sent, because device verification has failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendCodeReasonVerificationFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Cause of the verification failure, for example, "PLAY_SERVICES_NOT_AVAILABLE", "APNS_RECEIVE_TIMEOUT", or "APNS_INIT_FAILED"
  error_message: String,
}

impl RObject for ResendCodeReasonVerificationFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "resendCodeReasonVerificationFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDResendCodeReason for ResendCodeReasonVerificationFailed {}

impl ResendCodeReasonVerificationFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDResendCodeReasonVerificationFailedBuilder {
    let inner = ResendCodeReasonVerificationFailed {
      td_name: "resendCodeReasonVerificationFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDResendCodeReasonVerificationFailedBuilder { inner }
  }

  pub fn error_message(&self) -> &String {
    &self.error_message
  }
}

#[doc(hidden)]
pub struct RTDResendCodeReasonVerificationFailedBuilder {
  inner: ResendCodeReasonVerificationFailed,
}

impl RTDResendCodeReasonVerificationFailedBuilder {
  pub fn build(&self) -> ResendCodeReasonVerificationFailed {
    self.inner.clone()
  }

  pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
    self.inner.error_message = error_message.as_ref().to_string();
    self
  }
}

impl AsRef<ResendCodeReasonVerificationFailed> for ResendCodeReasonVerificationFailed {
  fn as_ref(&self) -> &ResendCodeReasonVerificationFailed {
    self
  }
}

impl AsRef<ResendCodeReasonVerificationFailed> for RTDResendCodeReasonVerificationFailedBuilder {
  fn as_ref(&self) -> &ResendCodeReasonVerificationFailed {
    &self.inner
  }
}
