
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDevice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // registerDevice
  /// Device token.
  device_token: Option<Box<DeviceToken>>,
  /// List of user identifiers of other users currently using the client.
  other_user_ids: Option<Vec<i32>>,
  
}


impl Clone for RegisterDevice {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RegisterDevice {}
impl RObject for RegisterDevice {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "registerDevice" }
  fn td_type(&self) -> RTDType { RTDType::RegisterDevice }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RegisterDevice {}


impl RegisterDevice {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "registerDevice".to_string(),
      device_token: None,
      other_user_ids: None,
      
    }
  }
  
  pub fn device_token(&self) -> Option<Box<DeviceToken>> { self.device_token.clone() }
  #[doc(hidden)] pub fn _set_device_token(&mut self, device_token: Box<DeviceToken>) -> &mut Self { self.device_token = Some(device_token); self }
  
  pub fn other_user_ids(&self) -> Option<Vec<i32>> { self.other_user_ids.clone() }
  #[doc(hidden)] pub fn _set_other_user_ids(&mut self, other_user_ids: Vec<i32>) -> &mut Self { self.other_user_ids = Some(other_user_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



