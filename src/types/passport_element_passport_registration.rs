
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's passport registration pages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementPassportRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementPassportRegistration
  /// Passport registration pages.
  passport_registration: Option<PersonalDocument>,
  
}



impl Object for PassportElementPassportRegistration {}
impl RObject for PassportElementPassportRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementPassportRegistration" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementPassportRegistration }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementPassportRegistration {}


impl PassportElementPassportRegistration {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementPassportRegistration".to_string(),
      passport_registration: None,
      
    }
  }
  
  pub fn passport_registration(&self) -> Option<PersonalDocument> { self.passport_registration.clone() }
  #[doc(hidden)] pub fn _set_passport_registration(&mut self, passport_registration: PersonalDocument) -> &mut Self { self.passport_registration = Some(passport_registration); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



