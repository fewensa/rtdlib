
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A page cover. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockCover {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockCover
  /// Cover.
  cover: Option<Box<PageBlock>>,
  
}


impl Clone for PageBlockCover {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockCover {}
impl RObject for PageBlockCover {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockCover" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockCover }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockCover {}


impl PageBlockCover {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockCover".to_string(),
      cover: None,
      
    }
  }
  
  pub fn cover(&self) -> Option<Box<PageBlock>> { self.cover.clone() }
  #[doc(hidden)] pub fn _set_cover(&mut self, cover: Box<PageBlock>) -> &mut Self { self.cover = Some(cover); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



