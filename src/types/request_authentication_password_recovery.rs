
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestAuthenticationPasswordRecovery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // requestAuthenticationPasswordRecovery
  
}



impl Object for RequestAuthenticationPasswordRecovery {}
impl RObject for RequestAuthenticationPasswordRecovery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "requestAuthenticationPasswordRecovery" }
  fn td_type(&self) -> RTDType { RTDType::RequestAuthenticationPasswordRecovery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RequestAuthenticationPasswordRecovery {}


impl RequestAuthenticationPasswordRecovery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "requestAuthenticationPasswordRecovery".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



