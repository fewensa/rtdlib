
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's internal passport. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementInternalPassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementInternalPassport
  /// The internal passport to be saved.
  internal_passport: Option<InputIdentityDocument>,
  
}



impl Object for InputPassportElementInternalPassport {}
impl RObject for InputPassportElementInternalPassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementInternalPassport" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementInternalPassport }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementInternalPassport {}


impl InputPassportElementInternalPassport {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementInternalPassport".to_string(),
      internal_passport: None,
      
    }
  }
  
  pub fn internal_passport(&self) -> Option<InputIdentityDocument> { self.internal_passport.clone() }
  #[doc(hidden)] pub fn _set_internal_passport(&mut self, internal_passport: InputIdentityDocument) -> &mut Self { self.internal_passport = Some(internal_passport); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



