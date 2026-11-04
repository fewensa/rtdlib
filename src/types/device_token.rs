use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a data needed to subscribe for push notifications through registerDevice method.
pub trait TDDeviceToken: Debug + RObject {}

/// Represents a data needed to subscribe for push notifications through registerDevice method.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DeviceToken {
  #[doc(hidden)]
  _Default(()),
  /// A token for Apple Push Notification service
  ApplePush(DeviceTokenApplePush),
  /// A token for Apple Push Notification service VoIP notifications
  ApplePushVoIP(DeviceTokenApplePushVoIP),
  /// A token for BlackBerry Push Service
  BlackBerryPush(DeviceTokenBlackBerryPush),
  /// To use specific push notification service, the correct application platform must be specified and a valid server authentication data must be uploaded at https://my.telegram.org A token for Firebase Cloud Messaging
  FirebaseCloudMessaging(DeviceTokenFirebaseCloudMessaging),
  /// A token for HUAWEI Push Service
  HuaweiPush(DeviceTokenHuaweiPush),
  /// A token for Microsoft Push Notification Service
  MicrosoftPush(DeviceTokenMicrosoftPush),
  /// A token for Microsoft Push Notification Service VoIP channel
  MicrosoftPushVoIP(DeviceTokenMicrosoftPushVoIP),
  /// A token for Simple Push API for Firefox OS
  SimplePush(DeviceTokenSimplePush),
  /// A token for Tizen Push Service
  TizenPush(DeviceTokenTizenPush),
  /// A token for Ubuntu Push Client service
  UbuntuPush(DeviceTokenUbuntuPush),
  /// A token for web Push API
  WebPush(DeviceTokenWebPush),
  /// A token for Windows Push Notification Services
  WindowsPush(DeviceTokenWindowsPush),
}

impl Default for DeviceToken {
  fn default() -> Self {
    DeviceToken::_Default(())
  }
}

impl<'de> Deserialize<'de> for DeviceToken {
  fn deserialize<D>(deserializer: D) -> Result<DeviceToken, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      DeviceToken,
      (deviceTokenApplePush, ApplePush);
      (deviceTokenApplePushVoIP, ApplePushVoIP);
      (deviceTokenBlackBerryPush, BlackBerryPush);
      (deviceTokenFirebaseCloudMessaging, FirebaseCloudMessaging);
      (deviceTokenHuaweiPush, HuaweiPush);
      (deviceTokenMicrosoftPush, MicrosoftPush);
      (deviceTokenMicrosoftPushVoIP, MicrosoftPushVoIP);
      (deviceTokenSimplePush, SimplePush);
      (deviceTokenTizenPush, TizenPush);
      (deviceTokenUbuntuPush, UbuntuPush);
      (deviceTokenWebPush, WebPush);
      (deviceTokenWindowsPush, WindowsPush);

    )(deserializer)
  }
}

