
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A token for Microsoft Push Notification Service. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deviceTokenMicrosoftPush
  /// Push notification channel URI; may be empty to de-register a device.
  channel_uri: Option<String>,
  
}



impl Object for DeviceTokenMicrosoftPush {}
impl RObject for DeviceTokenMicrosoftPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenMicrosoftPush" }
  fn td_type(&self) -> RTDType { RTDType::DeviceTokenMicrosoftPush }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl DeviceToken for DeviceTokenMicrosoftPush {}


impl DeviceTokenMicrosoftPush {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deviceTokenMicrosoftPush".to_string(),
      channel_uri: None,
      
    }
  }
  
  pub fn channel_uri(&self) -> Option<String> { self.channel_uri.clone() }
  #[doc(hidden)] pub fn _set_channel_uri(&mut self, channel_uri: String) -> &mut Self { self.channel_uri = Some(channel_uri); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



