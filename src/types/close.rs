
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Close {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // close
  
}



impl Object for Close {}
impl RObject for Close {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "close" }
  fn td_type(&self) -> RTDType { RTDType::Close }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for Close {}


impl Close {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "close".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



