
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An object of this type is returned on a successful function call for certain functions. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ok {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // ok
  
}



impl Object for Ok {}
impl RObject for Ok {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "ok" }
  fn td_type(&self) -> RTDType { RTDType::Ok }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Ok {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "ok".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



