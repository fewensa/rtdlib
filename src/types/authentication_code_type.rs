use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Provides information about the method by which an authentication code is delivered to the user
pub trait TDAuthenticationCodeType: Debug + RObject {}

/// Provides information about the method by which an authentication code is delivered to the user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AuthenticationCodeType {
  #[doc(hidden)]
  _Default(()),
  /// A digit-only authentication code is delivered via a phone call to the specified phone number
  Call(AuthenticationCodeTypeCall),
  /// A digit-only authentication code is delivered via Firebase Authentication to the official Android application
  FirebaseAndroid(AuthenticationCodeTypeFirebaseAndroid),
  /// A digit-only authentication code is delivered via Firebase Authentication to the official iOS application
  FirebaseIos(AuthenticationCodeTypeFirebaseIos),
  /// An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
  FlashCall(AuthenticationCodeTypeFlashCall),
  /// A digit-only authentication code is delivered to https://fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
  Fragment(AuthenticationCodeTypeFragment),
  /// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
  MissedCall(AuthenticationCodeTypeMissedCall),
  /// A digit-only authentication code is delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
  Sms(AuthenticationCodeTypeSms),
  /// An authentication code is a phrase from multiple words delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
  SmsPhrase(AuthenticationCodeTypeSmsPhrase),
  /// An authentication code is a word delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
  SmsWord(AuthenticationCodeTypeSmsWord),
  /// A digit-only authentication code is delivered via a private Telegram message, which can be viewed from another active session
  TelegramMessage(AuthenticationCodeTypeTelegramMessage),
}

impl Default for AuthenticationCodeType {
  fn default() -> Self {
    AuthenticationCodeType::_Default(())
  }
}

impl<'de> Deserialize<'de> for AuthenticationCodeType {
  fn deserialize<D>(deserializer: D) -> Result<AuthenticationCodeType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AuthenticationCodeType,
      (authenticationCodeTypeCall, Call);
      (authenticationCodeTypeFirebaseAndroid, FirebaseAndroid);
      (authenticationCodeTypeFirebaseIos, FirebaseIos);
      (authenticationCodeTypeFlashCall, FlashCall);
      (authenticationCodeTypeFragment, Fragment);
      (authenticationCodeTypeMissedCall, MissedCall);
      (authenticationCodeTypeSms, Sms);
      (authenticationCodeTypeSmsPhrase, SmsPhrase);
      (authenticationCodeTypeSmsWord, SmsWord);
      (authenticationCodeTypeTelegramMessage, TelegramMessage);

    )(deserializer)
  }
}

