
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An empty block separating a page. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockDivider {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockDivider
  
}



impl Object for PageBlockDivider {}
impl RObject for PageBlockDivider {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockDivider" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockDivider }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockDivider {}


impl PageBlockDivider {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockDivider".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



