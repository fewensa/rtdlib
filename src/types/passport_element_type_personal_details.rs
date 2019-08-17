
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's personal details. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypePersonalDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypePersonalDetails
  
}



impl Object for PassportElementTypePersonalDetails {}
impl RObject for PassportElementTypePersonalDetails {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypePersonalDetails" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypePersonalDetails }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypePersonalDetails {}


impl PassportElementTypePersonalDetails {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypePersonalDetails".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



