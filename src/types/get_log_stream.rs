
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about currently used log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLogStream {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getLogStream
  
}



impl Object for GetLogStream {}
impl RObject for GetLogStream {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLogStream" }
  fn td_type(&self) -> RTDType { RTDType::GetLogStream }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetLogStream {}


impl GetLogStream {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getLogStream".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



