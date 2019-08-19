
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A document message (general file). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageDocument
  /// Message content.
  document: Option<Document>,
  /// Document caption.
  caption: Option<FormattedText>,
  
}



impl Object for MessageDocument {}
impl RObject for MessageDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageDocument" }
  fn td_type(&self) -> RTDType { RTDType::MessageDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageDocument {}


impl MessageDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageDocument".to_string(),
      document: None,
      caption: None,
      
    }
  }
  
  pub fn document(&self) -> Option<Document> { self.document.clone() }
  #[doc(hidden)] pub fn _set_document(&mut self, document: Document) -> &mut Self { self.document = Some(document); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



