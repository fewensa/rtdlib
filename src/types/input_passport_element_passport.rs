
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's passport. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementPassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementPassport
  /// The passport to be saved.
  passport: Option<InputIdentityDocument>,
  
}



impl Object for InputPassportElementPassport {}
impl RObject for InputPassportElementPassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementPassport" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementPassport }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementPassport {}


impl InputPassportElementPassport {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementPassport".to_string(),
      passport: None,
      
    }
  }
  
  pub fn passport(&self) -> Option<InputIdentityDocument> { self.passport.clone() }
  #[doc(hidden)] pub fn _set_passport(&mut self, passport: InputIdentityDocument) -> &mut Self { self.passport = Some(passport); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



