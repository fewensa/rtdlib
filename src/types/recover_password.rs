
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Recovers the password using a recovery code sent to an email address that was previously set up.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // recoverPassword
  /// Recovery code to check.
  recovery_code: Option<String>,
  
}



impl Object for RecoverPassword {}
impl RObject for RecoverPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "recoverPassword" }
  fn td_type(&self) -> RTDType { RTDType::RecoverPassword }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RecoverPassword {}


impl RecoverPassword {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "recoverPassword".to_string(),
      recovery_code: None,
      
    }
  }
  
  pub fn recovery_code(&self) -> Option<String> { self.recovery_code.clone() }
  #[doc(hidden)] pub fn _set_recovery_code(&mut self, recovery_code: String) -> &mut Self { self.recovery_code = Some(recovery_code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



