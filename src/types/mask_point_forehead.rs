
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A mask should be placed relatively to the forehead. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskPointForehead {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // maskPointForehead
  
}



impl Object for MaskPointForehead {}
impl RObject for MaskPointForehead {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "maskPointForehead" }
  fn td_type(&self) -> RTDType { RTDType::MaskPointForehead }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MaskPoint for MaskPointForehead {}


impl MaskPointForehead {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "maskPointForehead".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



