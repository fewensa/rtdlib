
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A token for BlackBerry Push Service. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenBlackBerryPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deviceTokenBlackBerryPush
  /// Token; may be empty to de-register a device.
  token: Option<String>,
  
}



impl Object for DeviceTokenBlackBerryPush {}
impl RObject for DeviceTokenBlackBerryPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenBlackBerryPush" }
  fn td_type(&self) -> RTDType { RTDType::DeviceTokenBlackBerryPush }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl DeviceToken for DeviceTokenBlackBerryPush {}


impl DeviceTokenBlackBerryPush {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deviceTokenBlackBerryPush".to_string(),
      token: None,
      
    }
  }
  
  pub fn token(&self) -> Option<String> { self.token.clone() }
  #[doc(hidden)] pub fn _set_token(&mut self, token: String) -> &mut Self { self.token = Some(token); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



