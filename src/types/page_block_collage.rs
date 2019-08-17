
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A collage. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockCollage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockCollage
  /// Collage item contents.
  page_blocks: Option<Vec<Box<PageBlock>>>,
  /// Block caption.
  caption: Option<PageBlockCaption>,
  
}


impl Clone for PageBlockCollage {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockCollage {}
impl RObject for PageBlockCollage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockCollage" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockCollage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockCollage {}


impl PageBlockCollage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockCollage".to_string(),
      page_blocks: None,
      caption: None,
      
    }
  }
  
  pub fn page_blocks(&self) -> Option<Vec<Box<PageBlock>>> { self.page_blocks.clone() }
  #[doc(hidden)] pub fn _set_page_blocks(&mut self, page_blocks: Vec<Box<PageBlock>>) -> &mut Self { self.page_blocks = Some(page_blocks); self }
  
  pub fn caption(&self) -> Option<PageBlockCaption> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: PageBlockCaption) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



