
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's phone number. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementPhoneNumber
  /// The phone number to be saved.
  phone_number: Option<String>,
  
}



impl Object for InputPassportElementPhoneNumber {}
impl RObject for InputPassportElementPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementPhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementPhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementPhoneNumber {}


impl InputPassportElementPhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementPhoneNumber".to_string(),
      phone_number: None,
      
    }
  }
  
  pub fn phone_number(&self) -> Option<String> { self.phone_number.clone() }
  #[doc(hidden)] pub fn _set_phone_number(&mut self, phone_number: String) -> &mut Self { self.phone_number = Some(phone_number); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



