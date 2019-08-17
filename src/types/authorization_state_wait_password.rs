
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user has been authorized, but needs to enter a password to start using the application. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authorizationStateWaitPassword
  /// Hint for the password; may be empty.
  password_hint: Option<String>,
  /// True if a recovery email address has been set up.
  has_recovery_email_address: Option<bool>,
  /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent.
  recovery_email_address_pattern: Option<String>,
  
}



impl Object for AuthorizationStateWaitPassword {}
impl RObject for AuthorizationStateWaitPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitPassword" }
  fn td_type(&self) -> RTDType { RTDType::AuthorizationStateWaitPassword }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthorizationState for AuthorizationStateWaitPassword {}


impl AuthorizationStateWaitPassword {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authorizationStateWaitPassword".to_string(),
      password_hint: None,
      has_recovery_email_address: None,
      recovery_email_address_pattern: None,
      
    }
  }
  
  pub fn password_hint(&self) -> Option<String> { self.password_hint.clone() }
  #[doc(hidden)] pub fn _set_password_hint(&mut self, password_hint: String) -> &mut Self { self.password_hint = Some(password_hint); self }
  
  pub fn has_recovery_email_address(&self) -> Option<bool> { self.has_recovery_email_address.clone() }
  #[doc(hidden)] pub fn _set_has_recovery_email_address(&mut self, has_recovery_email_address: bool) -> &mut Self { self.has_recovery_email_address = Some(has_recovery_email_address); self }
  
  pub fn recovery_email_address_pattern(&self) -> Option<String> { self.recovery_email_address_pattern.clone() }
  #[doc(hidden)] pub fn _set_recovery_email_address_pattern(&mut self, recovery_email_address_pattern: String) -> &mut Self { self.recovery_email_address_pattern = Some(recovery_email_address_pattern); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



