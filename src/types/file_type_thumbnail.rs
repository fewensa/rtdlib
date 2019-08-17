
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a thumbnail of another file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeThumbnail
  
}



impl Object for FileTypeThumbnail {}
impl RObject for FileTypeThumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeThumbnail" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeThumbnail }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeThumbnail {}


impl FileTypeThumbnail {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeThumbnail".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



