
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's personal details. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementPersonalDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementPersonalDetails
  /// Personal details of the user.
  personal_details: Option<PersonalDetails>,
  
}



impl Object for InputPassportElementPersonalDetails {}
impl RObject for InputPassportElementPersonalDetails {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementPersonalDetails" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementPersonalDetails }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementPersonalDetails {}


impl InputPassportElementPersonalDetails {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementPersonalDetails".to_string(),
      personal_details: None,
      
    }
  }
  
  pub fn personal_details(&self) -> Option<PersonalDetails> { self.personal_details.clone() }
  #[doc(hidden)] pub fn _set_personal_details(&mut self, personal_details: PersonalDetails) -> &mut Self { self.personal_details = Some(personal_details); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



