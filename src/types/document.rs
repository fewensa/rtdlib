
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a document of any type. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // document
  /// Original name of the file; as defined by the sender.
  file_name: Option<String>,
  /// MIME type of the file; as defined by the sender.
  mime_type: Option<String>,
  /// Document thumbnail; as defined by the sender; may be null.
  thumbnail: Option<PhotoSize>,
  /// File containing the document.
  document: Option<File>,
  
}



impl Object for Document {}
impl RObject for Document {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "document" }
  fn td_type(&self) -> RTDType { RTDType::Document }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Document {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "document".to_string(),
      file_name: None,
      mime_type: None,
      thumbnail: None,
      document: None,
      
    }
  }
  
  pub fn file_name(&self) -> Option<String> { self.file_name.clone() }
  #[doc(hidden)] pub fn _set_file_name(&mut self, file_name: String) -> &mut Self { self.file_name = Some(file_name); self }
  
  pub fn mime_type(&self) -> Option<String> { self.mime_type.clone() }
  #[doc(hidden)] pub fn _set_mime_type(&mut self, mime_type: String) -> &mut Self { self.mime_type = Some(mime_type); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn document(&self) -> Option<File> { self.document.clone() }
  #[doc(hidden)] pub fn _set_document(&mut self, document: File) -> &mut Self { self.document = Some(document); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



