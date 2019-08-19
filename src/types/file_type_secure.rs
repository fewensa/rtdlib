
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a file from Secure storage used for storing Telegram Passport files. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeSecure {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeSecure
  
}



impl Object for FileTypeSecure {}
impl RObject for FileTypeSecure {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeSecure" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeSecure }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeSecure {}


impl FileTypeSecure {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeSecure".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



