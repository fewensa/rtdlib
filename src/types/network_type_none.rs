
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The network is not available. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTypeNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // networkTypeNone
  
}



impl Object for NetworkTypeNone {}
impl RObject for NetworkTypeNone {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeNone" }
  fn td_type(&self) -> RTDType { RTDType::NetworkTypeNone }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NetworkType for NetworkTypeNone {}


impl NetworkTypeNone {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "networkTypeNone".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



