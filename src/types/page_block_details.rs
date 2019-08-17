
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A collapsible block. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockDetails
  /// Always visible heading for the block.
  header: Option<Box<RichText>>,
  /// Block contents.
  page_blocks: Option<Vec<Box<PageBlock>>>,
  /// True, if the block is open by default.
  is_open: Option<bool>,
  
}


impl Clone for PageBlockDetails {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockDetails {}
impl RObject for PageBlockDetails {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockDetails" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockDetails }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockDetails {}


impl PageBlockDetails {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockDetails".to_string(),
      header: None,
      page_blocks: None,
      is_open: None,
      
    }
  }
  
  pub fn header(&self) -> Option<Box<RichText>> { self.header.clone() }
  #[doc(hidden)] pub fn _set_header(&mut self, header: Box<RichText>) -> &mut Self { self.header = Some(header); self }
  
  pub fn page_blocks(&self) -> Option<Vec<Box<PageBlock>>> { self.page_blocks.clone() }
  #[doc(hidden)] pub fn _set_page_blocks(&mut self, page_blocks: Vec<Box<PageBlock>>) -> &mut Self { self.page_blocks = Some(page_blocks); self }
  
  pub fn is_open(&self) -> Option<bool> { self.is_open.clone() }
  #[doc(hidden)] pub fn _set_is_open(&mut self, is_open: bool) -> &mut Self { self.is_open = Some(is_open); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



