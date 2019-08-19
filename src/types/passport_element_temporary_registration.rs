
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's temporary registration. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTemporaryRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTemporaryRegistration
  /// Temporary registration.
  temporary_registration: Option<PersonalDocument>,
  
}



impl Object for PassportElementTemporaryRegistration {}
impl RObject for PassportElementTemporaryRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTemporaryRegistration" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTemporaryRegistration }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementTemporaryRegistration {}


impl PassportElementTemporaryRegistration {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTemporaryRegistration".to_string(),
      temporary_registration: None,
      
    }
  }
  
  pub fn temporary_registration(&self) -> Option<PersonalDocument> { self.temporary_registration.clone() }
  #[doc(hidden)] pub fn _set_temporary_registration(&mut self, temporary_registration: PersonalDocument) -> &mut Self { self.temporary_registration = Some(temporary_registration); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



