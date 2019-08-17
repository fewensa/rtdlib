
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A small image inside the text. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextIcon {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextIcon
  /// The image represented as a document. The image can be in GIF, JPEG or PNG format.
  document: Option<Document>,
  /// Width of a bounding box in which the image should be shown, 0 if unknown.
  width: Option<i32>,
  /// Height of a bounding box in which the image should be shown, 0 if unknown.
  height: Option<i32>,
  
}



impl Object for RichTextIcon {}
impl RObject for RichTextIcon {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextIcon" }
  fn td_type(&self) -> RTDType { RTDType::RichTextIcon }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextIcon {}


impl RichTextIcon {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextIcon".to_string(),
      document: None,
      width: None,
      height: None,
      
    }
  }
  
  pub fn document(&self) -> Option<Document> { self.document.clone() }
  #[doc(hidden)] pub fn _set_document(&mut self, document: Document) -> &mut Self { self.document = Some(document); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



