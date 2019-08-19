
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's passport. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementPassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementPassport
  /// Passport.
  passport: Option<IdentityDocument>,
  
}



impl Object for PassportElementPassport {}
impl RObject for PassportElementPassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementPassport" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementPassport }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementPassport {}


impl PassportElementPassport {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementPassport".to_string(),
      passport: None,
      
    }
  }
  
  pub fn passport(&self) -> Option<IdentityDocument> { self.passport.clone() }
  #[doc(hidden)] pub fn _set_passport(&mut self, passport: IdentityDocument) -> &mut Self { self.passport = Some(passport); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



