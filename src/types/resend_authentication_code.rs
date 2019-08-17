
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Re-sends an authentication code to the user. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendAuthenticationCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // resendAuthenticationCode
  
}



impl Object for ResendAuthenticationCode {}
impl RObject for ResendAuthenticationCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendAuthenticationCode" }
  fn td_type(&self) -> RTDType { RTDType::ResendAuthenticationCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ResendAuthenticationCode {}


impl ResendAuthenticationCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "resendAuthenticationCode".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



