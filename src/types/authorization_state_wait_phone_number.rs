
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// TDLib needs the user's phone number to authorize. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateWaitPhoneNumber
  
}



impl Object for AuthorizationStateWaitPhoneNumber {}
impl RObject for AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitPhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateWaitPhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateWaitPhoneNumber {}


impl AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateWaitPhoneNumber".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



