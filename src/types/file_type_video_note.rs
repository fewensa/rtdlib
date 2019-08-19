
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a video note. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeVideoNote
  
}



impl Object for FileTypeVideoNote {}
impl RObject for FileTypeVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeVideoNote" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeVideoNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeVideoNote {}


impl FileTypeVideoNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeVideoNote".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



