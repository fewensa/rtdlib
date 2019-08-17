
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to an article or web page. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultArticle
  /// Unique identifier of the query result.
  id: Option<String>,
  /// URL of the result, if it exists.
  url: Option<String>,
  /// True, if the URL must be not shown.
  hide_url: Option<bool>,
  /// Title of the result.
  title: Option<String>,
  /// A short description of the result.
  description: Option<String>,
  /// Result thumbnail; may be null.
  thumbnail: Option<PhotoSize>,
  
}



impl Object for InlineQueryResultArticle {}
impl RObject for InlineQueryResultArticle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultArticle" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultArticle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultArticle {}


impl InlineQueryResultArticle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultArticle".to_string(),
      id: None,
      url: None,
      hide_url: None,
      title: None,
      description: None,
      thumbnail: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn hide_url(&self) -> Option<bool> { self.hide_url.clone() }
  #[doc(hidden)] pub fn _set_hide_url(&mut self, hide_url: bool) -> &mut Self { self.hide_url = Some(hide_url); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



