
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A list of data blocks. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockList {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockList
  /// The items of the list.
  items: Option<Vec<PageBlockListItem>>,
  
}



impl Object for PageBlockList {}
impl RObject for PageBlockList {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockList" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockList }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockList {}


impl PageBlockList {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockList".to_string(),
      items: None,
      
    }
  }
  
  pub fn items(&self) -> Option<Vec<PageBlockListItem>> { self.items.clone() }
  #[doc(hidden)] pub fn _set_items(&mut self, items: Vec<PageBlockListItem>) -> &mut Self { self.items = Some(items); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



