
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file was sent to a secret chat (the file type is not known to the server). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeSecret {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeSecret
  
}



impl Object for FileTypeSecret {}
impl RObject for FileTypeSecret {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeSecret" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeSecret }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeSecret {}


impl FileTypeSecret {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeSecret".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



