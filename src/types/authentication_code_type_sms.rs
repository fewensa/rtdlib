
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An authentication code is delivered via an SMS message to the specified phone number. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeSms {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authenticationCodeTypeSms
  /// Length of the code.
  length: Option<i32>,
  
}



impl Object for AuthenticationCodeTypeSms {}
impl RObject for AuthenticationCodeTypeSms {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authenticationCodeTypeSms" }
  fn td_type(&self) -> RTDType { RTDType::AuthenticationCodeTypeSms }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthenticationCodeType for AuthenticationCodeTypeSms {}


impl AuthenticationCodeTypeSms {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authenticationCodeTypeSms".to_string(),
      length: None,
      
    }
  }
  
  pub fn length(&self) -> Option<i32> { self.length.clone() }
  #[doc(hidden)] pub fn _set_length(&mut self, length: i32) -> &mut Self { self.length = Some(length); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



