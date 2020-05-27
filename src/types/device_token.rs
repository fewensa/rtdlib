
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a data needed to subscribe for push notifications. To use specific push notification service, you must specify the correct application platform and upload valid server authentication data at https://my.telegram.org
pub trait TDDeviceToken: Debug + RObject {}

/// Represents a data needed to subscribe for push notifications. To use specific push notification service, you must specify the correct application platform and upload valid server authentication data at https://my.telegram.org
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DeviceToken {
  #[doc(hidden)] _Default(()),
  /// A token for Apple Push Notification service
  ApplePush(DeviceTokenApplePush),
  /// A token for Apple Push Notification service VoIP notifications
  ApplePushVoIP(DeviceTokenApplePushVoIP),
  /// A token for BlackBerry Push Service
  BlackBerryPush(DeviceTokenBlackBerryPush),
  /// A token for Google Cloud Messaging
  GoogleCloudMessaging(DeviceTokenGoogleCloudMessaging),
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
  fn default() -> Self { DeviceToken::_Default(()) }
}

impl<'de> Deserialize<'de> for DeviceToken {
  fn deserialize<D>(deserializer: D) -> Result<DeviceToken, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      DeviceToken,
      (deviceTokenApplePush, ApplePush);
      (deviceTokenApplePushVoIP, ApplePushVoIP);
      (deviceTokenBlackBerryPush, BlackBerryPush);
      (deviceTokenGoogleCloudMessaging, GoogleCloudMessaging);
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
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      DeviceToken::ApplePush(t) => t.td_name(),
      DeviceToken::ApplePushVoIP(t) => t.td_name(),
      DeviceToken::BlackBerryPush(t) => t.td_name(),
      DeviceToken::GoogleCloudMessaging(t) => t.td_name(),
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
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl DeviceToken {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let DeviceToken::_Default(_) = self { true } else { false } }

  pub fn is_apple_push(&self) -> bool { if let DeviceToken::ApplePush(_) = self { true } else { false } }
  pub fn is_apple_push_vo_i_p(&self) -> bool { if let DeviceToken::ApplePushVoIP(_) = self { true } else { false } }
  pub fn is_black_berry_push(&self) -> bool { if let DeviceToken::BlackBerryPush(_) = self { true } else { false } }
  pub fn is_google_cloud_messaging(&self) -> bool { if let DeviceToken::GoogleCloudMessaging(_) = self { true } else { false } }
  pub fn is_microsoft_push(&self) -> bool { if let DeviceToken::MicrosoftPush(_) = self { true } else { false } }
  pub fn is_microsoft_push_vo_i_p(&self) -> bool { if let DeviceToken::MicrosoftPushVoIP(_) = self { true } else { false } }
  pub fn is_simple_push(&self) -> bool { if let DeviceToken::SimplePush(_) = self { true } else { false } }
  pub fn is_tizen_push(&self) -> bool { if let DeviceToken::TizenPush(_) = self { true } else { false } }
  pub fn is_ubuntu_push(&self) -> bool { if let DeviceToken::UbuntuPush(_) = self { true } else { false } }
  pub fn is_web_push(&self) -> bool { if let DeviceToken::WebPush(_) = self { true } else { false } }
  pub fn is_windows_push(&self) -> bool { if let DeviceToken::WindowsPush(_) = self { true } else { false } }