impl RObject for AuthenticationCodeType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      AuthenticationCodeType::Call(t) => t.td_name(),
      AuthenticationCodeType::FirebaseAndroid(t) => t.td_name(),
      AuthenticationCodeType::FirebaseIos(t) => t.td_name(),
      AuthenticationCodeType::FlashCall(t) => t.td_name(),
      AuthenticationCodeType::Fragment(t) => t.td_name(),
      AuthenticationCodeType::MissedCall(t) => t.td_name(),
      AuthenticationCodeType::Sms(t) => t.td_name(),
      AuthenticationCodeType::SmsPhrase(t) => t.td_name(),
      AuthenticationCodeType::SmsWord(t) => t.td_name(),
      AuthenticationCodeType::TelegramMessage(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      AuthenticationCodeType::Call(t) => t.extra(),
      AuthenticationCodeType::FirebaseAndroid(t) => t.extra(),
      AuthenticationCodeType::FirebaseIos(t) => t.extra(),
      AuthenticationCodeType::FlashCall(t) => t.extra(),
      AuthenticationCodeType::Fragment(t) => t.extra(),
      AuthenticationCodeType::MissedCall(t) => t.extra(),
      AuthenticationCodeType::Sms(t) => t.extra(),
      AuthenticationCodeType::SmsPhrase(t) => t.extra(),
      AuthenticationCodeType::SmsWord(t) => t.extra(),
      AuthenticationCodeType::TelegramMessage(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AuthenticationCodeType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, AuthenticationCodeType::_Default(_))
  }

  pub fn is_call(&self) -> bool {
    matches!(self, AuthenticationCodeType::Call(_))
  }
  pub fn is_firebase_android(&self) -> bool {
    matches!(self, AuthenticationCodeType::FirebaseAndroid(_))
  }
  pub fn is_firebase_ios(&self) -> bool {
    matches!(self, AuthenticationCodeType::FirebaseIos(_))
  }
  pub fn is_flash_call(&self) -> bool {
    matches!(self, AuthenticationCodeType::FlashCall(_))
  }
  pub fn is_fragment(&self) -> bool {
    matches!(self, AuthenticationCodeType::Fragment(_))
  }
  pub fn is_missed_call(&self) -> bool {
    matches!(self, AuthenticationCodeType::MissedCall(_))
  }
  pub fn is_sms(&self) -> bool {
    matches!(self, AuthenticationCodeType::Sms(_))
  }
  pub fn is_sms_phrase(&self) -> bool {
    matches!(self, AuthenticationCodeType::SmsPhrase(_))
  }
  pub fn is_sms_word(&self) -> bool {
    matches!(self, AuthenticationCodeType::SmsWord(_))
  }
  pub fn is_telegram_message(&self) -> bool {
    matches!(self, AuthenticationCodeType::TelegramMessage(_))
  }

  pub fn on_call<F: FnOnce(&AuthenticationCodeTypeCall)>(&self, fnc: F) -> &Self {
    if let AuthenticationCodeType::Call(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_firebase_android<F: FnOnce(&AuthenticationCodeTypeFirebaseAndroid)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AuthenticationCodeType::FirebaseAndroid(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_firebase_ios<F: FnOnce(&AuthenticationCodeTypeFirebaseIos)>(&self, fnc: F) -> &Self {
    if let AuthenticationCodeType::FirebaseIos(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_flash_call<F: FnOnce(&AuthenticationCodeTypeFlashCall)>(&self, fnc: F) -> &Self {
    if let AuthenticationCodeType::FlashCall(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fragment<F: FnOnce(&AuthenticationCodeTypeFragment)>(&self, fnc: F) -> &Self {
    if let AuthenticationCodeType::Fragment(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_missed_call<F: FnOnce(&AuthenticationCodeTypeMissedCall)>(&self, fnc: F) -> &Self {
    if let AuthenticationCodeType::MissedCall(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sms<F: FnOnce(&AuthenticationCodeTypeSms)>(&self, fnc: F) -> &Self {
    if let AuthenticationCodeType::Sms(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sms_phrase<F: FnOnce(&AuthenticationCodeTypeSmsPhrase)>(&self, fnc: F) -> &Self {
    if let AuthenticationCodeType::SmsPhrase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sms_word<F: FnOnce(&AuthenticationCodeTypeSmsWord)>(&self, fnc: F) -> &Self {
    if let AuthenticationCodeType::SmsWord(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_telegram_message<F: FnOnce(&AuthenticationCodeTypeTelegramMessage)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let AuthenticationCodeType::TelegramMessage(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_call(&self) -> Option<&AuthenticationCodeTypeCall> {
    if let AuthenticationCodeType::Call(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_firebase_android(&self) -> Option<&AuthenticationCodeTypeFirebaseAndroid> {
    if let AuthenticationCodeType::FirebaseAndroid(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_firebase_ios(&self) -> Option<&AuthenticationCodeTypeFirebaseIos> {
    if let AuthenticationCodeType::FirebaseIos(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_flash_call(&self) -> Option<&AuthenticationCodeTypeFlashCall> {
    if let AuthenticationCodeType::FlashCall(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fragment(&self) -> Option<&AuthenticationCodeTypeFragment> {
    if let AuthenticationCodeType::Fragment(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_missed_call(&self) -> Option<&AuthenticationCodeTypeMissedCall> {
    if let AuthenticationCodeType::MissedCall(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sms(&self) -> Option<&AuthenticationCodeTypeSms> {
    if let AuthenticationCodeType::Sms(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sms_phrase(&self) -> Option<&AuthenticationCodeTypeSmsPhrase> {
    if let AuthenticationCodeType::SmsPhrase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sms_word(&self) -> Option<&AuthenticationCodeTypeSmsWord> {
    if let AuthenticationCodeType::SmsWord(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_telegram_message(&self) -> Option<&AuthenticationCodeTypeTelegramMessage> {
    if let AuthenticationCodeType::TelegramMessage(t) = self {
      return Some(t);
    }
    None
  }

  pub fn call<T: AsRef<AuthenticationCodeTypeCall>>(t: T) -> Self {
    AuthenticationCodeType::Call(t.as_ref().clone())
  }

  pub fn firebase_android<T: AsRef<AuthenticationCodeTypeFirebaseAndroid>>(t: T) -> Self {
    AuthenticationCodeType::FirebaseAndroid(t.as_ref().clone())
  }

  pub fn firebase_ios<T: AsRef<AuthenticationCodeTypeFirebaseIos>>(t: T) -> Self {
    AuthenticationCodeType::FirebaseIos(t.as_ref().clone())
  }

  pub fn flash_call<T: AsRef<AuthenticationCodeTypeFlashCall>>(t: T) -> Self {
    AuthenticationCodeType::FlashCall(t.as_ref().clone())
  }

  pub fn fragment<T: AsRef<AuthenticationCodeTypeFragment>>(t: T) -> Self {
    AuthenticationCodeType::Fragment(t.as_ref().clone())
  }

  pub fn missed_call<T: AsRef<AuthenticationCodeTypeMissedCall>>(t: T) -> Self {
    AuthenticationCodeType::MissedCall(t.as_ref().clone())
  }

  pub fn sms<T: AsRef<AuthenticationCodeTypeSms>>(t: T) -> Self {
    AuthenticationCodeType::Sms(t.as_ref().clone())
  }

  pub fn sms_phrase<T: AsRef<AuthenticationCodeTypeSmsPhrase>>(t: T) -> Self {
    AuthenticationCodeType::SmsPhrase(t.as_ref().clone())
  }

  pub fn sms_word<T: AsRef<AuthenticationCodeTypeSmsWord>>(t: T) -> Self {
    AuthenticationCodeType::SmsWord(t.as_ref().clone())
  }

  pub fn telegram_message<T: AsRef<AuthenticationCodeTypeTelegramMessage>>(t: T) -> Self {
    AuthenticationCodeType::TelegramMessage(t.as_ref().clone())
  }
}

impl AsRef<AuthenticationCodeType> for AuthenticationCodeType {
  fn as_ref(&self) -> &AuthenticationCodeType {
    self
  }
}

/// A digit-only authentication code is delivered via a phone call to the specified phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Length of the code
  length: i64,
}

impl RObject for AuthenticationCodeTypeCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeCall"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeCall {}

impl AuthenticationCodeTypeCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeCallBuilder {
    let inner = AuthenticationCodeTypeCall {
      td_name: "authenticationCodeTypeCall".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeCallBuilder { inner }
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeCallBuilder {
  inner: AuthenticationCodeTypeCall,
}

impl RTDAuthenticationCodeTypeCallBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeCall {
    self.inner.clone()
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }
}

impl AsRef<AuthenticationCodeTypeCall> for AuthenticationCodeTypeCall {
  fn as_ref(&self) -> &AuthenticationCodeTypeCall {
    self
  }
}

impl AsRef<AuthenticationCodeTypeCall> for RTDAuthenticationCodeTypeCallBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeCall {
    &self.inner
  }
}

/// A digit-only authentication code is delivered via Firebase Authentication to the official Android application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFirebaseAndroid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Parameters to be used for device verification
  device_verification_parameters: FirebaseDeviceVerificationParameters,
  /// Length of the code
  length: i64,
}

impl RObject for AuthenticationCodeTypeFirebaseAndroid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeFirebaseAndroid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeFirebaseAndroid {}

impl AuthenticationCodeTypeFirebaseAndroid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeFirebaseAndroidBuilder {
    let inner = AuthenticationCodeTypeFirebaseAndroid {
      td_name: "authenticationCodeTypeFirebaseAndroid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeFirebaseAndroidBuilder { inner }
  }

  pub fn device_verification_parameters(&self) -> &FirebaseDeviceVerificationParameters {
    &self.device_verification_parameters
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeFirebaseAndroidBuilder {
  inner: AuthenticationCodeTypeFirebaseAndroid,
}

impl RTDAuthenticationCodeTypeFirebaseAndroidBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeFirebaseAndroid {
    self.inner.clone()
  }

  pub fn device_verification_parameters<T: AsRef<FirebaseDeviceVerificationParameters>>(
    &mut self,
    device_verification_parameters: T,
  ) -> &mut Self {
    self.inner.device_verification_parameters = device_verification_parameters.as_ref().clone();
    self
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }
}

impl AsRef<AuthenticationCodeTypeFirebaseAndroid> for AuthenticationCodeTypeFirebaseAndroid {
  fn as_ref(&self) -> &AuthenticationCodeTypeFirebaseAndroid {
    self
  }
}

impl AsRef<AuthenticationCodeTypeFirebaseAndroid>
  for RTDAuthenticationCodeTypeFirebaseAndroidBuilder
{
  fn as_ref(&self) -> &AuthenticationCodeTypeFirebaseAndroid {
    &self.inner
  }
}

/// A digit-only authentication code is delivered via Firebase Authentication to the official iOS application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFirebaseIos {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Receipt of successful application token validation to compare with receipt from push notification
  receipt: String,
  /// Time after the next authentication method is expected to be used if verification push notification isn't received, in seconds
  push_timeout: i64,
  /// Length of the code
  length: i64,
}

impl RObject for AuthenticationCodeTypeFirebaseIos {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeFirebaseIos"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeFirebaseIos {}

impl AuthenticationCodeTypeFirebaseIos {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeFirebaseIosBuilder {
    let inner = AuthenticationCodeTypeFirebaseIos {
      td_name: "authenticationCodeTypeFirebaseIos".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeFirebaseIosBuilder { inner }
  }

  pub fn receipt(&self) -> &String {
    &self.receipt
  }

  pub fn push_timeout(&self) -> i64 {
    self.push_timeout
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeFirebaseIosBuilder {
  inner: AuthenticationCodeTypeFirebaseIos,
}

impl RTDAuthenticationCodeTypeFirebaseIosBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeFirebaseIos {
    self.inner.clone()
  }

  pub fn receipt<T: AsRef<str>>(&mut self, receipt: T) -> &mut Self {
    self.inner.receipt = receipt.as_ref().to_string();
    self
  }

  pub fn push_timeout(&mut self, push_timeout: i64) -> &mut Self {
    self.inner.push_timeout = push_timeout;
    self
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }
}

impl AsRef<AuthenticationCodeTypeFirebaseIos> for AuthenticationCodeTypeFirebaseIos {
  fn as_ref(&self) -> &AuthenticationCodeTypeFirebaseIos {
    self
  }
}

impl AsRef<AuthenticationCodeTypeFirebaseIos> for RTDAuthenticationCodeTypeFirebaseIosBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeFirebaseIos {
    &self.inner
  }
}

/// An authentication code is delivered by an immediately canceled call to the specified phone number. The phone number that calls is the code that must be entered automatically
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFlashCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pattern of the phone number from which the call will be made
  pattern: String,
}

impl RObject for AuthenticationCodeTypeFlashCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeFlashCall"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeFlashCall {}

impl AuthenticationCodeTypeFlashCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeFlashCallBuilder {
    let inner = AuthenticationCodeTypeFlashCall {
      td_name: "authenticationCodeTypeFlashCall".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeFlashCallBuilder { inner }
  }

  pub fn pattern(&self) -> &String {
    &self.pattern
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeFlashCallBuilder {
  inner: AuthenticationCodeTypeFlashCall,
}

impl RTDAuthenticationCodeTypeFlashCallBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeFlashCall {
    self.inner.clone()
  }

  pub fn pattern<T: AsRef<str>>(&mut self, pattern: T) -> &mut Self {
    self.inner.pattern = pattern.as_ref().to_string();
    self
  }
}

impl AsRef<AuthenticationCodeTypeFlashCall> for AuthenticationCodeTypeFlashCall {
  fn as_ref(&self) -> &AuthenticationCodeTypeFlashCall {
    self
  }
}

impl AsRef<AuthenticationCodeTypeFlashCall> for RTDAuthenticationCodeTypeFlashCallBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeFlashCall {
    &self.inner
  }
}

/// A digit-only authentication code is delivered to https://fragment.com. The user must be logged in there via a wallet owning the phone number's NFT
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFragment {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL to open to receive the code
  url: String,
  /// Length of the code
  length: i64,
}

impl RObject for AuthenticationCodeTypeFragment {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeFragment"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeFragment {}

impl AuthenticationCodeTypeFragment {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeFragmentBuilder {
    let inner = AuthenticationCodeTypeFragment {
      td_name: "authenticationCodeTypeFragment".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeFragmentBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeFragmentBuilder {
  inner: AuthenticationCodeTypeFragment,
}

impl RTDAuthenticationCodeTypeFragmentBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeFragment {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }
}

impl AsRef<AuthenticationCodeTypeFragment> for AuthenticationCodeTypeFragment {
  fn as_ref(&self) -> &AuthenticationCodeTypeFragment {
    self
  }
}

impl AsRef<AuthenticationCodeTypeFragment> for RTDAuthenticationCodeTypeFragmentBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeFragment {
    &self.inner
  }
}

/// An authentication code is delivered by an immediately canceled call to the specified phone number. The last digits of the phone number that calls are the code that must be entered manually by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeMissedCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Prefix of the phone number from which the call will be made
  phone_number_prefix: String,
  /// Number of digits in the code, excluding the prefix
  length: i64,
}

impl RObject for AuthenticationCodeTypeMissedCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeMissedCall"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeMissedCall {}

impl AuthenticationCodeTypeMissedCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeMissedCallBuilder {
    let inner = AuthenticationCodeTypeMissedCall {
      td_name: "authenticationCodeTypeMissedCall".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeMissedCallBuilder { inner }
  }

  pub fn phone_number_prefix(&self) -> &String {
    &self.phone_number_prefix
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeMissedCallBuilder {
  inner: AuthenticationCodeTypeMissedCall,
}

impl RTDAuthenticationCodeTypeMissedCallBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeMissedCall {
    self.inner.clone()
  }

  pub fn phone_number_prefix<T: AsRef<str>>(&mut self, phone_number_prefix: T) -> &mut Self {
    self.inner.phone_number_prefix = phone_number_prefix.as_ref().to_string();
    self
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }
}

impl AsRef<AuthenticationCodeTypeMissedCall> for AuthenticationCodeTypeMissedCall {
  fn as_ref(&self) -> &AuthenticationCodeTypeMissedCall {
    self
  }
}

impl AsRef<AuthenticationCodeTypeMissedCall> for RTDAuthenticationCodeTypeMissedCallBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeMissedCall {
    &self.inner
  }
}

/// A digit-only authentication code is delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeSms {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Length of the code
  length: i64,
}

impl RObject for AuthenticationCodeTypeSms {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeSms"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeSms {}

impl AuthenticationCodeTypeSms {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeSmsBuilder {
    let inner = AuthenticationCodeTypeSms {
      td_name: "authenticationCodeTypeSms".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeSmsBuilder { inner }
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeSmsBuilder {
  inner: AuthenticationCodeTypeSms,
}

impl RTDAuthenticationCodeTypeSmsBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeSms {
    self.inner.clone()
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }
}

impl AsRef<AuthenticationCodeTypeSms> for AuthenticationCodeTypeSms {
  fn as_ref(&self) -> &AuthenticationCodeTypeSms {
    self
  }
}

impl AsRef<AuthenticationCodeTypeSms> for RTDAuthenticationCodeTypeSmsBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeSms {
    &self.inner
  }
}

/// An authentication code is a phrase from multiple words delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeSmsPhrase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The first word of the phrase if known
  first_word: String,
}

impl RObject for AuthenticationCodeTypeSmsPhrase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeSmsPhrase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeSmsPhrase {}

impl AuthenticationCodeTypeSmsPhrase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeSmsPhraseBuilder {
    let inner = AuthenticationCodeTypeSmsPhrase {
      td_name: "authenticationCodeTypeSmsPhrase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeSmsPhraseBuilder { inner }
  }

  pub fn first_word(&self) -> &String {
    &self.first_word
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeSmsPhraseBuilder {
  inner: AuthenticationCodeTypeSmsPhrase,
}

impl RTDAuthenticationCodeTypeSmsPhraseBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeSmsPhrase {
    self.inner.clone()
  }

  pub fn first_word<T: AsRef<str>>(&mut self, first_word: T) -> &mut Self {
    self.inner.first_word = first_word.as_ref().to_string();
    self
  }
}

impl AsRef<AuthenticationCodeTypeSmsPhrase> for AuthenticationCodeTypeSmsPhrase {
  fn as_ref(&self) -> &AuthenticationCodeTypeSmsPhrase {
    self
  }
}

impl AsRef<AuthenticationCodeTypeSmsPhrase> for RTDAuthenticationCodeTypeSmsPhraseBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeSmsPhrase {
    &self.inner
  }
}

/// An authentication code is a word delivered via an SMS message to the specified phone number; non-official applications may not receive this type of code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeSmsWord {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The first letters of the word if known
  first_letter: String,
}

impl RObject for AuthenticationCodeTypeSmsWord {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeSmsWord"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeSmsWord {}

impl AuthenticationCodeTypeSmsWord {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeSmsWordBuilder {
    let inner = AuthenticationCodeTypeSmsWord {
      td_name: "authenticationCodeTypeSmsWord".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeSmsWordBuilder { inner }
  }

  pub fn first_letter(&self) -> &String {
    &self.first_letter
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeSmsWordBuilder {
  inner: AuthenticationCodeTypeSmsWord,
}

impl RTDAuthenticationCodeTypeSmsWordBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeSmsWord {
    self.inner.clone()
  }

  pub fn first_letter<T: AsRef<str>>(&mut self, first_letter: T) -> &mut Self {
    self.inner.first_letter = first_letter.as_ref().to_string();
    self
  }
}

impl AsRef<AuthenticationCodeTypeSmsWord> for AuthenticationCodeTypeSmsWord {
  fn as_ref(&self) -> &AuthenticationCodeTypeSmsWord {
    self
  }
}

impl AsRef<AuthenticationCodeTypeSmsWord> for RTDAuthenticationCodeTypeSmsWordBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeSmsWord {
    &self.inner
  }
}

/// A digit-only authentication code is delivered via a private Telegram message, which can be viewed from another active session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeTelegramMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Length of the code
  length: i64,
}

impl RObject for AuthenticationCodeTypeTelegramMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "authenticationCodeTypeTelegramMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDAuthenticationCodeType for AuthenticationCodeTypeTelegramMessage {}

impl AuthenticationCodeTypeTelegramMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuthenticationCodeTypeTelegramMessageBuilder {
    let inner = AuthenticationCodeTypeTelegramMessage {
      td_name: "authenticationCodeTypeTelegramMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuthenticationCodeTypeTelegramMessageBuilder { inner }
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeTelegramMessageBuilder {
  inner: AuthenticationCodeTypeTelegramMessage,
}

impl RTDAuthenticationCodeTypeTelegramMessageBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeTelegramMessage {
    self.inner.clone()
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }
}

impl AsRef<AuthenticationCodeTypeTelegramMessage> for AuthenticationCodeTypeTelegramMessage {
  fn as_ref(&self) -> &AuthenticationCodeTypeTelegramMessage {
    self
  }
}

impl AsRef<AuthenticationCodeTypeTelegramMessage>
  for RTDAuthenticationCodeTypeTelegramMessageBuilder
{
  fn as_ref(&self) -> &AuthenticationCodeTypeTelegramMessage {
    &self.inner
  }
}
