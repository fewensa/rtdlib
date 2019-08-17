
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The phone number of user A has been saved to the contact list of user B. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkStateIsContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // linkStateIsContact
  
}



impl Object for LinkStateIsContact {}
impl RObject for LinkStateIsContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "linkStateIsContact" }
  fn td_type(&self) -> RTDType { RTDType::LinkStateIsContact }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl LinkState for LinkStateIsContact {}


impl LinkStateIsContact {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "linkStateIsContact".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



