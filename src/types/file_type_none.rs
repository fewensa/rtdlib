
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The data is not a file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeNone
  
}



impl Object for FileTypeNone {}
impl RObject for FileTypeNone {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeNone" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeNone }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeNone {}


impl FileTypeNone {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeNone".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