  pub fn on_apple_push<F: FnOnce(&DeviceTokenApplePush)>(&self, fnc: F) -> &Self { if let DeviceToken::ApplePush(t) = self { fnc(t) }; self }
  pub fn on_apple_push_vo_i_p<F: FnOnce(&DeviceTokenApplePushVoIP)>(&self, fnc: F) -> &Self { if let DeviceToken::ApplePushVoIP(t) = self { fnc(t) }; self }
  pub fn on_black_berry_push<F: FnOnce(&DeviceTokenBlackBerryPush)>(&self, fnc: F) -> &Self { if let DeviceToken::BlackBerryPush(t) = self { fnc(t) }; self }
  pub fn on_google_cloud_messaging<F: FnOnce(&DeviceTokenGoogleCloudMessaging)>(&self, fnc: F) -> &Self { if let DeviceToken::GoogleCloudMessaging(t) = self { fnc(t) }; self }
  pub fn on_microsoft_push<F: FnOnce(&DeviceTokenMicrosoftPush)>(&self, fnc: F) -> &Self { if let DeviceToken::MicrosoftPush(t) = self { fnc(t) }; self }
  pub fn on_microsoft_push_vo_i_p<F: FnOnce(&DeviceTokenMicrosoftPushVoIP)>(&self, fnc: F) -> &Self { if let DeviceToken::MicrosoftPushVoIP(t) = self { fnc(t) }; self }
  pub fn on_simple_push<F: FnOnce(&DeviceTokenSimplePush)>(&self, fnc: F) -> &Self { if let DeviceToken::SimplePush(t) = self { fnc(t) }; self }
  pub fn on_tizen_push<F: FnOnce(&DeviceTokenTizenPush)>(&self, fnc: F) -> &Self { if let DeviceToken::TizenPush(t) = self { fnc(t) }; self }
  pub fn on_ubuntu_push<F: FnOnce(&DeviceTokenUbuntuPush)>(&self, fnc: F) -> &Self { if let DeviceToken::UbuntuPush(t) = self { fnc(t) }; self }
  pub fn on_web_push<F: FnOnce(&DeviceTokenWebPush)>(&self, fnc: F) -> &Self { if let DeviceToken::WebPush(t) = self { fnc(t) }; self }
  pub fn on_windows_push<F: FnOnce(&DeviceTokenWindowsPush)>(&self, fnc: F) -> &Self { if let DeviceToken::WindowsPush(t) = self { fnc(t) }; self }

  pub fn as_apple_push(&self) -> Option<&DeviceTokenApplePush> { if let DeviceToken::ApplePush(t) = self { return Some(t) } None }
  pub fn as_apple_push_vo_i_p(&self) -> Option<&DeviceTokenApplePushVoIP> { if let DeviceToken::ApplePushVoIP(t) = self { return Some(t) } None }
  pub fn as_black_berry_push(&self) -> Option<&DeviceTokenBlackBerryPush> { if let DeviceToken::BlackBerryPush(t) = self { return Some(t) } None }
  pub fn as_google_cloud_messaging(&self) -> Option<&DeviceTokenGoogleCloudMessaging> { if let DeviceToken::GoogleCloudMessaging(t) = self { return Some(t) } None }
  pub fn as_microsoft_push(&self) -> Option<&DeviceTokenMicrosoftPush> { if let DeviceToken::MicrosoftPush(t) = self { return Some(t) } None }
  pub fn as_microsoft_push_vo_i_p(&self) -> Option<&DeviceTokenMicrosoftPushVoIP> { if let DeviceToken::MicrosoftPushVoIP(t) = self { return Some(t) } None }
  pub fn as_simple_push(&self) -> Option<&DeviceTokenSimplePush> { if let DeviceToken::SimplePush(t) = self { return Some(t) } None }
  pub fn as_tizen_push(&self) -> Option<&DeviceTokenTizenPush> { if let DeviceToken::TizenPush(t) = self { return Some(t) } None }
  pub fn as_ubuntu_push(&self) -> Option<&DeviceTokenUbuntuPush> { if let DeviceToken::UbuntuPush(t) = self { return Some(t) } None }
  pub fn as_web_push(&self) -> Option<&DeviceTokenWebPush> { if let DeviceToken::WebPush(t) = self { return Some(t) } None }
  pub fn as_windows_push(&self) -> Option<&DeviceTokenWindowsPush> { if let DeviceToken::WindowsPush(t) = self { return Some(t) } None }



  pub fn apple_push<T: AsRef<DeviceTokenApplePush>>(t: T) -> Self { DeviceToken::ApplePush(t.as_ref().clone()) }

  pub fn apple_push_vo_i_p<T: AsRef<DeviceTokenApplePushVoIP>>(t: T) -> Self { DeviceToken::ApplePushVoIP(t.as_ref().clone()) }

  pub fn black_berry_push<T: AsRef<DeviceTokenBlackBerryPush>>(t: T) -> Self { DeviceToken::BlackBerryPush(t.as_ref().clone()) }

  pub fn google_cloud_messaging<T: AsRef<DeviceTokenGoogleCloudMessaging>>(t: T) -> Self { DeviceToken::GoogleCloudMessaging(t.as_ref().clone()) }

  pub fn microsoft_push<T: AsRef<DeviceTokenMicrosoftPush>>(t: T) -> Self { DeviceToken::MicrosoftPush(t.as_ref().clone()) }

