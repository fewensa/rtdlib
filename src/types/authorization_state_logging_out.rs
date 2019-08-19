
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is currently logging out. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateLoggingOut {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateLoggingOut
  
}



impl Object for AuthorizationStateLoggingOut {}
impl RObject for AuthorizationStateLoggingOut {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateLoggingOut" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateLoggingOut }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateLoggingOut {}


impl AuthorizationStateLoggingOut {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateLoggingOut".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



