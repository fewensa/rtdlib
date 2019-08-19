
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A mask should be placed relatively to the chin. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskPointChin {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // maskPointChin
  
}



impl Object for MaskPointChin {}
impl RObject for MaskPointChin {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "maskPointChin" }
  fn td_type(&self) -> RTDType { RTDType::MaskPointChin }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MaskPoint for MaskPointChin {}


impl MaskPointChin {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "maskPointChin".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



