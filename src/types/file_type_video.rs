
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a video. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeVideo
  
}



impl Object for FileTypeVideo {}
impl RObject for FileTypeVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeVideo" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeVideo {}


impl FileTypeVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeVideo".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



