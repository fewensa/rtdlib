
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockAnchor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockAnchor
  /// Name of the anchor.
  name: Option<String>,
  
}



impl Object for PageBlockAnchor {}
impl RObject for PageBlockAnchor {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockAnchor" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockAnchor }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockAnchor {}


impl PageBlockAnchor {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockAnchor".to_string(),
      name: None,
      
    }
  }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



