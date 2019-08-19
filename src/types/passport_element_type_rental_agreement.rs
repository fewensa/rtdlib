
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's rental agreement. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeRentalAgreement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeRentalAgreement
  
}



impl Object for PassportElementTypeRentalAgreement {}
impl RObject for PassportElementTypeRentalAgreement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeRentalAgreement" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeRentalAgreement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeRentalAgreement {}


impl PassportElementTypeRentalAgreement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeRentalAgreement".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



