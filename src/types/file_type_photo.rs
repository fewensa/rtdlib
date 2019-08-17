
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypePhoto
  
}



impl Object for FileTypePhoto {}
impl RObject for FileTypePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypePhoto" }
  fn td_type(&self) -> RTDType { RTDType::FileTypePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypePhoto {}


impl FileTypePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypePhoto".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



