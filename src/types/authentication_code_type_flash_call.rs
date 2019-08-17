
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An authentication code is delivered by an immediately cancelled call to the specified phone number. The number from which the call was made is the code. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFlashCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authenticationCodeTypeFlashCall
  /// Pattern of the phone number from which the call will be made.
  pattern: Option<String>,
  
}



impl Object for AuthenticationCodeTypeFlashCall {}
impl RObject for AuthenticationCodeTypeFlashCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authenticationCodeTypeFlashCall" }
  fn td_type(&self) -> RTDType { RTDType::AuthenticationCodeTypeFlashCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthenticationCodeType for AuthenticationCodeTypeFlashCall {}


impl AuthenticationCodeTypeFlashCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authenticationCodeTypeFlashCall".to_string(),
      pattern: None,
      
    }
  }
  
  pub fn pattern(&self) -> Option<String> { self.pattern.clone() }
  #[doc(hidden)] pub fn _set_pattern(&mut self, pattern: String) -> &mut Self { self.pattern = Some(pattern); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



