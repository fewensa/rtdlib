
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is an audio file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeAudio
  
}



impl Object for FileTypeAudio {}
impl RObject for FileTypeAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeAudio" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeAudio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeAudio {}


impl FileTypeAudio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeAudio".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



