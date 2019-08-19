
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Related articles. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockRelatedArticles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockRelatedArticles
  /// Block header.
  header: Option<Box<RichText>>,
  /// List of related articles.
  articles: Option<Vec<PageBlockRelatedArticle>>,
  
}


impl Clone for PageBlockRelatedArticles {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockRelatedArticles {}
impl RObject for PageBlockRelatedArticles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockRelatedArticles" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockRelatedArticles }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockRelatedArticles {}


impl PageBlockRelatedArticles {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockRelatedArticles".to_string(),
      header: None,
      articles: None,
      
    }
  }
  
  pub fn header(&self) -> Option<Box<RichText>> { self.header.clone() }
  #[doc(hidden)] pub fn _set_header(&mut self, header: Box<RichText>) -> &mut Self { self.header = Some(header); self }
  
  pub fn articles(&self) -> Option<Vec<PageBlockRelatedArticle>> { self.articles.clone() }
  #[doc(hidden)] pub fn _set_articles(&mut self, articles: Vec<PageBlockRelatedArticle>) -> &mut Self { self.articles = Some(articles); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



