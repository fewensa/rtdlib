
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents a data needed to subscribe for push notifications through 
#[typetag::serde(tag = "@struct")]
pub trait DeviceToken: Object + RObject + Debug {}






impl DeviceToken {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<DeviceToken> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDDeviceTokenType {
  DeviceTokenApplePush,
  DeviceTokenApplePushVoIP,
  DeviceTokenBlackBerryPush,
  DeviceTokenFirebaseCloudMessaging,
  DeviceTokenMicrosoftPush,
  DeviceTokenMicrosoftPushVoIP,
  DeviceTokenSimplePush,
  DeviceTokenTizenPush,
  DeviceTokenUbuntuPush,
  DeviceTokenWebPush,
  DeviceTokenWindowsPush,
  
}
impl RTDDeviceTokenType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDDeviceTokenType)(text.as_ref()) }
}



