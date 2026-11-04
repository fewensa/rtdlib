use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains settings for Firebase Authentication in the official applications
pub trait TDFirebaseAuthenticationSettings: Debug + RObject {}

/// Contains settings for Firebase Authentication in the official applications
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FirebaseAuthenticationSettings {
  #[doc(hidden)]
  _Default(()),
  /// Settings for Firebase Authentication in the official Android application
  Android(FirebaseAuthenticationSettingsAndroid),
  /// Settings for Firebase Authentication in the official iOS application
  Ios(FirebaseAuthenticationSettingsIos),
}

impl Default for FirebaseAuthenticationSettings {
  fn default() -> Self {
    FirebaseAuthenticationSettings::_Default(())
  }
}

impl<'de> Deserialize<'de> for FirebaseAuthenticationSettings {
  fn deserialize<D>(deserializer: D) -> Result<FirebaseAuthenticationSettings, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      FirebaseAuthenticationSettings,
      (firebaseAuthenticationSettingsAndroid, Android);
      (firebaseAuthenticationSettingsIos, Ios);

    )(deserializer)
  }
}

impl RObject for FirebaseAuthenticationSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      FirebaseAuthenticationSettings::Android(t) => t.td_name(),
      FirebaseAuthenticationSettings::Ios(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      FirebaseAuthenticationSettings::Android(t) => t.extra(),
      FirebaseAuthenticationSettings::Ios(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FirebaseAuthenticationSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, FirebaseAuthenticationSettings::_Default(_))
  }

  pub fn is_android(&self) -> bool {
    matches!(self, FirebaseAuthenticationSettings::Android(_))
  }
  pub fn is_ios(&self) -> bool {
    matches!(self, FirebaseAuthenticationSettings::Ios(_))
  }

  pub fn on_android<F: FnOnce(&FirebaseAuthenticationSettingsAndroid)>(&self, fnc: F) -> &Self {
    if let FirebaseAuthenticationSettings::Android(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ios<F: FnOnce(&FirebaseAuthenticationSettingsIos)>(&self, fnc: F) -> &Self {
    if let FirebaseAuthenticationSettings::Ios(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_android(&self) -> Option<&FirebaseAuthenticationSettingsAndroid> {
    if let FirebaseAuthenticationSettings::Android(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ios(&self) -> Option<&FirebaseAuthenticationSettingsIos> {
    if let FirebaseAuthenticationSettings::Ios(t) = self {
      return Some(t);
    }
    None
  }

  pub fn android<T: AsRef<FirebaseAuthenticationSettingsAndroid>>(t: T) -> Self {
    FirebaseAuthenticationSettings::Android(t.as_ref().clone())
  }

  pub fn ios<T: AsRef<FirebaseAuthenticationSettingsIos>>(t: T) -> Self {
    FirebaseAuthenticationSettings::Ios(t.as_ref().clone())
  }
}

impl AsRef<FirebaseAuthenticationSettings> for FirebaseAuthenticationSettings {
  fn as_ref(&self) -> &FirebaseAuthenticationSettings {
    self
  }
}

/// Settings for Firebase Authentication in the official Android application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirebaseAuthenticationSettingsAndroid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for FirebaseAuthenticationSettingsAndroid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "firebaseAuthenticationSettingsAndroid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFirebaseAuthenticationSettings for FirebaseAuthenticationSettingsAndroid {}

impl FirebaseAuthenticationSettingsAndroid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFirebaseAuthenticationSettingsAndroidBuilder {
    let inner = FirebaseAuthenticationSettingsAndroid {
      td_name: "firebaseAuthenticationSettingsAndroid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDFirebaseAuthenticationSettingsAndroidBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDFirebaseAuthenticationSettingsAndroidBuilder {
  inner: FirebaseAuthenticationSettingsAndroid,
}

impl RTDFirebaseAuthenticationSettingsAndroidBuilder {
  pub fn build(&self) -> FirebaseAuthenticationSettingsAndroid {
    self.inner.clone()
  }
}

impl AsRef<FirebaseAuthenticationSettingsAndroid> for FirebaseAuthenticationSettingsAndroid {
  fn as_ref(&self) -> &FirebaseAuthenticationSettingsAndroid {
    self
  }
}

impl AsRef<FirebaseAuthenticationSettingsAndroid>
  for RTDFirebaseAuthenticationSettingsAndroidBuilder
{
  fn as_ref(&self) -> &FirebaseAuthenticationSettingsAndroid {
    &self.inner
  }
}

/// Settings for Firebase Authentication in the official iOS application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirebaseAuthenticationSettingsIos {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Device token from Apple Push Notification service
  device_token: String,
  /// True, if App Sandbox is enabled
  is_app_sandbox: bool,
}

impl RObject for FirebaseAuthenticationSettingsIos {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "firebaseAuthenticationSettingsIos"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDFirebaseAuthenticationSettings for FirebaseAuthenticationSettingsIos {}

impl FirebaseAuthenticationSettingsIos {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFirebaseAuthenticationSettingsIosBuilder {
    let inner = FirebaseAuthenticationSettingsIos {
      td_name: "firebaseAuthenticationSettingsIos".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFirebaseAuthenticationSettingsIosBuilder { inner }
  }

  pub fn device_token(&self) -> &String {
    &self.device_token
  }

  pub fn is_app_sandbox(&self) -> bool {
    self.is_app_sandbox
  }
}

#[doc(hidden)]
pub struct RTDFirebaseAuthenticationSettingsIosBuilder {
  inner: FirebaseAuthenticationSettingsIos,
}

impl RTDFirebaseAuthenticationSettingsIosBuilder {
  pub fn build(&self) -> FirebaseAuthenticationSettingsIos {
    self.inner.clone()
  }

  pub fn device_token<T: AsRef<str>>(&mut self, device_token: T) -> &mut Self {
    self.inner.device_token = device_token.as_ref().to_string();
    self
  }

  pub fn is_app_sandbox(&mut self, is_app_sandbox: bool) -> &mut Self {
    self.inner.is_app_sandbox = is_app_sandbox;
    self
  }
}

impl AsRef<FirebaseAuthenticationSettingsIos> for FirebaseAuthenticationSettingsIos {
  fn as_ref(&self) -> &FirebaseAuthenticationSettingsIos {
    self
  }
}

impl AsRef<FirebaseAuthenticationSettingsIos> for RTDFirebaseAuthenticationSettingsIosBuilder {
  fn as_ref(&self) -> &FirebaseAuthenticationSettingsIos {
    &self.inner
  }
}
