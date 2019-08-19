
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The call has ended with an error. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStateError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callStateError
  /// Error. An error with the code 4005000 will be returned if an outgoing call is missed because of an expired timeout.
  error: Option<Error>,
  
}



impl Object for CallStateError {}
impl RObject for CallStateError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callStateError" }
  fn td_type(&self) -> RTDType { RTDType::CallStateError }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallState for CallStateError {}


impl CallStateError {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callStateError".to_string(),
      error: None,
      
    }
  }
  
  pub fn error(&self) -> Option<Error> { self.error.clone() }
  #[doc(hidden)] pub fn _set_error(&mut self, error: Error) -> &mut Self { self.error = Some(error); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



