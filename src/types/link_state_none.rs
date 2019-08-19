
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The phone number of user A is not known to user B. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkStateNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // linkStateNone
  
}



impl Object for LinkStateNone {}
impl RObject for LinkStateNone {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "linkStateNone" }
  fn td_type(&self) -> RTDType { RTDType::LinkStateNone }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl LinkState for LinkStateNone {}


impl LinkStateNone {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "linkStateNone".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



