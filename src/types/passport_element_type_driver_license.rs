
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's driver license. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeDriverLicense {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeDriverLicense
  
}



impl Object for PassportElementTypeDriverLicense {}
impl RObject for PassportElementTypeDriverLicense {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeDriverLicense" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeDriverLicense }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeDriverLicense {}


impl PassportElementTypeDriverLicense {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeDriverLicense".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



