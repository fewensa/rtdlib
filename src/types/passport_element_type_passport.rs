
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's passport. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypePassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypePassport
  
}



impl Object for PassportElementTypePassport {}
impl RObject for PassportElementTypePassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypePassport" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypePassport }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypePassport {}


impl PassportElementTypePassport {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypePassport".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



