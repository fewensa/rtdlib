
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMe {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getMe
  
}



impl Object for GetMe {}
impl RObject for GetMe {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMe" }
  fn td_type(&self) -> RTDType { RTDType::GetMe }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetMe {}


impl GetMe {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getMe".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



