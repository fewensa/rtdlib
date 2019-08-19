
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRecoveryEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setRecoveryEmailAddress
  /// Password of the current user.
  password: Option<String>,
  /// New recovery email address.
  new_recovery_email_address: Option<String>,
  
}



impl Object for SetRecoveryEmailAddress {}
impl RObject for SetRecoveryEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setRecoveryEmailAddress" }
  fn td_type(&self) -> RTDType { RTDType::SetRecoveryEmailAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetRecoveryEmailAddress {}


impl SetRecoveryEmailAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setRecoveryEmailAddress".to_string(),
      password: None,
      new_recovery_email_address: None,
      
    }
  }
  
  pub fn password(&self) -> Option<String> { self.password.clone() }
  #[doc(hidden)] pub fn _set_password(&mut self, password: String) -> &mut Self { self.password = Some(password); self }
  
  pub fn new_recovery_email_address(&self) -> Option<String> { self.new_recovery_email_address.clone() }
  #[doc(hidden)] pub fn _set_new_recovery_email_address(&mut self, new_recovery_email_address: String) -> &mut Self { self.new_recovery_email_address = Some(new_recovery_email_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



