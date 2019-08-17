
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A token for Windows Push Notification Services. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenWindowsPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deviceTokenWindowsPush
  /// The access token that will be used to send notifications; may be empty to de-register a device.
  access_token: Option<String>,
  
}



impl Object for DeviceTokenWindowsPush {}
impl RObject for DeviceTokenWindowsPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenWindowsPush" }
  fn td_type(&self) -> RTDType { RTDType::DeviceTokenWindowsPush }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl DeviceToken for DeviceTokenWindowsPush {}


impl DeviceTokenWindowsPush {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deviceTokenWindowsPush".to_string(),
      access_token: None,
      
    }
  }
  
  pub fn access_token(&self) -> Option<String> { self.access_token.clone() }
  #[doc(hidden)] pub fn _set_access_token(&mut self, access_token: String) -> &mut Self { self.access_token = Some(access_token); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



