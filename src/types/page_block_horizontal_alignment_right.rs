
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The content should be right-aligned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentRight {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockHorizontalAlignmentRight
  
}



impl Object for PageBlockHorizontalAlignmentRight {}
impl RObject for PageBlockHorizontalAlignmentRight {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockHorizontalAlignmentRight" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockHorizontalAlignmentRight }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlockHorizontalAlignment for PageBlockHorizontalAlignmentRight {}


impl PageBlockHorizontalAlignmentRight {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockHorizontalAlignmentRight".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



