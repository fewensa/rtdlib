
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The log is written nowhere. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStreamEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // logStreamEmpty
  
}



impl Object for LogStreamEmpty {}
impl RObject for LogStreamEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logStreamEmpty" }
  fn td_type(&self) -> RTDType { RTDType::LogStreamEmpty }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl LogStream for LogStreamEmpty {}


impl LogStreamEmpty {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "logStreamEmpty".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



