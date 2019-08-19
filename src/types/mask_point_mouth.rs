
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A mask should be placed relatively to the mouth. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskPointMouth {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // maskPointMouth
  
}



impl Object for MaskPointMouth {}
impl RObject for MaskPointMouth {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "maskPointMouth" }
  fn td_type(&self) -> RTDType { RTDType::MaskPointMouth }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MaskPoint for MaskPointMouth {}


impl MaskPointMouth {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "maskPointMouth".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