  pub fn microsoft_push_vo_i_p<T: AsRef<DeviceTokenMicrosoftPushVoIP>>(t: T) -> Self { DeviceToken::MicrosoftPushVoIP(t.as_ref().clone()) }

  pub fn simple_push<T: AsRef<DeviceTokenSimplePush>>(t: T) -> Self { DeviceToken::SimplePush(t.as_ref().clone()) }

  pub fn tizen_push<T: AsRef<DeviceTokenTizenPush>>(t: T) -> Self { DeviceToken::TizenPush(t.as_ref().clone()) }

  pub fn ubuntu_push<T: AsRef<DeviceTokenUbuntuPush>>(t: T) -> Self { DeviceToken::UbuntuPush(t.as_ref().clone()) }

  pub fn web_push<T: AsRef<DeviceTokenWebPush>>(t: T) -> Self { DeviceToken::WebPush(t.as_ref().clone()) }

  pub fn windows_push<T: AsRef<DeviceTokenWindowsPush>>(t: T) -> Self { DeviceToken::WindowsPush(t.as_ref().clone()) }

}

impl AsRef<DeviceToken> for DeviceToken {
  fn as_ref(&self) -> &DeviceToken { self }
}







/// A token for Apple Push Notification service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenApplePush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Device token; may be empty to de-register a device
  device_token: String,
  /// True, if App Sandbox is enabled
  is_app_sandbox: bool,
  
}

impl RObject for DeviceTokenApplePush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenApplePush" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenApplePush {}



impl DeviceTokenApplePush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenApplePushBuilder {
    let mut inner = DeviceTokenApplePush::default();
    inner.td_name = "deviceTokenApplePush".to_string();
    RTDDeviceTokenApplePushBuilder { inner }
  }

  pub fn device_token(&self) -> &String { &self.device_token }

  pub fn is_app_sandbox(&self) -> bool { self.is_app_sandbox }

}

#[doc(hidden)]
pub struct RTDDeviceTokenApplePushBuilder {
  inner: DeviceTokenApplePush
}

impl RTDDeviceTokenApplePushBuilder {
  pub fn build(&self) -> DeviceTokenApplePush { self.inner.clone() }

   
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
  fn as_ref(&self) -> &DeviceTokenApplePush { self }
}

impl AsRef<DeviceTokenApplePush> for RTDDeviceTokenApplePushBuilder {
  fn as_ref(&self) -> &DeviceTokenApplePush { &self.inner }
}







/// A token for Apple Push Notification service VoIP notifications
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenApplePushVoIP {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Device token; may be empty to de-register a device
  device_token: String,
  /// True, if App Sandbox is enabled
  is_app_sandbox: bool,
  
}

impl RObject for DeviceTokenApplePushVoIP {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenApplePushVoIP" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenApplePushVoIP {}



impl DeviceTokenApplePushVoIP {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenApplePushVoIPBuilder {
    let mut inner = DeviceTokenApplePushVoIP::default();
    inner.td_name = "deviceTokenApplePushVoIP".to_string();
    RTDDeviceTokenApplePushVoIPBuilder { inner }
  }

  pub fn device_token(&self) -> &String { &self.device_token }

  pub fn is_app_sandbox(&self) -> bool { self.is_app_sandbox }

}

#[doc(hidden)]
pub struct RTDDeviceTokenApplePushVoIPBuilder {
  inner: DeviceTokenApplePushVoIP
}

impl RTDDeviceTokenApplePushVoIPBuilder {
  pub fn build(&self) -> DeviceTokenApplePushVoIP { self.inner.clone() }

   
  pub fn device_token<T: AsRef<str>>(&mut self, device_token: T) -> &mut Self {
    self.inner.device_token = device_token.as_ref().to_string();
    self
  }

   
  pub fn is_app_sandbox(&mut self, is_app_sandbox: bool) -> &mut Self {
    self.inner.is_app_sandbox = is_app_sandbox;
    self
  }

}

impl AsRef<DeviceTokenApplePushVoIP> for DeviceTokenApplePushVoIP {
  fn as_ref(&self) -> &DeviceTokenApplePushVoIP { self }
}

impl AsRef<DeviceTokenApplePushVoIP> for RTDDeviceTokenApplePushVoIPBuilder {
  fn as_ref(&self) -> &DeviceTokenApplePushVoIP { &self.inner }
}







/// A token for BlackBerry Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenBlackBerryPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Token; may be empty to de-register a device
  token: String,
  
}

impl RObject for DeviceTokenBlackBerryPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenBlackBerryPush" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenBlackBerryPush {}



impl DeviceTokenBlackBerryPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenBlackBerryPushBuilder {
    let mut inner = DeviceTokenBlackBerryPush::default();
    inner.td_name = "deviceTokenBlackBerryPush".to_string();
    RTDDeviceTokenBlackBerryPushBuilder { inner }
  }

  pub fn token(&self) -> &String { &self.token }

}

#[doc(hidden)]
pub struct RTDDeviceTokenBlackBerryPushBuilder {
  inner: DeviceTokenBlackBerryPush
}

impl RTDDeviceTokenBlackBerryPushBuilder {
  pub fn build(&self) -> DeviceTokenBlackBerryPush { self.inner.clone() }

   
  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

}

impl AsRef<DeviceTokenBlackBerryPush> for DeviceTokenBlackBerryPush {
  fn as_ref(&self) -> &DeviceTokenBlackBerryPush { self }
}

impl AsRef<DeviceTokenBlackBerryPush> for RTDDeviceTokenBlackBerryPushBuilder {
  fn as_ref(&self) -> &DeviceTokenBlackBerryPush { &self.inner }
}







/// A token for Google Cloud Messaging
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenGoogleCloudMessaging {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Device registration token; may be empty to de-register a device
  token: String,
  
}

impl RObject for DeviceTokenGoogleCloudMessaging {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenGoogleCloudMessaging" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenGoogleCloudMessaging {}



impl DeviceTokenGoogleCloudMessaging {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenGoogleCloudMessagingBuilder {
    let mut inner = DeviceTokenGoogleCloudMessaging::default();
    inner.td_name = "deviceTokenGoogleCloudMessaging".to_string();
    RTDDeviceTokenGoogleCloudMessagingBuilder { inner }
  }

  pub fn token(&self) -> &String { &self.token }

}

#[doc(hidden)]
pub struct RTDDeviceTokenGoogleCloudMessagingBuilder {
  inner: DeviceTokenGoogleCloudMessaging
}

impl RTDDeviceTokenGoogleCloudMessagingBuilder {
  pub fn build(&self) -> DeviceTokenGoogleCloudMessaging { self.inner.clone() }

   
  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

}

impl AsRef<DeviceTokenGoogleCloudMessaging> for DeviceTokenGoogleCloudMessaging {
  fn as_ref(&self) -> &DeviceTokenGoogleCloudMessaging { self }
}

impl AsRef<DeviceTokenGoogleCloudMessaging> for RTDDeviceTokenGoogleCloudMessagingBuilder {
  fn as_ref(&self) -> &DeviceTokenGoogleCloudMessaging { &self.inner }
}







/// A token for Microsoft Push Notification Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Push notification channel URI; may be empty to de-register a device
  channel_uri: String,
  
}

impl RObject for DeviceTokenMicrosoftPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenMicrosoftPush" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenMicrosoftPush {}



impl DeviceTokenMicrosoftPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenMicrosoftPushBuilder {
    let mut inner = DeviceTokenMicrosoftPush::default();
    inner.td_name = "deviceTokenMicrosoftPush".to_string();
    RTDDeviceTokenMicrosoftPushBuilder { inner }
  }

  pub fn channel_uri(&self) -> &String { &self.channel_uri }

}

#[doc(hidden)]
pub struct RTDDeviceTokenMicrosoftPushBuilder {
  inner: DeviceTokenMicrosoftPush
}

impl RTDDeviceTokenMicrosoftPushBuilder {
  pub fn build(&self) -> DeviceTokenMicrosoftPush { self.inner.clone() }

   
  pub fn channel_uri<T: AsRef<str>>(&mut self, channel_uri: T) -> &mut Self {
    self.inner.channel_uri = channel_uri.as_ref().to_string();
    self
  }

}

impl AsRef<DeviceTokenMicrosoftPush> for DeviceTokenMicrosoftPush {
  fn as_ref(&self) -> &DeviceTokenMicrosoftPush { self }
}

impl AsRef<DeviceTokenMicrosoftPush> for RTDDeviceTokenMicrosoftPushBuilder {
  fn as_ref(&self) -> &DeviceTokenMicrosoftPush { &self.inner }
}







