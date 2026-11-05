use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents the current authorization state of the TDLib client
pub trait TDAuthorizationState: Debug + RObject {}

/// Represents the current authorization state of the TDLib client
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AuthorizationState {
  #[doc(hidden)]
  _Default(()),
  /// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one must create a new instance of the TDLib client
  Closed(AuthorizationStateClosed),
  /// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
  Closing(AuthorizationStateClosing),
  /// The user is currently logging out
  LoggingOut(AuthorizationStateLoggingOut),
  /// The user has been successfully authorized. TDLib is now ready to answer general requests
  Ready(AuthorizationStateReady),
  /// TDLib needs the user's authentication code to authorize. Call checkAuthenticationCode to check the code
  WaitCode(AuthorizationStateWaitCode),
  /// TDLib needs the user's email address to authorize. Call setAuthenticationEmailAddress to provide the email address, or directly call checkAuthenticationEmailCode with Apple ID/Google ID token if allowed
  WaitEmailAddress(AuthorizationStateWaitEmailAddress),
  /// TDLib needs the user's authentication code sent to an email address to authorize. Call checkAuthenticationEmailCode to provide the code
  WaitEmailCode(AuthorizationStateWaitEmailCode),
  /// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
  WaitOtherDeviceConfirmation(AuthorizationStateWaitOtherDeviceConfirmation),
  /// The user has been authorized, but needs to enter a 2-step verification password to start using the application. Call checkAuthenticationPassword to provide the password, or requestAuthenticationPasswordRecovery to recover the password, or deleteAccount to delete the account after a week
  WaitPassword(AuthorizationStateWaitPassword),
  /// TDLib needs the user's phone number to authorize. Call setAuthenticationPhoneNumber to provide the phone number, or use requestQrCodeAuthentication, getAuthenticationPasskeyParameters, checkAuthenticationWebToken, or checkAuthenticationBotToken for other authentication options
  WaitPhoneNumber(AuthorizationStateWaitPhoneNumber),
  /// The user must buy Telegram Premium as an in-store purchase to log in. Call checkAuthenticationPremiumPurchase and then setAuthenticationPremiumPurchaseTransaction
  WaitPremiumPurchase(AuthorizationStateWaitPremiumPurchase),
  /// The user is unregistered and needs to accept terms of service and enter their first name and last name to finish registration. Call registerUser to accept the terms of service and provide the data
  WaitRegistration(AuthorizationStateWaitRegistration),
  /// Initialization parameters are needed. Call setTdlibParameters to provide them
  WaitTdlibParameters(AuthorizationStateWaitTdlibParameters),
  /// Returns the current authorization state. This is an offline method. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state. Can be called before initialization
  GetAuthorizationState(GetAuthorizationState),
}

impl Default for AuthorizationState {
  fn default() -> Self {
    AuthorizationState::_Default(())
  }
}

impl<'de> Deserialize<'de> for AuthorizationState {
  fn deserialize<D>(deserializer: D) -> Result<AuthorizationState, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AuthorizationState,
      (authorizationStateClosed, Closed);
      (authorizationStateClosing, Closing);
      (authorizationStateLoggingOut, LoggingOut);
      (authorizationStateReady, Ready);
      (authorizationStateWaitCode, WaitCode);
      (authorizationStateWaitEmailAddress, WaitEmailAddress);
      (authorizationStateWaitEmailCode, WaitEmailCode);
      (authorizationStateWaitOtherDeviceConfirmation, WaitOtherDeviceConfirmation);
      (authorizationStateWaitPassword, WaitPassword);
      (authorizationStateWaitPhoneNumber, WaitPhoneNumber);
      (authorizationStateWaitPremiumPurchase, WaitPremiumPurchase);
      (authorizationStateWaitRegistration, WaitRegistration);
      (authorizationStateWaitTdlibParameters, WaitTdlibParameters);
      (getAuthorizationState, GetAuthorizationState);

    )(deserializer)
  }
}

