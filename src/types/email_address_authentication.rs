use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains authentication data for an email address
pub trait TDEmailAddressAuthentication: Debug + RObject {}

/// Contains authentication data for an email address
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EmailAddressAuthentication {
  #[doc(hidden)]
  _Default(()),
  /// An authentication token received through Apple ID
  AppleId(EmailAddressAuthenticationAppleId),
  /// An authentication code delivered to a user's email address
  Code(EmailAddressAuthenticationCode),
  /// An authentication token received through Google ID
  GoogleId(EmailAddressAuthenticationGoogleId),
}

impl Default for EmailAddressAuthentication {
  fn default() -> Self {
    EmailAddressAuthentication::_Default(())
  }
}

impl<'de> Deserialize<'de> for EmailAddressAuthentication {
  fn deserialize<D>(deserializer: D) -> Result<EmailAddressAuthentication, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      EmailAddressAuthentication,
      (emailAddressAuthenticationAppleId, AppleId);
      (emailAddressAuthenticationCode, Code);
      (emailAddressAuthenticationGoogleId, GoogleId);

    )(deserializer)
  }
}

impl RObject for EmailAddressAuthentication {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      EmailAddressAuthentication::AppleId(t) => t.td_name(),
      EmailAddressAuthentication::Code(t) => t.td_name(),
      EmailAddressAuthentication::GoogleId(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      EmailAddressAuthentication::AppleId(t) => t.extra(),
      EmailAddressAuthentication::Code(t) => t.extra(),
      EmailAddressAuthentication::GoogleId(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmailAddressAuthentication {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, EmailAddressAuthentication::_Default(_))
  }

  pub fn is_apple_id(&self) -> bool {
    matches!(self, EmailAddressAuthentication::AppleId(_))
  }
  pub fn is_code(&self) -> bool {
    matches!(self, EmailAddressAuthentication::Code(_))
  }
  pub fn is_google_id(&self) -> bool {
    matches!(self, EmailAddressAuthentication::GoogleId(_))
  }

  pub fn on_apple_id<F: FnOnce(&EmailAddressAuthenticationAppleId)>(&self, fnc: F) -> &Self {
    if let EmailAddressAuthentication::AppleId(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_code<F: FnOnce(&EmailAddressAuthenticationCode)>(&self, fnc: F) -> &Self {
    if let EmailAddressAuthentication::Code(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_google_id<F: FnOnce(&EmailAddressAuthenticationGoogleId)>(&self, fnc: F) -> &Self {
    if let EmailAddressAuthentication::GoogleId(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_apple_id(&self) -> Option<&EmailAddressAuthenticationAppleId> {
    if let EmailAddressAuthentication::AppleId(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_code(&self) -> Option<&EmailAddressAuthenticationCode> {
    if let EmailAddressAuthentication::Code(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_google_id(&self) -> Option<&EmailAddressAuthenticationGoogleId> {
    if let EmailAddressAuthentication::GoogleId(t) = self {
      return Some(t);
    }
    None
  }

  pub fn apple_id<T: AsRef<EmailAddressAuthenticationAppleId>>(t: T) -> Self {
    EmailAddressAuthentication::AppleId(t.as_ref().clone())
  }

  pub fn code<T: AsRef<EmailAddressAuthenticationCode>>(t: T) -> Self {
    EmailAddressAuthentication::Code(t.as_ref().clone())
  }

  pub fn google_id<T: AsRef<EmailAddressAuthenticationGoogleId>>(t: T) -> Self {
    EmailAddressAuthentication::GoogleId(t.as_ref().clone())
  }
}

impl AsRef<EmailAddressAuthentication> for EmailAddressAuthentication {
  fn as_ref(&self) -> &EmailAddressAuthentication {
    self
  }
}

/// An authentication token received through Apple ID
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationAppleId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The token
  token: String,
}

impl RObject for EmailAddressAuthenticationAppleId {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emailAddressAuthenticationAppleId"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmailAddressAuthentication for EmailAddressAuthenticationAppleId {}

impl EmailAddressAuthenticationAppleId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmailAddressAuthenticationAppleIdBuilder {
    let inner = EmailAddressAuthenticationAppleId {
      td_name: "emailAddressAuthenticationAppleId".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmailAddressAuthenticationAppleIdBuilder { inner }
  }

  pub fn token(&self) -> &String {
    &self.token
  }
}

#[doc(hidden)]
pub struct RTDEmailAddressAuthenticationAppleIdBuilder {
  inner: EmailAddressAuthenticationAppleId,
}

impl RTDEmailAddressAuthenticationAppleIdBuilder {
  pub fn build(&self) -> EmailAddressAuthenticationAppleId {
    self.inner.clone()
  }

  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }
}

impl AsRef<EmailAddressAuthenticationAppleId> for EmailAddressAuthenticationAppleId {
  fn as_ref(&self) -> &EmailAddressAuthenticationAppleId {
    self
  }
}

impl AsRef<EmailAddressAuthenticationAppleId> for RTDEmailAddressAuthenticationAppleIdBuilder {
  fn as_ref(&self) -> &EmailAddressAuthenticationAppleId {
    &self.inner
  }
}

/// An authentication code delivered to a user's email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The code
  code: String,
}

impl RObject for EmailAddressAuthenticationCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emailAddressAuthenticationCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmailAddressAuthentication for EmailAddressAuthenticationCode {}

impl EmailAddressAuthenticationCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmailAddressAuthenticationCodeBuilder {
    let inner = EmailAddressAuthenticationCode {
      td_name: "emailAddressAuthenticationCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmailAddressAuthenticationCodeBuilder { inner }
  }

  pub fn code(&self) -> &String {
    &self.code
  }
}

#[doc(hidden)]
pub struct RTDEmailAddressAuthenticationCodeBuilder {
  inner: EmailAddressAuthenticationCode,
}

impl RTDEmailAddressAuthenticationCodeBuilder {
  pub fn build(&self) -> EmailAddressAuthenticationCode {
    self.inner.clone()
  }

  pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
    self.inner.code = code.as_ref().to_string();
    self
  }
}

impl AsRef<EmailAddressAuthenticationCode> for EmailAddressAuthenticationCode {
  fn as_ref(&self) -> &EmailAddressAuthenticationCode {
    self
  }
}

impl AsRef<EmailAddressAuthenticationCode> for RTDEmailAddressAuthenticationCodeBuilder {
  fn as_ref(&self) -> &EmailAddressAuthenticationCode {
    &self.inner
  }
}

/// An authentication token received through Google ID
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationGoogleId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The token
  token: String,
}

impl RObject for EmailAddressAuthenticationGoogleId {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emailAddressAuthenticationGoogleId"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDEmailAddressAuthentication for EmailAddressAuthenticationGoogleId {}

impl EmailAddressAuthenticationGoogleId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmailAddressAuthenticationGoogleIdBuilder {
    let inner = EmailAddressAuthenticationGoogleId {
      td_name: "emailAddressAuthenticationGoogleId".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmailAddressAuthenticationGoogleIdBuilder { inner }
  }

  pub fn token(&self) -> &String {
    &self.token
  }
}

#[doc(hidden)]
pub struct RTDEmailAddressAuthenticationGoogleIdBuilder {
  inner: EmailAddressAuthenticationGoogleId,
}

impl RTDEmailAddressAuthenticationGoogleIdBuilder {
  pub fn build(&self) -> EmailAddressAuthenticationGoogleId {
    self.inner.clone()
  }

  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }
}

impl AsRef<EmailAddressAuthenticationGoogleId> for EmailAddressAuthenticationGoogleId {
  fn as_ref(&self) -> &EmailAddressAuthenticationGoogleId {
    self
  }
}

impl AsRef<EmailAddressAuthenticationGoogleId> for RTDEmailAddressAuthenticationGoogleIdBuilder {
  fn as_ref(&self) -> &EmailAddressAuthenticationGoogleId {
    &self.inner
  }
}
