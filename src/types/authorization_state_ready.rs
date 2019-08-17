
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user has been successfully authorized. TDLib is now ready to answer queries. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateReady
  
}



impl Object for AuthorizationStateReady {}
impl RObject for AuthorizationStateReady {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateReady" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateReady }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateReady {}


impl AuthorizationStateReady {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateReady".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



