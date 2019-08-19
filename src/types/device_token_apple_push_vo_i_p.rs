
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A token for Apple Push Notification service VoIP notifications. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenApplePushVoIP {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deviceTokenApplePushVoIP
  /// Device token; may be empty to de-register a device.
  device_token: Option<String>,
  /// True, if App Sandbox is enabled.
  is_app_sandbox: Option<bool>,
  /// True, if push notifications should be additionally encrypted.
  encrypt: Option<bool>,
  
}



impl Object for DeviceTokenApplePushVoIP {}
impl RObject for DeviceTokenApplePushVoIP {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenApplePushVoIP" }
  fn td_type(&self) -> RTDType { RTDType::DeviceTokenApplePushVoIP }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl DeviceToken for DeviceTokenApplePushVoIP {}


impl DeviceTokenApplePushVoIP {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deviceTokenApplePushVoIP".to_string(),
      device_token: None,
      is_app_sandbox: None,
      encrypt: None,
      
    }
  }
  
  pub fn device_token(&self) -> Option<String> { self.device_token.clone() }
  #[doc(hidden)] pub fn _set_device_token(&mut self, device_token: String) -> &mut Self { self.device_token = Some(device_token); self }
  
  pub fn is_app_sandbox(&self) -> Option<bool> { self.is_app_sandbox.clone() }
  #[doc(hidden)] pub fn _set_is_app_sandbox(&mut self, is_app_sandbox: bool) -> &mut Self { self.is_app_sandbox = Some(is_app_sandbox); self }
  
  pub fn encrypt(&self) -> Option<bool> { self.encrypt.clone() }
  #[doc(hidden)] pub fn _set_encrypt(&mut self, encrypt: bool) -> &mut Self { self.encrypt = Some(encrypt); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



