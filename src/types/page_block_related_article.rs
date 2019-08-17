
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a related article. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockRelatedArticle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockRelatedArticle
  /// Related article URL.
  url: Option<String>,
  /// Article title; may be empty.
  title: Option<String>,
  /// Article description; may be empty.
  description: Option<String>,
  /// Article photo; may be null.
  photo: Option<Photo>,
  /// Article author; may be empty.
  author: Option<String>,
  /// Point in time (Unix timestamp) when the article was published; 0 if unknown.
  publish_date: Option<i32>,
  
}



impl Object for PageBlockRelatedArticle {}
impl RObject for PageBlockRelatedArticle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockRelatedArticle" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockRelatedArticle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PageBlockRelatedArticle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockRelatedArticle".to_string(),
      url: None,
      title: None,
      description: None,
      photo: None,
      author: None,
      publish_date: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn author(&self) -> Option<String> { self.author.clone() }
  #[doc(hidden)] pub fn _set_author(&mut self, author: String) -> &mut Self { self.author = Some(author); self }
  
  pub fn publish_date(&self) -> Option<i32> { self.publish_date.clone() }
  #[doc(hidden)] pub fn _set_publish_date(&mut self, publish_date: i32) -> &mut Self { self.publish_date = Some(publish_date); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



