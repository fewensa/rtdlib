
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's internal passport. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementInternalPassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementInternalPassport
  /// Internal passport.
  internal_passport: Option<IdentityDocument>,
  
}



impl Object for PassportElementInternalPassport {}
impl RObject for PassportElementInternalPassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementInternalPassport" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementInternalPassport }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementInternalPassport {}


impl PassportElementInternalPassport {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementInternalPassport".to_string(),
      internal_passport: None,
      
    }
  }
  
  pub fn internal_passport(&self) -> Option<IdentityDocument> { self.internal_passport.clone() }
  #[doc(hidden)] pub fn _set_internal_passport(&mut self, internal_passport: IdentityDocument) -> &mut Self { self.internal_passport = Some(internal_passport); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



