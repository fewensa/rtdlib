
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A token for Simple Push API for Firefox OS. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenSimplePush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deviceTokenSimplePush
  /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device.
  endpoint: Option<String>,
  
}



impl Object for DeviceTokenSimplePush {}
impl RObject for DeviceTokenSimplePush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenSimplePush" }
  fn td_type(&self) -> RTDType { RTDType::DeviceTokenSimplePush }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl DeviceToken for DeviceTokenSimplePush {}


impl DeviceTokenSimplePush {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deviceTokenSimplePush".to_string(),
      endpoint: None,
      
    }
  }
  
  pub fn endpoint(&self) -> Option<String> { self.endpoint.clone() }
  #[doc(hidden)] pub fn _set_endpoint(&mut self, endpoint: String) -> &mut Self { self.endpoint = Some(endpoint); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



