
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An embedded post. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockEmbeddedPost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockEmbeddedPost
  /// Web page URL.
  url: Option<String>,
  /// Post author.
  author: Option<String>,
  /// Post author photo.
  author_photo: Option<Photo>,
  /// Point in time (Unix timestamp) when the post was created; 0 if unknown.
  date: Option<i32>,
  /// Post content.
  page_blocks: Option<Vec<Box<PageBlock>>>,
  /// Post caption.
  caption: Option<PageBlockCaption>,
  
}


impl Clone for PageBlockEmbeddedPost {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockEmbeddedPost {}
impl RObject for PageBlockEmbeddedPost {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockEmbeddedPost" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockEmbeddedPost }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockEmbeddedPost {}


impl PageBlockEmbeddedPost {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockEmbeddedPost".to_string(),
      url: None,
      author: None,
      author_photo: None,
      date: None,
      page_blocks: None,
      caption: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn author(&self) -> Option<String> { self.author.clone() }
  #[doc(hidden)] pub fn _set_author(&mut self, author: String) -> &mut Self { self.author = Some(author); self }
  
  pub fn author_photo(&self) -> Option<Photo> { self.author_photo.clone() }
  #[doc(hidden)] pub fn _set_author_photo(&mut self, author_photo: Photo) -> &mut Self { self.author_photo = Some(author_photo); self }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn page_blocks(&self) -> Option<Vec<Box<PageBlock>>> { self.page_blocks.clone() }
  #[doc(hidden)] pub fn _set_page_blocks(&mut self, page_blocks: Vec<Box<PageBlock>>) -> &mut Self { self.page_blocks = Some(page_blocks); self }
  
  pub fn caption(&self) -> Option<PageBlockCaption> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: PageBlockCaption) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



