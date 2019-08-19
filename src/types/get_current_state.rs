
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially usefull if TDLib is run in a separate process. This is an offline method. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrentState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getCurrentState
  
}



impl Object for GetCurrentState {}
impl RObject for GetCurrentState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getCurrentState" }
  fn td_type(&self) -> RTDType { RTDType::GetCurrentState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetCurrentState {}


impl GetCurrentState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getCurrentState".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



