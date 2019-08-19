
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A mobile network. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTypeMobile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // networkTypeMobile
  
}



impl Object for NetworkTypeMobile {}
impl RObject for NetworkTypeMobile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeMobile" }
  fn td_type(&self) -> RTDType { RTDType::NetworkTypeMobile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NetworkType for NetworkTypeMobile {}


impl NetworkTypeMobile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "networkTypeMobile".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



