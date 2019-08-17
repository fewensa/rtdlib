
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverAuthenticationPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // recoverAuthenticationPassword
  /// Recovery code to check.
  recovery_code: Option<String>,
  
}



impl Object for RecoverAuthenticationPassword {}
impl RObject for RecoverAuthenticationPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "recoverAuthenticationPassword" }
  fn td_type(&self) -> RTDType { RTDType::RecoverAuthenticationPassword }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RecoverAuthenticationPassword {}


impl RecoverAuthenticationPassword {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "recoverAuthenticationPassword".to_string(),
      recovery_code: None,
      
    }
  }
  
  pub fn recovery_code(&self) -> Option<String> { self.recovery_code.clone() }
  #[doc(hidden)] pub fn _set_recovery_code(&mut self, recovery_code: String) -> &mut Self { self.recovery_code = Some(recovery_code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



