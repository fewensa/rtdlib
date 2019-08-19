
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The content should be middle-aligned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentMiddle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockVerticalAlignmentMiddle
  
}



impl Object for PageBlockVerticalAlignmentMiddle {}
impl RObject for PageBlockVerticalAlignmentMiddle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockVerticalAlignmentMiddle" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockVerticalAlignmentMiddle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlockVerticalAlignment for PageBlockVerticalAlignmentMiddle {}


impl PageBlockVerticalAlignmentMiddle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockVerticalAlignmentMiddle".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



