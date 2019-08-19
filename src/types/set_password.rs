
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the password for the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setPassword
  /// Previous password of the user.
  old_password: Option<String>,
  /// New password of the user; may be empty to remove the password.
  new_password: Option<String>,
  /// New password hint; may be empty.
  new_hint: Option<String>,
  /// Pass true if the recovery email address should be changed.
  set_recovery_email_address: Option<bool>,
  /// New recovery email address; may be empty.
  new_recovery_email_address: Option<String>,
  
}



impl Object for SetPassword {}
impl RObject for SetPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPassword" }
  fn td_type(&self) -> RTDType { RTDType::SetPassword }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetPassword {}


impl SetPassword {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setPassword".to_string(),
      old_password: None,
      new_password: None,
      new_hint: None,
      set_recovery_email_address: None,
      new_recovery_email_address: None,
      
    }
  }
  
  pub fn old_password(&self) -> Option<String> { self.old_password.clone() }
  #[doc(hidden)] pub fn _set_old_password(&mut self, old_password: String) -> &mut Self { self.old_password = Some(old_password); self }
  
  pub fn new_password(&self) -> Option<String> { self.new_password.clone() }
  #[doc(hidden)] pub fn _set_new_password(&mut self, new_password: String) -> &mut Self { self.new_password = Some(new_password); self }
  
  pub fn new_hint(&self) -> Option<String> { self.new_hint.clone() }
  #[doc(hidden)] pub fn _set_new_hint(&mut self, new_hint: String) -> &mut Self { self.new_hint = Some(new_hint); self }
  
  pub fn set_recovery_email_address(&self) -> Option<bool> { self.set_recovery_email_address.clone() }
  #[doc(hidden)] pub fn _set_set_recovery_email_address(&mut self, set_recovery_email_address: bool) -> &mut Self { self.set_recovery_email_address = Some(set_recovery_email_address); self }
  
  pub fn new_recovery_email_address(&self) -> Option<String> { self.new_recovery_email_address.clone() }
  #[doc(hidden)] pub fn _set_new_recovery_email_address(&mut self, new_recovery_email_address: String) -> &mut Self { self.new_recovery_email_address = Some(new_recovery_email_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



