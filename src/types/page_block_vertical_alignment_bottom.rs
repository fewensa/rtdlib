
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The content should be bottom-aligned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentBottom {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockVerticalAlignmentBottom
  
}



impl Object for PageBlockVerticalAlignmentBottom {}
impl RObject for PageBlockVerticalAlignmentBottom {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockVerticalAlignmentBottom" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockVerticalAlignmentBottom }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlockVerticalAlignment for PageBlockVerticalAlignmentBottom {}


impl PageBlockVerticalAlignmentBottom {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockVerticalAlignmentBottom".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



