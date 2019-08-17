
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A token for Firebase Cloud Messaging. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenFirebaseCloudMessaging {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deviceTokenFirebaseCloudMessaging
  /// Device registration token; may be empty to de-register a device.
  token: Option<String>,
  /// True, if push notifications should be additionally encrypted.
  encrypt: Option<bool>,
  
}



impl Object for DeviceTokenFirebaseCloudMessaging {}
impl RObject for DeviceTokenFirebaseCloudMessaging {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenFirebaseCloudMessaging" }
  fn td_type(&self) -> RTDType { RTDType::DeviceTokenFirebaseCloudMessaging }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl DeviceToken for DeviceTokenFirebaseCloudMessaging {}


impl DeviceTokenFirebaseCloudMessaging {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deviceTokenFirebaseCloudMessaging".to_string(),
      token: None,
      encrypt: None,
      
    }
  }
  
  pub fn token(&self) -> Option<String> { self.token.clone() }
  #[doc(hidden)] pub fn _set_token(&mut self, token: String) -> &mut Self { self.token = Some(token); self }
  
  pub fn encrypt(&self) -> Option<bool> { self.encrypt.clone() }
  #[doc(hidden)] pub fn _set_encrypt(&mut self, encrypt: bool) -> &mut Self { self.encrypt = Some(encrypt); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



