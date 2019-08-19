
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the 2-step verification recovery email address verification code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRecoveryEmailAddressCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkRecoveryEmailAddressCode
  /// Verification code.
  code: Option<String>,
  
}



impl Object for CheckRecoveryEmailAddressCode {}
impl RObject for CheckRecoveryEmailAddressCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkRecoveryEmailAddressCode" }
  fn td_type(&self) -> RTDType { RTDType::CheckRecoveryEmailAddressCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckRecoveryEmailAddressCode {}


impl CheckRecoveryEmailAddressCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkRecoveryEmailAddressCode".to_string(),
      code: None,
      
    }
  }
  
  pub fn code(&self) -> Option<String> { self.code.clone() }
  #[doc(hidden)] pub fn _set_code(&mut self, code: String) -> &mut Self { self.code = Some(code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



