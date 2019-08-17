
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Requests to send a password recovery code to an email address that was previously set up.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestPasswordRecovery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // requestPasswordRecovery
  
}



impl Object for RequestPasswordRecovery {}
impl RObject for RequestPasswordRecovery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "requestPasswordRecovery" }
  fn td_type(&self) -> RTDType { RTDType::RequestPasswordRecovery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RequestPasswordRecovery {}


impl RequestPasswordRecovery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "requestPasswordRecovery".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



