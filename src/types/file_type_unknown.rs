
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file type is not yet known. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeUnknown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeUnknown
  
}



impl Object for FileTypeUnknown {}
impl RObject for FileTypeUnknown {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeUnknown" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeUnknown }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeUnknown {}


impl FileTypeUnknown {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeUnknown".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



