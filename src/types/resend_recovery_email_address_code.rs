
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Resends the 2-step verification recovery email address verification code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendRecoveryEmailAddressCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // resendRecoveryEmailAddressCode
  
}



impl Object for ResendRecoveryEmailAddressCode {}
impl RObject for ResendRecoveryEmailAddressCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resendRecoveryEmailAddressCode" }
  fn td_type(&self) -> RTDType { RTDType::ResendRecoveryEmailAddressCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ResendRecoveryEmailAddressCode {}


impl ResendRecoveryEmailAddressCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "resendRecoveryEmailAddressCode".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



