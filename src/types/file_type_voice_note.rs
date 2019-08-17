
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a voice note. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeVoiceNote
  
}



impl Object for FileTypeVoiceNote {}
impl RObject for FileTypeVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeVoiceNote" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeVoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeVoiceNote {}


impl FileTypeVoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeVoiceNote".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



