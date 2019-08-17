
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// TDLib needs TdlibParameters for initialization. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateWaitTdlibParameters
  
}



impl Object for AuthorizationStateWaitTdlibParameters {}
impl RObject for AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitTdlibParameters" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateWaitTdlibParameters }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateWaitTdlibParameters {}


impl AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateWaitTdlibParameters".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



