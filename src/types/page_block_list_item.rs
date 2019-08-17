
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes an item of a list page block. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockListItem {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockListItem
  /// Item label.
  label: Option<String>,
  /// Item blocks.
  page_blocks: Option<Vec<Box<PageBlock>>>,
  
}


impl Clone for PageBlockListItem {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockListItem {}
impl RObject for PageBlockListItem {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockListItem" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockListItem }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PageBlockListItem {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockListItem".to_string(),
      label: None,
      page_blocks: None,
      
    }
  }
  
  pub fn label(&self) -> Option<String> { self.label.clone() }
  #[doc(hidden)] pub fn _set_label(&mut self, label: String) -> &mut Self { self.label = Some(label); self }
  
  pub fn page_blocks(&self) -> Option<Vec<Box<PageBlock>>> { self.page_blocks.clone() }
  #[doc(hidden)] pub fn _set_page_blocks(&mut self, page_blocks: Vec<Box<PageBlock>>) -> &mut Self { self.page_blocks = Some(page_blocks); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



