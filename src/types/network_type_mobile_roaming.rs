
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A mobile roaming network. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTypeMobileRoaming {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // networkTypeMobileRoaming
  
}



impl Object for NetworkTypeMobileRoaming {}
impl RObject for NetworkTypeMobileRoaming {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeMobileRoaming" }
  fn td_type(&self) -> RTDType { RTDType::NetworkTypeMobileRoaming }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NetworkType for NetworkTypeMobileRoaming {}


impl NetworkTypeMobileRoaming {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "networkTypeMobileRoaming".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



