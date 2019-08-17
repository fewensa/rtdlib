
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's utility bill. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeUtilityBill {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeUtilityBill
  
}



impl Object for PassportElementTypeUtilityBill {}
impl RObject for PassportElementTypeUtilityBill {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeUtilityBill" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeUtilityBill }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeUtilityBill {}


impl PassportElementTypeUtilityBill {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeUtilityBill".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



