
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is an animation. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeAnimation
  
}



impl Object for FileTypeAnimation {}
impl RObject for FileTypeAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeAnimation" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeAnimation {}


impl FileTypeAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeAnimation".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



