
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A document message (a general file). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentDocument
  /// Message content; may be null.
  document: Option<Document>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentDocument {}
impl RObject for PushMessageContentDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentDocument" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentDocument {}


impl PushMessageContentDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentDocument".to_string(),
      document: None,
      is_pinned: None,
      
    }
  }
  
  pub fn document(&self) -> Option<Document> { self.document.clone() }
  #[doc(hidden)] pub fn _set_document(&mut self, document: Document) -> &mut Self { self.document = Some(document); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



