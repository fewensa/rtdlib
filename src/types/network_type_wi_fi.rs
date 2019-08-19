
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Wi-Fi network. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTypeWiFi {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // networkTypeWiFi
  
}



impl Object for NetworkTypeWiFi {}
impl RObject for NetworkTypeWiFi {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeWiFi" }
  fn td_type(&self) -> RTDType { RTDType::NetworkTypeWiFi }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NetworkType for NetworkTypeWiFi {}


impl NetworkTypeWiFi {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "networkTypeWiFi".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



