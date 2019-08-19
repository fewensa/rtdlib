
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destroy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // destroy
  
}



impl Object for Destroy {}
impl RObject for Destroy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "destroy" }
  fn td_type(&self) -> RTDType { RTDType::Destroy }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for Destroy {}


impl Destroy {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "destroy".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



