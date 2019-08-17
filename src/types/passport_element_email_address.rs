
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's email address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementEmailAddress
  /// Email address.
  email_address: Option<String>,
  
}



impl Object for PassportElementEmailAddress {}
impl RObject for PassportElementEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementEmailAddress" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementEmailAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementEmailAddress {}


impl PassportElementEmailAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementEmailAddress".to_string(),
      email_address: None,
      
    }
  }
  
  pub fn email_address(&self) -> Option<String> { self.email_address.clone() }
  #[doc(hidden)] pub fn _set_email_address(&mut self, email_address: String) -> &mut Self { self.email_address = Some(email_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



