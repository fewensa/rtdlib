
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about the current recovery email address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // recoveryEmailAddress
  /// Recovery email address.
  recovery_email_address: Option<String>,
  
}



impl Object for RecoveryEmailAddress {}
impl RObject for RecoveryEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "recoveryEmailAddress" }
  fn td_type(&self) -> RTDType { RTDType::RecoveryEmailAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl RecoveryEmailAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "recoveryEmailAddress".to_string(),
      recovery_email_address: None,
      
    }
  }
  
  pub fn recovery_email_address(&self) -> Option<String> { self.recovery_email_address.clone() }
  #[doc(hidden)] pub fn _set_recovery_email_address(&mut self, recovery_email_address: String) -> &mut Self { self.recovery_email_address = Some(recovery_email_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



