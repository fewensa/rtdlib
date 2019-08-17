
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The log is written to stderr or an OS specific log. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStreamDefault {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // logStreamDefault
  
}



impl Object for LogStreamDefault {}
impl RObject for LogStreamDefault {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logStreamDefault" }
  fn td_type(&self) -> RTDType { RTDType::LogStreamDefault }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl LogStream for LogStreamDefault {}


impl LogStreamDefault {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "logStreamDefault".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