/// A token for Microsoft Push Notification Service VoIP channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPushVoIP {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Push notification channel URI; may be empty to de-register a device
  channel_uri: String,
  
}

impl RObject for DeviceTokenMicrosoftPushVoIP {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenMicrosoftPushVoIP" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenMicrosoftPushVoIP {}



impl DeviceTokenMicrosoftPushVoIP {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenMicrosoftPushVoIPBuilder {
    let mut inner = DeviceTokenMicrosoftPushVoIP::default();
    inner.td_name = "deviceTokenMicrosoftPushVoIP".to_string();
    RTDDeviceTokenMicrosoftPushVoIPBuilder { inner }
  }

  pub fn channel_uri(&self) -> &String { &self.channel_uri }

}

#[doc(hidden)]
pub struct RTDDeviceTokenMicrosoftPushVoIPBuilder {
  inner: DeviceTokenMicrosoftPushVoIP
}

impl RTDDeviceTokenMicrosoftPushVoIPBuilder {
  pub fn build(&self) -> DeviceTokenMicrosoftPushVoIP { self.inner.clone() }

   
  pub fn channel_uri<T: AsRef<str>>(&mut self, channel_uri: T) -> &mut Self {
    self.inner.channel_uri = channel_uri.as_ref().to_string();
    self
  }

}

impl AsRef<DeviceTokenMicrosoftPushVoIP> for DeviceTokenMicrosoftPushVoIP {
  fn as_ref(&self) -> &DeviceTokenMicrosoftPushVoIP { self }
}

impl AsRef<DeviceTokenMicrosoftPushVoIP> for RTDDeviceTokenMicrosoftPushVoIPBuilder {
  fn as_ref(&self) -> &DeviceTokenMicrosoftPushVoIP { &self.inner }
}







/// A token for Simple Push API for Firefox OS
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenSimplePush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device
  endpoint: String,
  
}

impl RObject for DeviceTokenSimplePush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenSimplePush" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenSimplePush {}



impl DeviceTokenSimplePush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenSimplePushBuilder {
    let mut inner = DeviceTokenSimplePush::default();
    inner.td_name = "deviceTokenSimplePush".to_string();
    RTDDeviceTokenSimplePushBuilder { inner }
  }

  pub fn endpoint(&self) -> &String { &self.endpoint }

}

#[doc(hidden)]
pub struct RTDDeviceTokenSimplePushBuilder {
  inner: DeviceTokenSimplePush
}

impl RTDDeviceTokenSimplePushBuilder {
  pub fn build(&self) -> DeviceTokenSimplePush { self.inner.clone() }

   
  pub fn endpoint<T: AsRef<str>>(&mut self, endpoint: T) -> &mut Self {
    self.inner.endpoint = endpoint.as_ref().to_string();
    self
  }

}

impl AsRef<DeviceTokenSimplePush> for DeviceTokenSimplePush {
  fn as_ref(&self) -> &DeviceTokenSimplePush { self }
}

impl AsRef<DeviceTokenSimplePush> for RTDDeviceTokenSimplePushBuilder {
  fn as_ref(&self) -> &DeviceTokenSimplePush { &self.inner }
}







/// A token for Tizen Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenTizenPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Push service registration identifier; may be empty to de-register a device
  reg_id: String,
  
}

impl RObject for DeviceTokenTizenPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenTizenPush" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenTizenPush {}



impl DeviceTokenTizenPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenTizenPushBuilder {
    let mut inner = DeviceTokenTizenPush::default();
    inner.td_name = "deviceTokenTizenPush".to_string();
    RTDDeviceTokenTizenPushBuilder { inner }
  }

  pub fn reg_id(&self) -> &String { &self.reg_id }

}

#[doc(hidden)]
pub struct RTDDeviceTokenTizenPushBuilder {
  inner: DeviceTokenTizenPush
}

impl RTDDeviceTokenTizenPushBuilder {
  pub fn build(&self) -> DeviceTokenTizenPush { self.inner.clone() }

   
  pub fn reg_id<T: AsRef<str>>(&mut self, reg_id: T) -> &mut Self {
    self.inner.reg_id = reg_id.as_ref().to_string();
    self
  }

}

impl AsRef<DeviceTokenTizenPush> for DeviceTokenTizenPush {
  fn as_ref(&self) -> &DeviceTokenTizenPush { self }
}

