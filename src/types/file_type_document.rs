
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a document. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeDocument
  
}



impl Object for FileTypeDocument {}
impl RObject for FileTypeDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeDocument" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeDocument {}


impl FileTypeDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeDocument".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



