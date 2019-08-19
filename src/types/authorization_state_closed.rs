
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one should create a new instance of the TDLib client. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateClosed
  
}



impl Object for AuthorizationStateClosed {}
impl RObject for AuthorizationStateClosed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateClosed" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateClosed }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateClosed {}


impl AuthorizationStateClosed {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateClosed".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



