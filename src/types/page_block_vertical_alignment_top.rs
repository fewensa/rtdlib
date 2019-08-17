
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The content should be top-aligned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentTop {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockVerticalAlignmentTop
  
}



impl Object for PageBlockVerticalAlignmentTop {}
impl RObject for PageBlockVerticalAlignmentTop {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockVerticalAlignmentTop" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockVerticalAlignmentTop }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlockVerticalAlignment for PageBlockVerticalAlignmentTop {}


impl PageBlockVerticalAlignmentTop {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockVerticalAlignmentTop".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



