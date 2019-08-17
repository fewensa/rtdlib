
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A mask should be placed relatively to the eyes. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskPointEyes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // maskPointEyes
  
}



impl Object for MaskPointEyes {}
impl RObject for MaskPointEyes {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "maskPointEyes" }
  fn td_type(&self) -> RTDType { RTDType::MaskPointEyes }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MaskPoint for MaskPointEyes {}


impl MaskPointEyes {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "maskPointEyes".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



