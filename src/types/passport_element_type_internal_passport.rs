
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's internal passport. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeInternalPassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeInternalPassport
  
}



impl Object for PassportElementTypeInternalPassport {}
impl RObject for PassportElementTypeInternalPassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeInternalPassport" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeInternalPassport }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeInternalPassport {}


impl PassportElementTypeInternalPassport {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeInternalPassport".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



