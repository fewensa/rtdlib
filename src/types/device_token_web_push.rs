
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A token for web Push API. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenWebPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deviceTokenWebPush
  /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device.
  endpoint: Option<String>,
  /// Base64url-encoded P-256 elliptic curve Diffie-Hellman public key.
  p256dh_base64url: Option<String>,
  /// Base64url-encoded authentication secret.
  auth_base64url: Option<String>,
  
}



impl Object for DeviceTokenWebPush {}
impl RObject for DeviceTokenWebPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenWebPush" }
  fn td_type(&self) -> RTDType { RTDType::DeviceTokenWebPush }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl DeviceToken for DeviceTokenWebPush {}


impl DeviceTokenWebPush {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deviceTokenWebPush".to_string(),
      endpoint: None,
      p256dh_base64url: None,
      auth_base64url: None,
      
    }
  }
  
  pub fn endpoint(&self) -> Option<String> { self.endpoint.clone() }
  #[doc(hidden)] pub fn _set_endpoint(&mut self, endpoint: String) -> &mut Self { self.endpoint = Some(endpoint); self }
  
  pub fn p256dh_base64url(&self) -> Option<String> { self.p256dh_base64url.clone() }
  #[doc(hidden)] pub fn _set_p256dh_base64url(&mut self, p256dh_base64url: String) -> &mut Self { self.p256dh_base64url = Some(p256dh_base64url); self }
  
  pub fn auth_base64url(&self) -> Option<String> { self.auth_base64url.clone() }
  #[doc(hidden)] pub fn _set_auth_base64url(&mut self, auth_base64url: String) -> &mut Self { self.auth_base64url = Some(auth_base64url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



