
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a sticker. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeSticker
  
}



impl Object for FileTypeSticker {}
impl RObject for FileTypeSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeSticker" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeSticker {}


impl FileTypeSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeSticker".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



