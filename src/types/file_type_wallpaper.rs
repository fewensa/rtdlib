
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file is a wallpaper. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeWallpaper {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // fileTypeWallpaper
  
}



impl Object for FileTypeWallpaper {}
impl RObject for FileTypeWallpaper {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeWallpaper" }
  fn td_type(&self) -> RTDType { RTDType::FileTypeWallpaper }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl FileType for FileTypeWallpaper {}


impl FileTypeWallpaper {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "fileTypeWallpaper".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



