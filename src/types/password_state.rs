
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents the current state of 2-step verification. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passwordState
  /// True, if a 2-step verification password is set.
  has_password: Option<bool>,
  /// Hint for the password; may be empty.
  password_hint: Option<String>,
  /// True, if a recovery email is set.
  has_recovery_email_address: Option<bool>,
  /// True, if some Telegram Passport elements were saved.
  has_passport_data: Option<bool>,
  /// Information about the recovery email address to which the confirmation email was sent; may be null.
  recovery_email_address_code_info: Option<EmailAddressAuthenticationCodeInfo>,
  
}



impl Object for PasswordState {}
impl RObject for PasswordState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passwordState" }
  fn td_type(&self) -> RTDType { RTDType::PasswordState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PasswordState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passwordState".to_string(),
      has_password: None,
      password_hint: None,
      has_recovery_email_address: None,
      has_passport_data: None,
      recovery_email_address_code_info: None,
      
    }
  }
  
  pub fn has_password(&self) -> Option<bool> { self.has_password.clone() }
  #[doc(hidden)] pub fn _set_has_password(&mut self, has_password: bool) -> &mut Self { self.has_password = Some(has_password); self }
  
  pub fn password_hint(&self) -> Option<String> { self.password_hint.clone() }
  #[doc(hidden)] pub fn _set_password_hint(&mut self, password_hint: String) -> &mut Self { self.password_hint = Some(password_hint); self }
  
  pub fn has_recovery_email_address(&self) -> Option<bool> { self.has_recovery_email_address.clone() }
  #[doc(hidden)] pub fn _set_has_recovery_email_address(&mut self, has_recovery_email_address: bool) -> &mut Self { self.has_recovery_email_address = Some(has_recovery_email_address); self }
  
  pub fn has_passport_data(&self) -> Option<bool> { self.has_passport_data.clone() }
  #[doc(hidden)] pub fn _set_has_passport_data(&mut self, has_passport_data: bool) -> &mut Self { self.has_passport_data = Some(has_passport_data); self }
  
  pub fn recovery_email_address_code_info(&self) -> Option<EmailAddressAuthenticationCodeInfo> { self.recovery_email_address_code_info.clone() }
  #[doc(hidden)] pub fn _set_recovery_email_address_code_info(&mut self, recovery_email_address_code_info: EmailAddressAuthenticationCodeInfo) -> &mut Self { self.recovery_email_address_code_info = Some(recovery_email_address_code_info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