impl RObject for DeviceToken {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      DeviceToken::ApplePush(t) => t.td_name(),
      DeviceToken::ApplePushVoIP(t) => t.td_name(),
      DeviceToken::BlackBerryPush(t) => t.td_name(),
      DeviceToken::FirebaseCloudMessaging(t) => t.td_name(),
      DeviceToken::HuaweiPush(t) => t.td_name(),
      DeviceToken::MicrosoftPush(t) => t.td_name(),
      DeviceToken::MicrosoftPushVoIP(t) => t.td_name(),
      DeviceToken::SimplePush(t) => t.td_name(),
      DeviceToken::TizenPush(t) => t.td_name(),
      DeviceToken::UbuntuPush(t) => t.td_name(),
      DeviceToken::WebPush(t) => t.td_name(),
      DeviceToken::WindowsPush(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      DeviceToken::ApplePush(t) => t.extra(),
      DeviceToken::ApplePushVoIP(t) => t.extra(),
      DeviceToken::BlackBerryPush(t) => t.extra(),
      DeviceToken::FirebaseCloudMessaging(t) => t.extra(),
      DeviceToken::HuaweiPush(t) => t.extra(),
      DeviceToken::MicrosoftPush(t) => t.extra(),
      DeviceToken::MicrosoftPushVoIP(t) => t.extra(),
      DeviceToken::SimplePush(t) => t.extra(),
      DeviceToken::TizenPush(t) => t.extra(),
      DeviceToken::UbuntuPush(t) => t.extra(),
      DeviceToken::WebPush(t) => t.extra(),
      DeviceToken::WindowsPush(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DeviceToken {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, DeviceToken::_Default(_))
  }

  pub fn is_apple_push(&self) -> bool {
    matches!(self, DeviceToken::ApplePush(_))
  }
  pub fn is_apple_push_vo_i_p(&self) -> bool {
    matches!(self, DeviceToken::ApplePushVoIP(_))
  }
  pub fn is_black_berry_push(&self) -> bool {
    matches!(self, DeviceToken::BlackBerryPush(_))
  }
  pub fn is_firebase_cloud_messaging(&self) -> bool {
    matches!(self, DeviceToken::FirebaseCloudMessaging(_))
  }
  pub fn is_huawei_push(&self) -> bool {
    matches!(self, DeviceToken::HuaweiPush(_))
  }
  pub fn is_microsoft_push(&self) -> bool {
    matches!(self, DeviceToken::MicrosoftPush(_))
  }
  pub fn is_microsoft_push_vo_i_p(&self) -> bool {
    matches!(self, DeviceToken::MicrosoftPushVoIP(_))
  }
  pub fn is_simple_push(&self) -> bool {
    matches!(self, DeviceToken::SimplePush(_))
  }
  pub fn is_tizen_push(&self) -> bool {
    matches!(self, DeviceToken::TizenPush(_))
  }
  pub fn is_ubuntu_push(&self) -> bool {
    matches!(self, DeviceToken::UbuntuPush(_))
  }
  pub fn is_web_push(&self) -> bool {
    matches!(self, DeviceToken::WebPush(_))
  }
  pub fn is_windows_push(&self) -> bool {
    matches!(self, DeviceToken::WindowsPush(_))
  }

  pub fn on_apple_push<F: FnOnce(&DeviceTokenApplePush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::ApplePush(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_apple_push_vo_i_p<F: FnOnce(&DeviceTokenApplePushVoIP)>(&self, fnc: F) -> &Self {
    if let DeviceToken::ApplePushVoIP(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_black_berry_push<F: FnOnce(&DeviceTokenBlackBerryPush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::BlackBerryPush(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_firebase_cloud_messaging<F: FnOnce(&DeviceTokenFirebaseCloudMessaging)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let DeviceToken::FirebaseCloudMessaging(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_huawei_push<F: FnOnce(&DeviceTokenHuaweiPush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::HuaweiPush(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_microsoft_push<F: FnOnce(&DeviceTokenMicrosoftPush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::MicrosoftPush(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_microsoft_push_vo_i_p<F: FnOnce(&DeviceTokenMicrosoftPushVoIP)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let DeviceToken::MicrosoftPushVoIP(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_simple_push<F: FnOnce(&DeviceTokenSimplePush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::SimplePush(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_tizen_push<F: FnOnce(&DeviceTokenTizenPush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::TizenPush(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ubuntu_push<F: FnOnce(&DeviceTokenUbuntuPush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::UbuntuPush(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_push<F: FnOnce(&DeviceTokenWebPush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::WebPush(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_windows_push<F: FnOnce(&DeviceTokenWindowsPush)>(&self, fnc: F) -> &Self {
    if let DeviceToken::WindowsPush(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_apple_push(&self) -> Option<&DeviceTokenApplePush> {
    if let DeviceToken::ApplePush(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_apple_push_vo_i_p(&self) -> Option<&DeviceTokenApplePushVoIP> {
    if let DeviceToken::ApplePushVoIP(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_black_berry_push(&self) -> Option<&DeviceTokenBlackBerryPush> {
    if let DeviceToken::BlackBerryPush(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_firebase_cloud_messaging(&self) -> Option<&DeviceTokenFirebaseCloudMessaging> {
    if let DeviceToken::FirebaseCloudMessaging(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_huawei_push(&self) -> Option<&DeviceTokenHuaweiPush> {
    if let DeviceToken::HuaweiPush(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_microsoft_push(&self) -> Option<&DeviceTokenMicrosoftPush> {
    if let DeviceToken::MicrosoftPush(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_microsoft_push_vo_i_p(&self) -> Option<&DeviceTokenMicrosoftPushVoIP> {
    if let DeviceToken::MicrosoftPushVoIP(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_simple_push(&self) -> Option<&DeviceTokenSimplePush> {
    if let DeviceToken::SimplePush(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_tizen_push(&self) -> Option<&DeviceTokenTizenPush> {
    if let DeviceToken::TizenPush(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ubuntu_push(&self) -> Option<&DeviceTokenUbuntuPush> {
    if let DeviceToken::UbuntuPush(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_push(&self) -> Option<&DeviceTokenWebPush> {
    if let DeviceToken::WebPush(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_windows_push(&self) -> Option<&DeviceTokenWindowsPush> {
    if let DeviceToken::WindowsPush(t) = self {
      return Some(t);
    }
    None
  }

  pub fn apple_push<T: AsRef<DeviceTokenApplePush>>(t: T) -> Self {
    DeviceToken::ApplePush(t.as_ref().clone())
  }

  pub fn apple_push_vo_i_p<T: AsRef<DeviceTokenApplePushVoIP>>(t: T) -> Self {
    DeviceToken::ApplePushVoIP(t.as_ref().clone())
  }

  pub fn black_berry_push<T: AsRef<DeviceTokenBlackBerryPush>>(t: T) -> Self {
    DeviceToken::BlackBerryPush(t.as_ref().clone())
  }

  pub fn firebase_cloud_messaging<T: AsRef<DeviceTokenFirebaseCloudMessaging>>(t: T) -> Self {
    DeviceToken::FirebaseCloudMessaging(t.as_ref().clone())
  }

  pub fn huawei_push<T: AsRef<DeviceTokenHuaweiPush>>(t: T) -> Self {
    DeviceToken::HuaweiPush(t.as_ref().clone())
  }

  pub fn microsoft_push<T: AsRef<DeviceTokenMicrosoftPush>>(t: T) -> Self {
    DeviceToken::MicrosoftPush(t.as_ref().clone())
  }

  pub fn microsoft_push_vo_i_p<T: AsRef<DeviceTokenMicrosoftPushVoIP>>(t: T) -> Self {
    DeviceToken::MicrosoftPushVoIP(t.as_ref().clone())
  }

  pub fn simple_push<T: AsRef<DeviceTokenSimplePush>>(t: T) -> Self {
    DeviceToken::SimplePush(t.as_ref().clone())
  }

  pub fn tizen_push<T: AsRef<DeviceTokenTizenPush>>(t: T) -> Self {
    DeviceToken::TizenPush(t.as_ref().clone())
  }

  pub fn ubuntu_push<T: AsRef<DeviceTokenUbuntuPush>>(t: T) -> Self {
    DeviceToken::UbuntuPush(t.as_ref().clone())
  }

  pub fn web_push<T: AsRef<DeviceTokenWebPush>>(t: T) -> Self {
    DeviceToken::WebPush(t.as_ref().clone())
  }

  pub fn windows_push<T: AsRef<DeviceTokenWindowsPush>>(t: T) -> Self {
    DeviceToken::WindowsPush(t.as_ref().clone())
  }
}

impl AsRef<DeviceToken> for DeviceToken {
  fn as_ref(&self) -> &DeviceToken {
    self
  }
}

/// A token for Apple Push Notification service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenApplePush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Device token; may be empty to deregister a device
  device_token: String,
  /// True, if App Sandbox is enabled
  is_app_sandbox: bool,
}

impl RObject for DeviceTokenApplePush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenApplePush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenApplePush {}

impl DeviceTokenApplePush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenApplePushBuilder {
    let inner = DeviceTokenApplePush {
      td_name: "deviceTokenApplePush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenApplePushBuilder { inner }
  }

  pub fn device_token(&self) -> &String {
    &self.device_token
  }

  pub fn is_app_sandbox(&self) -> bool {
    self.is_app_sandbox
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenApplePushBuilder {
  inner: DeviceTokenApplePush,
}

impl RTDDeviceTokenApplePushBuilder {
  pub fn build(&self) -> DeviceTokenApplePush {
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

impl AsRef<DeviceTokenApplePush> for DeviceTokenApplePush {
  fn as_ref(&self) -> &DeviceTokenApplePush {
    self
  }
}

impl AsRef<DeviceTokenApplePush> for RTDDeviceTokenApplePushBuilder {
  fn as_ref(&self) -> &DeviceTokenApplePush {
    &self.inner
  }
}

/// A token for Apple Push Notification service VoIP notifications
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenApplePushVoIP {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Device token; may be empty to deregister a device
  device_token: String,
  /// True, if App Sandbox is enabled
  is_app_sandbox: bool,
  /// True, if push notifications must be additionally encrypted
  encrypt: bool,
}

impl RObject for DeviceTokenApplePushVoIP {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenApplePushVoIP"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenApplePushVoIP {}

impl DeviceTokenApplePushVoIP {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenApplePushVoIPBuilder {
    let inner = DeviceTokenApplePushVoIP {
      td_name: "deviceTokenApplePushVoIP".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenApplePushVoIPBuilder { inner }
  }

  pub fn device_token(&self) -> &String {
    &self.device_token
  }

  pub fn is_app_sandbox(&self) -> bool {
    self.is_app_sandbox
  }

  pub fn encrypt(&self) -> bool {
    self.encrypt
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenApplePushVoIPBuilder {
  inner: DeviceTokenApplePushVoIP,
}

impl RTDDeviceTokenApplePushVoIPBuilder {
  pub fn build(&self) -> DeviceTokenApplePushVoIP {
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

  pub fn encrypt(&mut self, encrypt: bool) -> &mut Self {
    self.inner.encrypt = encrypt;
    self
  }
}

impl AsRef<DeviceTokenApplePushVoIP> for DeviceTokenApplePushVoIP {
  fn as_ref(&self) -> &DeviceTokenApplePushVoIP {
    self
  }
}

impl AsRef<DeviceTokenApplePushVoIP> for RTDDeviceTokenApplePushVoIPBuilder {
  fn as_ref(&self) -> &DeviceTokenApplePushVoIP {
    &self.inner
  }
}

/// A token for BlackBerry Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenBlackBerryPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Token; may be empty to deregister a device
  token: String,
}

impl RObject for DeviceTokenBlackBerryPush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenBlackBerryPush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenBlackBerryPush {}

impl DeviceTokenBlackBerryPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenBlackBerryPushBuilder {
    let inner = DeviceTokenBlackBerryPush {
      td_name: "deviceTokenBlackBerryPush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenBlackBerryPushBuilder { inner }
  }

  pub fn token(&self) -> &String {
    &self.token
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenBlackBerryPushBuilder {
  inner: DeviceTokenBlackBerryPush,
}

impl RTDDeviceTokenBlackBerryPushBuilder {
  pub fn build(&self) -> DeviceTokenBlackBerryPush {
    self.inner.clone()
  }

  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }
}

impl AsRef<DeviceTokenBlackBerryPush> for DeviceTokenBlackBerryPush {
  fn as_ref(&self) -> &DeviceTokenBlackBerryPush {
    self
  }
}

impl AsRef<DeviceTokenBlackBerryPush> for RTDDeviceTokenBlackBerryPushBuilder {
  fn as_ref(&self) -> &DeviceTokenBlackBerryPush {
    &self.inner
  }
}

/// To use specific push notification service, the correct application platform must be specified and a valid server authentication data must be uploaded at https://my.telegram.org A token for Firebase Cloud Messaging
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenFirebaseCloudMessaging {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Device registration token; may be empty to deregister a device
  token: String,
  /// True, if push notifications must be additionally encrypted
  encrypt: bool,
}

impl RObject for DeviceTokenFirebaseCloudMessaging {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenFirebaseCloudMessaging"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenFirebaseCloudMessaging {}

impl DeviceTokenFirebaseCloudMessaging {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenFirebaseCloudMessagingBuilder {
    let inner = DeviceTokenFirebaseCloudMessaging {
      td_name: "deviceTokenFirebaseCloudMessaging".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenFirebaseCloudMessagingBuilder { inner }
  }

  pub fn token(&self) -> &String {
    &self.token
  }

  pub fn encrypt(&self) -> bool {
    self.encrypt
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenFirebaseCloudMessagingBuilder {
  inner: DeviceTokenFirebaseCloudMessaging,
}

impl RTDDeviceTokenFirebaseCloudMessagingBuilder {
  pub fn build(&self) -> DeviceTokenFirebaseCloudMessaging {
    self.inner.clone()
  }

  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

  pub fn encrypt(&mut self, encrypt: bool) -> &mut Self {
    self.inner.encrypt = encrypt;
    self
  }
}

impl AsRef<DeviceTokenFirebaseCloudMessaging> for DeviceTokenFirebaseCloudMessaging {
  fn as_ref(&self) -> &DeviceTokenFirebaseCloudMessaging {
    self
  }
}

impl AsRef<DeviceTokenFirebaseCloudMessaging> for RTDDeviceTokenFirebaseCloudMessagingBuilder {
  fn as_ref(&self) -> &DeviceTokenFirebaseCloudMessaging {
    &self.inner
  }
}

/// A token for HUAWEI Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenHuaweiPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Device registration token; may be empty to deregister a device
  token: String,
  /// True, if push notifications must be additionally encrypted
  encrypt: bool,
}

impl RObject for DeviceTokenHuaweiPush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenHuaweiPush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenHuaweiPush {}

impl DeviceTokenHuaweiPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenHuaweiPushBuilder {
    let inner = DeviceTokenHuaweiPush {
      td_name: "deviceTokenHuaweiPush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenHuaweiPushBuilder { inner }
  }

  pub fn token(&self) -> &String {
    &self.token
  }

  pub fn encrypt(&self) -> bool {
    self.encrypt
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenHuaweiPushBuilder {
  inner: DeviceTokenHuaweiPush,
}

impl RTDDeviceTokenHuaweiPushBuilder {
  pub fn build(&self) -> DeviceTokenHuaweiPush {
    self.inner.clone()
  }

  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

  pub fn encrypt(&mut self, encrypt: bool) -> &mut Self {
    self.inner.encrypt = encrypt;
    self
  }
}

impl AsRef<DeviceTokenHuaweiPush> for DeviceTokenHuaweiPush {
  fn as_ref(&self) -> &DeviceTokenHuaweiPush {
    self
  }
}

impl AsRef<DeviceTokenHuaweiPush> for RTDDeviceTokenHuaweiPushBuilder {
  fn as_ref(&self) -> &DeviceTokenHuaweiPush {
    &self.inner
  }
}

/// A token for Microsoft Push Notification Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Push notification channel URI; may be empty to deregister a device
  channel_uri: String,
}

impl RObject for DeviceTokenMicrosoftPush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenMicrosoftPush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenMicrosoftPush {}

impl DeviceTokenMicrosoftPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenMicrosoftPushBuilder {
    let inner = DeviceTokenMicrosoftPush {
      td_name: "deviceTokenMicrosoftPush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenMicrosoftPushBuilder { inner }
  }

  pub fn channel_uri(&self) -> &String {
    &self.channel_uri
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenMicrosoftPushBuilder {
  inner: DeviceTokenMicrosoftPush,
}

impl RTDDeviceTokenMicrosoftPushBuilder {
  pub fn build(&self) -> DeviceTokenMicrosoftPush {
    self.inner.clone()
  }

  pub fn channel_uri<T: AsRef<str>>(&mut self, channel_uri: T) -> &mut Self {
    self.inner.channel_uri = channel_uri.as_ref().to_string();
    self
  }
}

impl AsRef<DeviceTokenMicrosoftPush> for DeviceTokenMicrosoftPush {
  fn as_ref(&self) -> &DeviceTokenMicrosoftPush {
    self
  }
}

impl AsRef<DeviceTokenMicrosoftPush> for RTDDeviceTokenMicrosoftPushBuilder {
  fn as_ref(&self) -> &DeviceTokenMicrosoftPush {
    &self.inner
  }
}

/// A token for Microsoft Push Notification Service VoIP channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPushVoIP {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Push notification channel URI; may be empty to deregister a device
  channel_uri: String,
}

impl RObject for DeviceTokenMicrosoftPushVoIP {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenMicrosoftPushVoIP"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenMicrosoftPushVoIP {}

impl DeviceTokenMicrosoftPushVoIP {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenMicrosoftPushVoIPBuilder {
    let inner = DeviceTokenMicrosoftPushVoIP {
      td_name: "deviceTokenMicrosoftPushVoIP".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenMicrosoftPushVoIPBuilder { inner }
  }

  pub fn channel_uri(&self) -> &String {
    &self.channel_uri
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenMicrosoftPushVoIPBuilder {
  inner: DeviceTokenMicrosoftPushVoIP,
}

impl RTDDeviceTokenMicrosoftPushVoIPBuilder {
  pub fn build(&self) -> DeviceTokenMicrosoftPushVoIP {
    self.inner.clone()
  }

  pub fn channel_uri<T: AsRef<str>>(&mut self, channel_uri: T) -> &mut Self {
    self.inner.channel_uri = channel_uri.as_ref().to_string();
    self
  }
}

impl AsRef<DeviceTokenMicrosoftPushVoIP> for DeviceTokenMicrosoftPushVoIP {
  fn as_ref(&self) -> &DeviceTokenMicrosoftPushVoIP {
    self
  }
}

impl AsRef<DeviceTokenMicrosoftPushVoIP> for RTDDeviceTokenMicrosoftPushVoIPBuilder {
  fn as_ref(&self) -> &DeviceTokenMicrosoftPushVoIP {
    &self.inner
  }
}

/// A token for Simple Push API for Firefox OS
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenSimplePush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to deregister a device
  endpoint: String,
}

impl RObject for DeviceTokenSimplePush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenSimplePush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenSimplePush {}

impl DeviceTokenSimplePush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenSimplePushBuilder {
    let inner = DeviceTokenSimplePush {
      td_name: "deviceTokenSimplePush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenSimplePushBuilder { inner }
  }

  pub fn endpoint(&self) -> &String {
    &self.endpoint
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenSimplePushBuilder {
  inner: DeviceTokenSimplePush,
}

impl RTDDeviceTokenSimplePushBuilder {
  pub fn build(&self) -> DeviceTokenSimplePush {
    self.inner.clone()
  }

  pub fn endpoint<T: AsRef<str>>(&mut self, endpoint: T) -> &mut Self {
    self.inner.endpoint = endpoint.as_ref().to_string();
    self
  }
}

impl AsRef<DeviceTokenSimplePush> for DeviceTokenSimplePush {
  fn as_ref(&self) -> &DeviceTokenSimplePush {
    self
  }
}

impl AsRef<DeviceTokenSimplePush> for RTDDeviceTokenSimplePushBuilder {
  fn as_ref(&self) -> &DeviceTokenSimplePush {
    &self.inner
  }
}

/// A token for Tizen Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenTizenPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Push service registration identifier; may be empty to deregister a device
  reg_id: String,
}

impl RObject for DeviceTokenTizenPush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenTizenPush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenTizenPush {}

impl DeviceTokenTizenPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenTizenPushBuilder {
    let inner = DeviceTokenTizenPush {
      td_name: "deviceTokenTizenPush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenTizenPushBuilder { inner }
  }

  pub fn reg_id(&self) -> &String {
    &self.reg_id
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenTizenPushBuilder {
  inner: DeviceTokenTizenPush,
}

impl RTDDeviceTokenTizenPushBuilder {
  pub fn build(&self) -> DeviceTokenTizenPush {
    self.inner.clone()
  }

  pub fn reg_id<T: AsRef<str>>(&mut self, reg_id: T) -> &mut Self {
    self.inner.reg_id = reg_id.as_ref().to_string();
    self
  }
}

impl AsRef<DeviceTokenTizenPush> for DeviceTokenTizenPush {
  fn as_ref(&self) -> &DeviceTokenTizenPush {
    self
  }
}

impl AsRef<DeviceTokenTizenPush> for RTDDeviceTokenTizenPushBuilder {
  fn as_ref(&self) -> &DeviceTokenTizenPush {
    &self.inner
  }
}

/// A token for Ubuntu Push Client service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenUbuntuPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Token; may be empty to deregister a device
  token: String,
}

impl RObject for DeviceTokenUbuntuPush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenUbuntuPush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenUbuntuPush {}

impl DeviceTokenUbuntuPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenUbuntuPushBuilder {
    let inner = DeviceTokenUbuntuPush {
      td_name: "deviceTokenUbuntuPush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenUbuntuPushBuilder { inner }
  }

  pub fn token(&self) -> &String {
    &self.token
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenUbuntuPushBuilder {
  inner: DeviceTokenUbuntuPush,
}

impl RTDDeviceTokenUbuntuPushBuilder {
  pub fn build(&self) -> DeviceTokenUbuntuPush {
    self.inner.clone()
  }

  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }
}

impl AsRef<DeviceTokenUbuntuPush> for DeviceTokenUbuntuPush {
  fn as_ref(&self) -> &DeviceTokenUbuntuPush {
    self
  }
}

impl AsRef<DeviceTokenUbuntuPush> for RTDDeviceTokenUbuntuPushBuilder {
  fn as_ref(&self) -> &DeviceTokenUbuntuPush {
    &self.inner
  }
}

/// A token for web Push API
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenWebPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to deregister a device
  endpoint: String,
  /// Base64url-encoded P-256 elliptic curve Diffie-Hellman public key
  p256dh_base64url: String,
  /// Base64url-encoded authentication secret
  auth_base64url: String,
}

impl RObject for DeviceTokenWebPush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenWebPush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenWebPush {}

impl DeviceTokenWebPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenWebPushBuilder {
    let inner = DeviceTokenWebPush {
      td_name: "deviceTokenWebPush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenWebPushBuilder { inner }
  }

  pub fn endpoint(&self) -> &String {
    &self.endpoint
  }

  pub fn p256dh_base64url(&self) -> &String {
    &self.p256dh_base64url
  }

  pub fn auth_base64url(&self) -> &String {
    &self.auth_base64url
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenWebPushBuilder {
  inner: DeviceTokenWebPush,
}

impl RTDDeviceTokenWebPushBuilder {
  pub fn build(&self) -> DeviceTokenWebPush {
    self.inner.clone()
  }

  pub fn endpoint<T: AsRef<str>>(&mut self, endpoint: T) -> &mut Self {
    self.inner.endpoint = endpoint.as_ref().to_string();
    self
  }

  pub fn p256dh_base64url<T: AsRef<str>>(&mut self, p256dh_base64url: T) -> &mut Self {
    self.inner.p256dh_base64url = p256dh_base64url.as_ref().to_string();
    self
  }

  pub fn auth_base64url<T: AsRef<str>>(&mut self, auth_base64url: T) -> &mut Self {
    self.inner.auth_base64url = auth_base64url.as_ref().to_string();
    self
  }
}

impl AsRef<DeviceTokenWebPush> for DeviceTokenWebPush {
  fn as_ref(&self) -> &DeviceTokenWebPush {
    self
  }
}

impl AsRef<DeviceTokenWebPush> for RTDDeviceTokenWebPushBuilder {
  fn as_ref(&self) -> &DeviceTokenWebPush {
    &self.inner
  }
}

/// A token for Windows Push Notification Services
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenWindowsPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The access token that will be used to send notifications; may be empty to deregister a device
  access_token: String,
}

impl RObject for DeviceTokenWindowsPush {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "deviceTokenWindowsPush"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDeviceToken for DeviceTokenWindowsPush {}

impl DeviceTokenWindowsPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDeviceTokenWindowsPushBuilder {
    let inner = DeviceTokenWindowsPush {
      td_name: "deviceTokenWindowsPush".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDeviceTokenWindowsPushBuilder { inner }
  }

  pub fn access_token(&self) -> &String {
    &self.access_token
  }
}

#[doc(hidden)]
pub struct RTDDeviceTokenWindowsPushBuilder {
  inner: DeviceTokenWindowsPush,
}

impl RTDDeviceTokenWindowsPushBuilder {
  pub fn build(&self) -> DeviceTokenWindowsPush {
    self.inner.clone()
  }

  pub fn access_token<T: AsRef<str>>(&mut self, access_token: T) -> &mut Self {
    self.inner.access_token = access_token.as_ref().to_string();
    self
  }
}

impl AsRef<DeviceTokenWindowsPush> for DeviceTokenWindowsPush {
  fn as_ref(&self) -> &DeviceTokenWindowsPush {
    self
  }
}

impl AsRef<DeviceTokenWindowsPush> for RTDDeviceTokenWindowsPushBuilder {
  fn as_ref(&self) -> &DeviceTokenWindowsPush {
    &self.inner
  }
}
