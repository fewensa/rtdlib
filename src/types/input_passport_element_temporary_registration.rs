
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's temporary registration. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementTemporaryRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementTemporaryRegistration
  /// The temporary registration document to be saved.
  temporary_registration: Option<InputPersonalDocument>,
  
}



impl Object for InputPassportElementTemporaryRegistration {}
impl RObject for InputPassportElementTemporaryRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementTemporaryRegistration" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementTemporaryRegistration }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementTemporaryRegistration {}


impl InputPassportElementTemporaryRegistration {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementTemporaryRegistration".to_string(),
      temporary_registration: None,
      
    }
  }
  
  pub fn temporary_registration(&self) -> Option<InputPersonalDocument> { self.temporary_registration.clone() }
  #[doc(hidden)] pub fn _set_temporary_registration(&mut self, temporary_registration: InputPersonalDocument) -> &mut Self { self.temporary_registration = Some(temporary_registration); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



