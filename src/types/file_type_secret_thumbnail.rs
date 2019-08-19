
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a thumbnail of a file from a secret chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeSecretThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeSecretThumbnail
  
}



impl Object for FileTypeSecretThumbnail {}
impl RObject for FileTypeSecretThumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeSecretThumbnail" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeSecretThumbnail }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeSecretThumbnail {}


impl FileTypeSecretThumbnail {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeSecretThumbnail".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