impl RObject for AuthorizationState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      AuthorizationState::Closed(t) => t.td_name(),
      AuthorizationState::Closing(t) => t.td_name(),
      AuthorizationState::LoggingOut(t) => t.td_name(),
      AuthorizationState::Ready(t) => t.td_name(),
      AuthorizationState::WaitCode(t) => t.td_name(),
      AuthorizationState::WaitEmailAddress(t) => t.td_name(),
      AuthorizationState::WaitEmailCode(t) => t.td_name(),
      AuthorizationState::WaitOtherDeviceConfirmation(t) => t.td_name(),
      AuthorizationState::WaitPassword(t) => t.td_name(),
      AuthorizationState::WaitPhoneNumber(t) => t.td_name(),
      AuthorizationState::WaitPremiumPurchase(t) => t.td_name(),
      AuthorizationState::WaitRegistration(t) => t.td_name(),
      AuthorizationState::WaitTdlibParameters(t) => t.td_name(),
      AuthorizationState::GetAuthorizationState(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      AuthorizationState::Closed(t) => t.extra(),
      AuthorizationState::Closing(t) => t.extra(),
      AuthorizationState::LoggingOut(t) => t.extra(),
      AuthorizationState::Ready(t) => t.extra(),
      AuthorizationState::WaitCode(t) => t.extra(),
      AuthorizationState::WaitEmailAddress(t) => t.extra(),
      AuthorizationState::WaitEmailCode(t) => t.extra(),
      AuthorizationState::WaitOtherDeviceConfirmation(t) => t.extra(),
      AuthorizationState::WaitPassword(t) => t.extra(),
      AuthorizationState::WaitPhoneNumber(t) => t.extra(),
      AuthorizationState::WaitPremiumPurchase(t) => t.extra(),
      AuthorizationState::WaitRegistration(t) => t.extra(),
      AuthorizationState::WaitTdlibParameters(t) => t.extra(),
      AuthorizationState::GetAuthorizationState(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AuthorizationState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, AuthorizationState::_Default(_))
  }

  pub fn is_closed(&self) -> bool {
    matches!(self, AuthorizationState::Closed(_))
  }
  pub fn is_closing(&self) -> bool {
    matches!(self, AuthorizationState::Closing(_))
  }
  pub fn is_logging_out(&self) -> bool {
    matches!(self, AuthorizationState::LoggingOut(_))
  }
  pub fn is_ready(&self) -> bool {
    matches!(self, AuthorizationState::Ready(_))
  }
  pub fn is_wait_code(&self) -> bool {
    matches!(self, AuthorizationState::WaitCode(_))
  }
  pub fn is_wait_email_address(&self) -> bool {
    matches!(self, AuthorizationState::WaitEmailAddress(_))
  }
  pub fn is_wait_email_code(&self) -> bool {
    matches!(self, AuthorizationState::WaitEmailCode(_))
  }
  pub fn is_wait_other_device_confirmation(&self) -> bool {
    matches!(self, AuthorizationState::WaitOtherDeviceConfirmation(_))
  }
  pub fn is_wait_password(&self) -> bool {
    matches!(self, AuthorizationState::WaitPassword(_))
  }
  pub fn is_wait_phone_number(&self) -> bool {
    matches!(self, AuthorizationState::WaitPhoneNumber(_))
  }
  pub fn is_wait_premium_purchase(&self) -> bool {
    matches!(self, AuthorizationState::WaitPremiumPurchase(_))
  }
  pub fn is_wait_registration(&self) -> bool {
    matches!(self, AuthorizationState::WaitRegistration(_))
  }
  pub fn is_wait_tdlib_parameters(&self) -> bool {
    matches!(self, AuthorizationState::WaitTdlibParameters(_))
  }
  pub fn is_get_authorization_state(&self) -> bool {
    matches!(self, AuthorizationState::GetAuthorizationState(_))
  }

  pub fn on_closed<F: FnOnce(&AuthorizationStateClosed)>(&self, fnc: F) -> &Self {
    if let AuthorizationState::Closed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_closing<F: FnOnce(&AuthorizationStateClosing)>(&self, fnc: F) -> &Self {
    if let AuthorizationState::Closing(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_logging_out<F: FnOnce(&AuthorizationStateLoggingOut)>(&self, fnc: F) -> &Self {
    if let AuthorizationState::LoggingOut(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ready<F: FnOnce(&AuthorizationStateReady)>(&self, fnc: F) -> &Self {
    if let AuthorizationState::Ready(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_code<F: FnOnce(&AuthorizationStateWaitCode)>(&self, fnc: F) -> &Self {
    if let AuthorizationState::WaitCode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_email_address<F: FnOnce(&AuthorizationStateWaitEmailAddress)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AuthorizationState::WaitEmailAddress(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_email_code<F: FnOnce(&AuthorizationStateWaitEmailCode)>(&self, fnc: F) -> &Self {
    if let AuthorizationState::WaitEmailCode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_other_device_confirmation<
    F: FnOnce(&AuthorizationStateWaitOtherDeviceConfirmation),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let AuthorizationState::WaitOtherDeviceConfirmation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_password<F: FnOnce(&AuthorizationStateWaitPassword)>(&self, fnc: F) -> &Self {
    if let AuthorizationState::WaitPassword(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_phone_number<F: FnOnce(&AuthorizationStateWaitPhoneNumber)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AuthorizationState::WaitPhoneNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_premium_purchase<F: FnOnce(&AuthorizationStateWaitPremiumPurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AuthorizationState::WaitPremiumPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_registration<F: FnOnce(&AuthorizationStateWaitRegistration)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AuthorizationState::WaitRegistration(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_wait_tdlib_parameters<F: FnOnce(&AuthorizationStateWaitTdlibParameters)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AuthorizationState::WaitTdlibParameters(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_get_authorization_state<F: FnOnce(&GetAuthorizationState)>(&self, fnc: F) -> &Self {
    if let AuthorizationState::GetAuthorizationState(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_closed(&self) -> Option<&AuthorizationStateClosed> {
    if let AuthorizationState::Closed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_closing(&self) -> Option<&AuthorizationStateClosing> {
    if let AuthorizationState::Closing(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_logging_out(&self) -> Option<&AuthorizationStateLoggingOut> {
    if let AuthorizationState::LoggingOut(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ready(&self) -> Option<&AuthorizationStateReady> {
    if let AuthorizationState::Ready(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_code(&self) -> Option<&AuthorizationStateWaitCode> {
    if let AuthorizationState::WaitCode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_email_address(&self) -> Option<&AuthorizationStateWaitEmailAddress> {
    if let AuthorizationState::WaitEmailAddress(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_email_code(&self) -> Option<&AuthorizationStateWaitEmailCode> {
    if let AuthorizationState::WaitEmailCode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_other_device_confirmation(
    &self,
  ) -> Option<&AuthorizationStateWaitOtherDeviceConfirmation> {
    if let AuthorizationState::WaitOtherDeviceConfirmation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_password(&self) -> Option<&AuthorizationStateWaitPassword> {
    if let AuthorizationState::WaitPassword(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_phone_number(&self) -> Option<&AuthorizationStateWaitPhoneNumber> {
    if let AuthorizationState::WaitPhoneNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_premium_purchase(&self) -> Option<&AuthorizationStateWaitPremiumPurchase> {
    if let AuthorizationState::WaitPremiumPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_registration(&self) -> Option<&AuthorizationStateWaitRegistration> {
    if let AuthorizationState::WaitRegistration(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_wait_tdlib_parameters(&self) -> Option<&AuthorizationStateWaitTdlibParameters> {
    if let AuthorizationState::WaitTdlibParameters(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_get_authorization_state(&self) -> Option<&GetAuthorizationState> {
    if let AuthorizationState::GetAuthorizationState(t) = self {
      return Some(t);
    }
    None
  }

  pub fn closed<T: AsRef<AuthorizationStateClosed>>(t: T) -> Self {
    AuthorizationState::Closed(t.as_ref().clone())
  }

  pub fn closing<T: AsRef<AuthorizationStateClosing>>(t: T) -> Self {
    AuthorizationState::Closing(t.as_ref().clone())
  }

  pub fn logging_out<T: AsRef<AuthorizationStateLoggingOut>>(t: T) -> Self {
    AuthorizationState::LoggingOut(t.as_ref().clone())
  }

  pub fn ready<T: AsRef<AuthorizationStateReady>>(t: T) -> Self {
    AuthorizationState::Ready(t.as_ref().clone())
  }

  pub fn wait_code<T: AsRef<AuthorizationStateWaitCode>>(t: T) -> Self {
    AuthorizationState::WaitCode(t.as_ref().clone())
  }

  pub fn wait_email_address<T: AsRef<AuthorizationStateWaitEmailAddress>>(t: T) -> Self {
    AuthorizationState::WaitEmailAddress(t.as_ref().clone())
  }

  pub fn wait_email_code<T: AsRef<AuthorizationStateWaitEmailCode>>(t: T) -> Self {
    AuthorizationState::WaitEmailCode(t.as_ref().clone())
  }

  pub fn wait_other_device_confirmation<T: AsRef<AuthorizationStateWaitOtherDeviceConfirmation>>(
    t: T,
  ) -> Self {
    AuthorizationState::WaitOtherDeviceConfirmation(t.as_ref().clone())
  }

  pub fn wait_password<T: AsRef<AuthorizationStateWaitPassword>>(t: T) -> Self {
    AuthorizationState::WaitPassword(t.as_ref().clone())
  }

  pub fn wait_phone_number<T: AsRef<AuthorizationStateWaitPhoneNumber>>(t: T) -> Self {
    AuthorizationState::WaitPhoneNumber(t.as_ref().clone())
  }

  pub fn wait_premium_purchase<T: AsRef<AuthorizationStateWaitPremiumPurchase>>(t: T) -> Self {
    AuthorizationState::WaitPremiumPurchase(t.as_ref().clone())
  }

  pub fn wait_registration<T: AsRef<AuthorizationStateWaitRegistration>>(t: T) -> Self {
    AuthorizationState::WaitRegistration(t.as_ref().clone())
  }

  pub fn wait_tdlib_parameters<T: AsRef<AuthorizationStateWaitTdlibParameters>>(t: T) -> Self {
    AuthorizationState::WaitTdlibParameters(t.as_ref().clone())
  }

  pub fn get_authorization_state<T: AsRef<GetAuthorizationState>>(t: T) -> Self {
    AuthorizationState::GetAuthorizationState(t.as_ref().clone())
  }
}

impl AsRef<AuthorizationState> for AuthorizationState {
  fn as_ref(&self) -> &AuthorizationState {
    self
  }
}

/// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one must create a new instance of the TDLib client
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AuthorizationStateClosed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateClosed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateClosed {}

impl AuthorizationStateClosed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateClosedBuilder {
    let inner = AuthorizationStateClosed {
      td_name: "authorizationStateClosed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAuthorizationStateClosedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateClosedBuilder {
  inner: AuthorizationStateClosed,
}

impl RTDAuthorizationStateClosedBuilder {
  pub fn build(&self) -> AuthorizationStateClosed {
    self.inner.clone()
  }
}

impl AsRef<AuthorizationStateClosed> for AuthorizationStateClosed {
  fn as_ref(&self) -> &AuthorizationStateClosed {
    self
  }
}

impl AsRef<AuthorizationStateClosed> for RTDAuthorizationStateClosedBuilder {
  fn as_ref(&self) -> &AuthorizationStateClosed {
    &self.inner
  }
}

/// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateClosing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AuthorizationStateClosing {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateClosing"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateClosing {}

impl AuthorizationStateClosing {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateClosingBuilder {
    let inner = AuthorizationStateClosing {
      td_name: "authorizationStateClosing".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAuthorizationStateClosingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateClosingBuilder {
  inner: AuthorizationStateClosing,
}

impl RTDAuthorizationStateClosingBuilder {
  pub fn build(&self) -> AuthorizationStateClosing {
    self.inner.clone()
  }
}

impl AsRef<AuthorizationStateClosing> for AuthorizationStateClosing {
  fn as_ref(&self) -> &AuthorizationStateClosing {
    self
  }
}

impl AsRef<AuthorizationStateClosing> for RTDAuthorizationStateClosingBuilder {
  fn as_ref(&self) -> &AuthorizationStateClosing {
    &self.inner
  }
}

/// The user is currently logging out
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateLoggingOut {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AuthorizationStateLoggingOut {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateLoggingOut"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateLoggingOut {}

impl AuthorizationStateLoggingOut {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateLoggingOutBuilder {
    let inner = AuthorizationStateLoggingOut {
      td_name: "authorizationStateLoggingOut".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAuthorizationStateLoggingOutBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateLoggingOutBuilder {
  inner: AuthorizationStateLoggingOut,
}

impl RTDAuthorizationStateLoggingOutBuilder {
  pub fn build(&self) -> AuthorizationStateLoggingOut {
    self.inner.clone()
  }
}

impl AsRef<AuthorizationStateLoggingOut> for AuthorizationStateLoggingOut {
  fn as_ref(&self) -> &AuthorizationStateLoggingOut {
    self
  }
}

impl AsRef<AuthorizationStateLoggingOut> for RTDAuthorizationStateLoggingOutBuilder {
  fn as_ref(&self) -> &AuthorizationStateLoggingOut {
    &self.inner
  }
}

/// The user has been successfully authorized. TDLib is now ready to answer general requests
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AuthorizationStateReady {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateReady"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateReady {}

impl AuthorizationStateReady {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateReadyBuilder {
    let inner = AuthorizationStateReady {
      td_name: "authorizationStateReady".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAuthorizationStateReadyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateReadyBuilder {
  inner: AuthorizationStateReady,
}

impl RTDAuthorizationStateReadyBuilder {
  pub fn build(&self) -> AuthorizationStateReady {
    self.inner.clone()
  }
}

impl AsRef<AuthorizationStateReady> for AuthorizationStateReady {
  fn as_ref(&self) -> &AuthorizationStateReady {
    self
  }
}

impl AsRef<AuthorizationStateReady> for RTDAuthorizationStateReadyBuilder {
  fn as_ref(&self) -> &AuthorizationStateReady {
    &self.inner
  }
}

/// TDLib needs the user's authentication code to authorize. Call checkAuthenticationCode to check the code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the authorization code that was sent
  code_info: AuthenticationCodeInfo,
}

impl RObject for AuthorizationStateWaitCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitCode {}

impl AuthorizationStateWaitCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitCodeBuilder {
    let inner = AuthorizationStateWaitCode {
      td_name: "authorizationStateWaitCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthorizationStateWaitCodeBuilder { inner }
  }

  pub fn code_info(&self) -> &AuthenticationCodeInfo {
    &self.code_info
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitCodeBuilder {
  inner: AuthorizationStateWaitCode,
}

impl RTDAuthorizationStateWaitCodeBuilder {
  pub fn build(&self) -> AuthorizationStateWaitCode {
    self.inner.clone()
  }

  pub fn code_info<T: AsRef<AuthenticationCodeInfo>>(&mut self, code_info: T) -> &mut Self {
    self.inner.code_info = code_info.as_ref().clone();
    self
  }
}

impl AsRef<AuthorizationStateWaitCode> for AuthorizationStateWaitCode {
  fn as_ref(&self) -> &AuthorizationStateWaitCode {
    self
  }
}

impl AsRef<AuthorizationStateWaitCode> for RTDAuthorizationStateWaitCodeBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitCode {
    &self.inner
  }
}

/// TDLib needs the user's email address to authorize. Call setAuthenticationEmailAddress to provide the email address, or directly call checkAuthenticationEmailCode with Apple ID/Google ID token if allowed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if authorization through Apple ID is allowed
  allow_apple_id: bool,
  /// True, if authorization through Google ID is allowed
  allow_google_id: bool,
}

impl RObject for AuthorizationStateWaitEmailAddress {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitEmailAddress"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitEmailAddress {}

impl AuthorizationStateWaitEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitEmailAddressBuilder {
    let inner = AuthorizationStateWaitEmailAddress {
      td_name: "authorizationStateWaitEmailAddress".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthorizationStateWaitEmailAddressBuilder { inner }
  }

  pub fn allow_apple_id(&self) -> bool {
    self.allow_apple_id
  }

  pub fn allow_google_id(&self) -> bool {
    self.allow_google_id
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitEmailAddressBuilder {
  inner: AuthorizationStateWaitEmailAddress,
}

impl RTDAuthorizationStateWaitEmailAddressBuilder {
  pub fn build(&self) -> AuthorizationStateWaitEmailAddress {
    self.inner.clone()
  }

  pub fn allow_apple_id(&mut self, allow_apple_id: bool) -> &mut Self {
    self.inner.allow_apple_id = allow_apple_id;
    self
  }

  pub fn allow_google_id(&mut self, allow_google_id: bool) -> &mut Self {
    self.inner.allow_google_id = allow_google_id;
    self
  }
}

impl AsRef<AuthorizationStateWaitEmailAddress> for AuthorizationStateWaitEmailAddress {
  fn as_ref(&self) -> &AuthorizationStateWaitEmailAddress {
    self
  }
}

impl AsRef<AuthorizationStateWaitEmailAddress> for RTDAuthorizationStateWaitEmailAddressBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitEmailAddress {
    &self.inner
  }
}

/// TDLib needs the user's authentication code sent to an email address to authorize. Call checkAuthenticationEmailCode to provide the code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitEmailCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if authorization through Apple ID is allowed
  allow_apple_id: bool,
  /// True, if authorization through Google ID is allowed
  allow_google_id: bool,
  /// Information about the sent authentication code
  code_info: EmailAddressAuthenticationCodeInfo,
  /// Reset state of the email address; may be null if the email address can't be reset
  email_address_reset_state: Option<EmailAddressResetState>,
}

impl RObject for AuthorizationStateWaitEmailCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitEmailCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitEmailCode {}

impl AuthorizationStateWaitEmailCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitEmailCodeBuilder {
    let inner = AuthorizationStateWaitEmailCode {
      td_name: "authorizationStateWaitEmailCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthorizationStateWaitEmailCodeBuilder { inner }
  }

  pub fn allow_apple_id(&self) -> bool {
    self.allow_apple_id
  }

  pub fn allow_google_id(&self) -> bool {
    self.allow_google_id
  }

  pub fn code_info(&self) -> &EmailAddressAuthenticationCodeInfo {
    &self.code_info
  }

  pub fn email_address_reset_state(&self) -> &Option<EmailAddressResetState> {
    &self.email_address_reset_state
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitEmailCodeBuilder {
  inner: AuthorizationStateWaitEmailCode,
}

impl RTDAuthorizationStateWaitEmailCodeBuilder {
  pub fn build(&self) -> AuthorizationStateWaitEmailCode {
    self.inner.clone()
  }

  pub fn allow_apple_id(&mut self, allow_apple_id: bool) -> &mut Self {
    self.inner.allow_apple_id = allow_apple_id;
    self
  }

  pub fn allow_google_id(&mut self, allow_google_id: bool) -> &mut Self {
    self.inner.allow_google_id = allow_google_id;
    self
  }

  pub fn code_info<T: AsRef<EmailAddressAuthenticationCodeInfo>>(
    &mut self,
    code_info: T,
  ) -> &mut Self {
    self.inner.code_info = code_info.as_ref().clone();
    self
  }

  pub fn email_address_reset_state<T: AsRef<EmailAddressResetState>>(
    &mut self,
    email_address_reset_state: T,
  ) -> &mut Self {
    self.inner.email_address_reset_state = Some(email_address_reset_state.as_ref().clone());
    self
  }
}

impl AsRef<AuthorizationStateWaitEmailCode> for AuthorizationStateWaitEmailCode {
  fn as_ref(&self) -> &AuthorizationStateWaitEmailCode {
    self
  }
}

impl AsRef<AuthorizationStateWaitEmailCode> for RTDAuthorizationStateWaitEmailCodeBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitEmailCode {
    &self.inner
  }
}

/// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitOtherDeviceConfirmation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A tg:// URL for the QR code. The link will be updated frequently
  link: String,
}

impl RObject for AuthorizationStateWaitOtherDeviceConfirmation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitOtherDeviceConfirmation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitOtherDeviceConfirmation {}

impl AuthorizationStateWaitOtherDeviceConfirmation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
    let inner = AuthorizationStateWaitOtherDeviceConfirmation {
      td_name: "authorizationStateWaitOtherDeviceConfirmation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder { inner }
  }

  pub fn link(&self) -> &String {
    &self.link
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
  inner: AuthorizationStateWaitOtherDeviceConfirmation,
}

impl RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
  pub fn build(&self) -> AuthorizationStateWaitOtherDeviceConfirmation {
    self.inner.clone()
  }

  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }
}

impl AsRef<AuthorizationStateWaitOtherDeviceConfirmation>
  for AuthorizationStateWaitOtherDeviceConfirmation
{
  fn as_ref(&self) -> &AuthorizationStateWaitOtherDeviceConfirmation {
    self
  }
}

impl AsRef<AuthorizationStateWaitOtherDeviceConfirmation>
  for RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder
{
  fn as_ref(&self) -> &AuthorizationStateWaitOtherDeviceConfirmation {
    &self.inner
  }
}

/// The user has been authorized, but needs to enter a 2-step verification password to start using the application. Call checkAuthenticationPassword to provide the password, or requestAuthenticationPasswordRecovery to recover the password, or deleteAccount to delete the account after a week
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hint for the password; may be empty
  password_hint: String,
  /// True, if a recovery email address has been set up
  has_recovery_email_address: bool,
  /// True, if some Telegram Passport elements were saved
  has_passport_data: bool,
  /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent
  recovery_email_address_pattern: String,
}

impl RObject for AuthorizationStateWaitPassword {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitPassword"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitPassword {}

impl AuthorizationStateWaitPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitPasswordBuilder {
    let inner = AuthorizationStateWaitPassword {
      td_name: "authorizationStateWaitPassword".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthorizationStateWaitPasswordBuilder { inner }
  }

  pub fn password_hint(&self) -> &String {
    &self.password_hint
  }

  pub fn has_recovery_email_address(&self) -> bool {
    self.has_recovery_email_address
  }

  pub fn has_passport_data(&self) -> bool {
    self.has_passport_data
  }

  pub fn recovery_email_address_pattern(&self) -> &String {
    &self.recovery_email_address_pattern
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitPasswordBuilder {
  inner: AuthorizationStateWaitPassword,
}

impl RTDAuthorizationStateWaitPasswordBuilder {
  pub fn build(&self) -> AuthorizationStateWaitPassword {
    self.inner.clone()
  }

  pub fn password_hint<T: AsRef<str>>(&mut self, password_hint: T) -> &mut Self {
    self.inner.password_hint = password_hint.as_ref().to_string();
    self
  }

  pub fn has_recovery_email_address(&mut self, has_recovery_email_address: bool) -> &mut Self {
    self.inner.has_recovery_email_address = has_recovery_email_address;
    self
  }

  pub fn has_passport_data(&mut self, has_passport_data: bool) -> &mut Self {
    self.inner.has_passport_data = has_passport_data;
    self
  }

  pub fn recovery_email_address_pattern<T: AsRef<str>>(
    &mut self,
    recovery_email_address_pattern: T,
  ) -> &mut Self {
    self.inner.recovery_email_address_pattern = recovery_email_address_pattern.as_ref().to_string();
    self
  }
}

impl AsRef<AuthorizationStateWaitPassword> for AuthorizationStateWaitPassword {
  fn as_ref(&self) -> &AuthorizationStateWaitPassword {
    self
  }
}

impl AsRef<AuthorizationStateWaitPassword> for RTDAuthorizationStateWaitPasswordBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitPassword {
    &self.inner
  }
}

/// TDLib needs the user's phone number to authorize. Call setAuthenticationPhoneNumber to provide the phone number, or use requestQrCodeAuthentication, getAuthenticationPasskeyParameters, checkAuthenticationWebToken, or checkAuthenticationBotToken for other authentication options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitPhoneNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitPhoneNumber {}

impl AuthorizationStateWaitPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitPhoneNumberBuilder {
    let inner = AuthorizationStateWaitPhoneNumber {
      td_name: "authorizationStateWaitPhoneNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAuthorizationStateWaitPhoneNumberBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitPhoneNumberBuilder {
  inner: AuthorizationStateWaitPhoneNumber,
}

impl RTDAuthorizationStateWaitPhoneNumberBuilder {
  pub fn build(&self) -> AuthorizationStateWaitPhoneNumber {
    self.inner.clone()
  }
}

impl AsRef<AuthorizationStateWaitPhoneNumber> for AuthorizationStateWaitPhoneNumber {
  fn as_ref(&self) -> &AuthorizationStateWaitPhoneNumber {
    self
  }
}

impl AsRef<AuthorizationStateWaitPhoneNumber> for RTDAuthorizationStateWaitPhoneNumberBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitPhoneNumber {
    &self.inner
  }
}

/// The user must buy Telegram Premium as an in-store purchase to log in. Call checkAuthenticationPremiumPurchase and then setAuthenticationPremiumPurchaseTransaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPremiumPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the store product that must be bought
  store_product_id: String,
  /// Duration of the Telegram Premium subscription after the purchase; may be 0 if Telegram Premium subscription will not be granted
  premium_day_count: i64,
  /// Email address to use for support if the user has issues with Telegram Premium purchase
  support_email_address: String,
  /// Subject for the email sent to the support email address
  support_email_subject: String,
}

impl RObject for AuthorizationStateWaitPremiumPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitPremiumPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitPremiumPurchase {}

impl AuthorizationStateWaitPremiumPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitPremiumPurchaseBuilder {
    let inner = AuthorizationStateWaitPremiumPurchase {
      td_name: "authorizationStateWaitPremiumPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthorizationStateWaitPremiumPurchaseBuilder { inner }
  }

  pub fn store_product_id(&self) -> &String {
    &self.store_product_id
  }

  pub fn premium_day_count(&self) -> i64 {
    self.premium_day_count
  }

  pub fn support_email_address(&self) -> &String {
    &self.support_email_address
  }

  pub fn support_email_subject(&self) -> &String {
    &self.support_email_subject
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitPremiumPurchaseBuilder {
  inner: AuthorizationStateWaitPremiumPurchase,
}

impl RTDAuthorizationStateWaitPremiumPurchaseBuilder {
  pub fn build(&self) -> AuthorizationStateWaitPremiumPurchase {
    self.inner.clone()
  }

  pub fn store_product_id<T: AsRef<str>>(&mut self, store_product_id: T) -> &mut Self {
    self.inner.store_product_id = store_product_id.as_ref().to_string();
    self
  }

  pub fn premium_day_count(&mut self, premium_day_count: i64) -> &mut Self {
    self.inner.premium_day_count = premium_day_count;
    self
  }

  pub fn support_email_address<T: AsRef<str>>(&mut self, support_email_address: T) -> &mut Self {
    self.inner.support_email_address = support_email_address.as_ref().to_string();
    self
  }

  pub fn support_email_subject<T: AsRef<str>>(&mut self, support_email_subject: T) -> &mut Self {
    self.inner.support_email_subject = support_email_subject.as_ref().to_string();
    self
  }
}

impl AsRef<AuthorizationStateWaitPremiumPurchase> for AuthorizationStateWaitPremiumPurchase {
  fn as_ref(&self) -> &AuthorizationStateWaitPremiumPurchase {
    self
  }
}

impl AsRef<AuthorizationStateWaitPremiumPurchase>
  for RTDAuthorizationStateWaitPremiumPurchaseBuilder
{
  fn as_ref(&self) -> &AuthorizationStateWaitPremiumPurchase {
    &self.inner
  }
}

/// The user is unregistered and needs to accept terms of service and enter their first name and last name to finish registration. Call registerUser to accept the terms of service and provide the data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Telegram terms of service
  terms_of_service: TermsOfService,
}

impl RObject for AuthorizationStateWaitRegistration {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitRegistration"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitRegistration {}

impl AuthorizationStateWaitRegistration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitRegistrationBuilder {
    let inner = AuthorizationStateWaitRegistration {
      td_name: "authorizationStateWaitRegistration".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthorizationStateWaitRegistrationBuilder { inner }
  }

  pub fn terms_of_service(&self) -> &TermsOfService {
    &self.terms_of_service
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitRegistrationBuilder {
  inner: AuthorizationStateWaitRegistration,
}

impl RTDAuthorizationStateWaitRegistrationBuilder {
  pub fn build(&self) -> AuthorizationStateWaitRegistration {
    self.inner.clone()
  }

  pub fn terms_of_service<T: AsRef<TermsOfService>>(&mut self, terms_of_service: T) -> &mut Self {
    self.inner.terms_of_service = terms_of_service.as_ref().clone();
    self
  }
}

impl AsRef<AuthorizationStateWaitRegistration> for AuthorizationStateWaitRegistration {
  fn as_ref(&self) -> &AuthorizationStateWaitRegistration {
    self
  }
}

impl AsRef<AuthorizationStateWaitRegistration> for RTDAuthorizationStateWaitRegistrationBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitRegistration {
    &self.inner
  }
}

/// Initialization parameters are needed. Call setTdlibParameters to provide them
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authorizationStateWaitTdlibParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthorizationState for AuthorizationStateWaitTdlibParameters {}

impl AuthorizationStateWaitTdlibParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthorizationStateWaitTdlibParametersBuilder {
    let inner = AuthorizationStateWaitTdlibParameters {
      td_name: "authorizationStateWaitTdlibParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDAuthorizationStateWaitTdlibParametersBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitTdlibParametersBuilder {
  inner: AuthorizationStateWaitTdlibParameters,
}

impl RTDAuthorizationStateWaitTdlibParametersBuilder {
  pub fn build(&self) -> AuthorizationStateWaitTdlibParameters {
    self.inner.clone()
  }
}

impl AsRef<AuthorizationStateWaitTdlibParameters> for AuthorizationStateWaitTdlibParameters {
  fn as_ref(&self) -> &AuthorizationStateWaitTdlibParameters {
    self
  }
}

impl AsRef<AuthorizationStateWaitTdlibParameters>
  for RTDAuthorizationStateWaitTdlibParametersBuilder
{
  fn as_ref(&self) -> &AuthorizationStateWaitTdlibParameters {
    &self.inner
  }
}
