
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateClosing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateClosing
  
}



impl Object for AuthorizationStateClosing {}
impl RObject for AuthorizationStateClosing {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateClosing" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateClosing }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateClosing {}


impl AuthorizationStateClosing {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateClosing".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



