
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is uploading a document. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionUploadingDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionUploadingDocument
  /// Upload progress, as a percentage.
  progress: Option<i32>,
  
}



impl Object for ChatActionUploadingDocument {}
impl RObject for ChatActionUploadingDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionUploadingDocument" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionUploadingDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionUploadingDocument {}


impl ChatActionUploadingDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionUploadingDocument".to_string(),
      progress: None,
      
    }
  }
  
  pub fn progress(&self) -> Option<i32> { self.progress.clone() }
  #[doc(hidden)] pub fn _set_progress(&mut self, progress: i32) -> &mut Self { self.progress = Some(progress); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



