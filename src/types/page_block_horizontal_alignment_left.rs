
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The content should be left-aligned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockHorizontalAlignmentLeft
  
}



impl Object for PageBlockHorizontalAlignmentLeft {}
impl RObject for PageBlockHorizontalAlignmentLeft {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockHorizontalAlignmentLeft" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockHorizontalAlignmentLeft }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlockHorizontalAlignment for PageBlockHorizontalAlignmentLeft {}


impl PageBlockHorizontalAlignmentLeft {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockHorizontalAlignmentLeft".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



