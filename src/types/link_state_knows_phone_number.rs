
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The phone number of user A is known but that number has not been saved to the contact list of user B. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkStateKnowsPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // linkStateKnowsPhoneNumber
  
}



impl Object for LinkStateKnowsPhoneNumber {}
impl RObject for LinkStateKnowsPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "linkStateKnowsPhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::LinkStateKnowsPhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl LinkState for LinkStateKnowsPhoneNumber {}


impl LinkStateKnowsPhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "linkStateKnowsPhoneNumber".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



