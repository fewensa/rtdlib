
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents the current state of 2-step verification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PasswordState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True if a 2-step verification password is set
  has_password: bool,
  /// Hint for the password; can be empty
  password_hint: String,
  /// True if a recovery email is set
  has_recovery_email_address: bool,
  /// True if some Telegram Passport elements were saved
  has_passport_data: bool,
  /// Pattern of the email address to which the confirmation email was sent
  unconfirmed_recovery_email_address_pattern: String,
  
}

impl RObject for PasswordState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passwordState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PasswordState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPasswordStateBuilder {
    let mut inner = PasswordState::default();
    inner.td_name = "passwordState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPasswordStateBuilder { inner }
  }

  pub fn has_password(&self) -> bool { self.has_password }

  pub fn password_hint(&self) -> &String { &self.password_hint }

  pub fn has_recovery_email_address(&self) -> bool { self.has_recovery_email_address }

  pub fn has_passport_data(&self) -> bool { self.has_passport_data }

  pub fn unconfirmed_recovery_email_address_pattern(&self) -> &String { &self.unconfirmed_recovery_email_address_pattern }

}

#[doc(hidden)]
pub struct RTDPasswordStateBuilder {
  inner: PasswordState
}

impl RTDPasswordStateBuilder {
  pub fn build(&self) -> PasswordState { self.inner.clone() }

   
  pub fn has_password(&mut self, has_password: bool) -> &mut Self {
    self.inner.has_password = has_password;
    self
  }

   
  pub fn password_hint<T: AsRef<str>>(&mut self, password_hint: T) -> &mut Self {
    self.inner.password_hint = password_hint.as_ref().to_string();
    self
  }

   
  pub fn has_recovery_email_address(&mut self, has_recovery_email_address: bool) -> &mut Self {
    self.inner.has_recovery_email_address = has_recovery_email_address;
    self
  }

   
  pub fn has_passport_data(&mut self, has_passport_data: bool) -> &mut Self {
    self.inner.has_passport_data = has_passport_data;
    self
  }

   
  pub fn unconfirmed_recovery_email_address_pattern<T: AsRef<str>>(&mut self, unconfirmed_recovery_email_address_pattern: T) -> &mut Self {
    self.inner.unconfirmed_recovery_email_address_pattern = unconfirmed_recovery_email_address_pattern.as_ref().to_string();
    self
  }

}

impl AsRef<PasswordState> for PasswordState {
  fn as_ref(&self) -> &PasswordState { self }
}

impl AsRef<PasswordState> for RTDPasswordStateBuilder {
  fn as_ref(&self) -> &PasswordState { &self.inner }
}



