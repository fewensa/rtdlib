
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A token for Tizen Push Service. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTokenTizenPush {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deviceTokenTizenPush
  /// Push service registration identifier; may be empty to de-register a device.
  reg_id: Option<String>,
  
}



impl Object for DeviceTokenTizenPush {}
impl RObject for DeviceTokenTizenPush {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deviceTokenTizenPush" }
  fn td_type(&self) -> RTDType { RTDType::DeviceTokenTizenPush }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl DeviceToken for DeviceTokenTizenPush {}


impl DeviceTokenTizenPush {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deviceTokenTizenPush".to_string(),
      reg_id: None,
      
    }
  }
  
  pub fn reg_id(&self) -> Option<String> { self.reg_id.clone() }
  #[doc(hidden)] pub fn _set_reg_id(&mut self, reg_id: String) -> &mut Self { self.reg_id = Some(reg_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



