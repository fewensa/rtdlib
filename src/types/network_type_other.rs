
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A different network type (e.g., Ethernet network). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTypeOther {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // networkTypeOther
  
}



impl Object for NetworkTypeOther {}
impl RObject for NetworkTypeOther {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeOther" }
  fn td_type(&self) -> RTDType { RTDType::NetworkTypeOther }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NetworkType for NetworkTypeOther {}


impl NetworkTypeOther {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "networkTypeOther".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



