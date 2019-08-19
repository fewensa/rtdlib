
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's email address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementEmailAddress
  /// The email address to be saved.
  email_address: Option<String>,
  
}



impl Object for InputPassportElementEmailAddress {}
impl RObject for InputPassportElementEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementEmailAddress" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementEmailAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementEmailAddress {}


impl InputPassportElementEmailAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementEmailAddress".to_string(),
      email_address: None,
      
    }
  }
  
  pub fn email_address(&self) -> Option<String> { self.email_address.clone() }
  #[doc(hidden)] pub fn _set_email_address(&mut self, email_address: String) -> &mut Self { self.email_address = Some(email_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



