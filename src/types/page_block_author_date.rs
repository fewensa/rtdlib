
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The author and publishing date of a page. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockAuthorDate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockAuthorDate
  /// Author.
  author: Option<Box<RichText>>,
  /// Point in time (Unix timestamp) when the article was published; 0 if unknown.
  publish_date: Option<i32>,
  
}


impl Clone for PageBlockAuthorDate {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockAuthorDate {}
impl RObject for PageBlockAuthorDate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockAuthorDate" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockAuthorDate }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockAuthorDate {}


impl PageBlockAuthorDate {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockAuthorDate".to_string(),
      author: None,
      publish_date: None,
      
    }
  }
  
  pub fn author(&self) -> Option<Box<RichText>> { self.author.clone() }
  #[doc(hidden)] pub fn _set_author(&mut self, author: Box<RichText>) -> &mut Self { self.author = Some(author); self }
  
  pub fn publish_date(&self) -> Option<i32> { self.publish_date.clone() }
  #[doc(hidden)] pub fn _set_publish_date(&mut self, publish_date: i32) -> &mut Self { self.publish_date = Some(publish_date); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



