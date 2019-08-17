
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The content should be center-aligned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentCenter {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockHorizontalAlignmentCenter
  
}



impl Object for PageBlockHorizontalAlignmentCenter {}
impl RObject for PageBlockHorizontalAlignmentCenter {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockHorizontalAlignmentCenter" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockHorizontalAlignmentCenter }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlockHorizontalAlignment for PageBlockHorizontalAlignmentCenter {}


impl PageBlockHorizontalAlignmentCenter {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockHorizontalAlignmentCenter".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