impl AsRef<DeviceTokenTizenPush> for RTDDeviceTokenTizenPushBuilder {
  fn as_ref(&self) -> &DeviceTokenTizenPush { &self.inner }
}







/// A token for Ubuntu Push Client service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenUbuntuPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Token; may be empty to de-register a device
  token: String,
  
}

impl RObject for DeviceTokenUbuntuPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenUbuntuPush" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenUbuntuPush {}



impl DeviceTokenUbuntuPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenUbuntuPushBuilder {
    let mut inner = DeviceTokenUbuntuPush::default();
    inner.td_name = "deviceTokenUbuntuPush".to_string();
    RTDDeviceTokenUbuntuPushBuilder { inner }
  }

  pub fn token(&self) -> &String { &self.token }

}

#[doc(hidden)]
pub struct RTDDeviceTokenUbuntuPushBuilder {
  inner: DeviceTokenUbuntuPush
}

impl RTDDeviceTokenUbuntuPushBuilder {
  pub fn build(&self) -> DeviceTokenUbuntuPush { self.inner.clone() }

   
  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

}

impl AsRef<DeviceTokenUbuntuPush> for DeviceTokenUbuntuPush {
  fn as_ref(&self) -> &DeviceTokenUbuntuPush { self }
}

impl AsRef<DeviceTokenUbuntuPush> for RTDDeviceTokenUbuntuPushBuilder {
  fn as_ref(&self) -> &DeviceTokenUbuntuPush { &self.inner }
}







/// A token for web Push API
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenWebPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device
  endpoint: String,
  /// Base64url-encoded P-256 elliptic curve Diffie-Hellman public key
  p256dh_base64url: String,
  /// Base64url-encoded authentication secret
  auth_base64url: String,
  
}

impl RObject for DeviceTokenWebPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenWebPush" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenWebPush {}



impl DeviceTokenWebPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenWebPushBuilder {
    let mut inner = DeviceTokenWebPush::default();
    inner.td_name = "deviceTokenWebPush".to_string();
    RTDDeviceTokenWebPushBuilder { inner }
  }

  pub fn endpoint(&self) -> &String { &self.endpoint }

  pub fn p256dh_base64url(&self) -> &String { &self.p256dh_base64url }

  pub fn auth_base64url(&self) -> &String { &self.auth_base64url }

}

#[doc(hidden)]
pub struct RTDDeviceTokenWebPushBuilder {
  inner: DeviceTokenWebPush
}

impl RTDDeviceTokenWebPushBuilder {
  pub fn build(&self) -> DeviceTokenWebPush { self.inner.clone() }

   
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
  fn as_ref(&self) -> &DeviceTokenWebPush { self }
}

impl AsRef<DeviceTokenWebPush> for RTDDeviceTokenWebPushBuilder {
  fn as_ref(&self) -> &DeviceTokenWebPush { &self.inner }
}







/// A token for Windows Push Notification Services
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenWindowsPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The access token that will be used to send notifications; may be empty to de-register a device
  access_token: String,
  
}

impl RObject for DeviceTokenWindowsPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenWindowsPush" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDDeviceToken for DeviceTokenWindowsPush {}



impl DeviceTokenWindowsPush {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDeviceTokenWindowsPushBuilder {
    let mut inner = DeviceTokenWindowsPush::default();
    inner.td_name = "deviceTokenWindowsPush".to_string();
    RTDDeviceTokenWindowsPushBuilder { inner }
  }

  pub fn access_token(&self) -> &String { &self.access_token }

}

#[doc(hidden)]
pub struct RTDDeviceTokenWindowsPushBuilder {
  inner: DeviceTokenWindowsPush
}

impl RTDDeviceTokenWindowsPushBuilder {
  pub fn build(&self) -> DeviceTokenWindowsPush { self.inner.clone() }

   
  pub fn access_token<T: AsRef<str>>(&mut self, access_token: T) -> &mut Self {
    self.inner.access_token = access_token.as_ref().to_string();
    self
  }

}

impl AsRef<DeviceTokenWindowsPush> for DeviceTokenWindowsPush {
  fn as_ref(&self) -> &DeviceTokenWindowsPush { self }
}

impl AsRef<DeviceTokenWindowsPush> for RTDDeviceTokenWindowsPushBuilder {
  fn as_ref(&self) -> &DeviceTokenWindowsPush { &self.inner }
}



