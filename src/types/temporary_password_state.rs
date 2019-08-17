
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about the availability of a temporary password, which can be used for payments. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryPasswordState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // temporaryPasswordState
  /// True, if a temporary password is available.
  has_password: Option<bool>,
  /// Time left before the temporary password expires, in seconds.
  valid_for: Option<i32>,
  
}



impl Object for TemporaryPasswordState {}
impl RObject for TemporaryPasswordState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "temporaryPasswordState" }
  fn td_type(&self) -> RTDType { RTDType::TemporaryPasswordState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TemporaryPasswordState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "temporaryPasswordState".to_string(),
      has_password: None,
      valid_for: None,
      
    }
  }
  
  pub fn has_password(&self) -> Option<bool> { self.has_password.clone() }
  #[doc(hidden)] pub fn _set_has_password(&mut self, has_password: bool) -> &mut Self { self.has_password = Some(has_password); self }
  
  pub fn valid_for(&self) -> Option<i32> { self.valid_for.clone() }
  #[doc(hidden)] pub fn _set_valid_for(&mut self, valid_for: i32) -> &mut Self { self.valid_for = Some(valid_for); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



