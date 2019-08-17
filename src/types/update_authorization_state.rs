
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user authorization state has changed. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAuthorizationState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateAuthorizationState
  /// New authorization state.
  authorization_state: Option<Box<AuthorizationState>>,
  
}


impl Clone for UpdateAuthorizationState {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateAuthorizationState {}
impl RObject for UpdateAuthorizationState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateAuthorizationState" }
  fn td_type(&self) -> RTDType { RTDType::UpdateAuthorizationState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateAuthorizationState {}


impl UpdateAuthorizationState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateAuthorizationState".to_string(),
      authorization_state: None,
      
    }
  }
  
  pub fn authorization_state(&self) -> Option<Box<AuthorizationState>> { self.authorization_state.clone() }
  #[doc(hidden)] pub fn _set_authorization_state(&mut self, authorization_state: Box<AuthorizationState>) -> &mut Self { self.authorization_state = Some(authorization_state); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



