
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's passport registration. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementPassportRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementPassportRegistration
  /// The passport registration page to be saved.
  passport_registration: Option<InputPersonalDocument>,
  
}



impl Object for InputPassportElementPassportRegistration {}
impl RObject for InputPassportElementPassportRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementPassportRegistration" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementPassportRegistration }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementPassportRegistration {}


impl InputPassportElementPassportRegistration {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementPassportRegistration".to_string(),
      passport_registration: None,
      
    }
  }
  
  pub fn passport_registration(&self) -> Option<InputPersonalDocument> { self.passport_registration.clone() }
  #[doc(hidden)] pub fn _set_passport_registration(&mut self, passport_registration: InputPersonalDocument) -> &mut Self { self.passport_registration = Some(passport_registration); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



