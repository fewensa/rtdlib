
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a profile photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeProfilePhoto
  
}



impl Object for FileTypeProfilePhoto {}
impl RObject for FileTypeProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeProfilePhoto" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeProfilePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeProfilePhoto {}


impl FileTypeProfilePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeProfilePhoto".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



