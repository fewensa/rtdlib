
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The connection state has changed. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConnectionState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateConnectionState
  /// The new connection state.
  state: Option<Box<ConnectionState>>,
  
}


impl Clone for UpdateConnectionState {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UpdateConnectionState {}
impl RObject for UpdateConnectionState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateConnectionState" }
  fn td_type(&self) -> RTDType { RTDType::UpdateConnectionState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateConnectionState {}


impl UpdateConnectionState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateConnectionState".to_string(),
      state: None,
      
    }
  }
  
  pub fn state(&self) -> Option<Box<ConnectionState>> { self.state.clone() }
  #[doc(hidden)] pub fn _set_state(&mut self, state: Box<ConnectionState>) -> &mut Self { self.state = Some(state); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



