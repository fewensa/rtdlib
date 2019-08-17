
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A header. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockHeader {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockHeader
  /// Header.
  header: Option<Box<RichText>>,
  
}


impl Clone for PageBlockHeader {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockHeader {}
impl RObject for PageBlockHeader {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockHeader" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockHeader }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockHeader {}


impl PageBlockHeader {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockHeader".to_string(),
      header: None,
      
    }
  }
  
  pub fn header(&self) -> Option<Box<RichText>> { self.header.clone() }
  #[doc(hidden)] pub fn _set_header(&mut self, header: Box<RichText>) -> &mut Self { self.header = Some(header); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



