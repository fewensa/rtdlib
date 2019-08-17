
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a document. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultDocument
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Document.
  document: Option<Document>,
  /// Document title.
  title: Option<String>,
  /// Document description.
  description: Option<String>,
  
}



impl Object for InlineQueryResultDocument {}
impl RObject for InlineQueryResultDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultDocument" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultDocument {}


impl InlineQueryResultDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultDocument".to_string(),
      id: None,
      document: None,
      title: None,
      description: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn document(&self) -> Option<Document> { self.document.clone() }
  #[doc(hidden)] pub fn _set_document(&mut self, document: Document) -> &mut Self { self.document = Some(document); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



