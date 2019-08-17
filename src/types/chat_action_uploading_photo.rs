
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is uploading a photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionUploadingPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionUploadingPhoto
  /// Upload progress, as a percentage.
  progress: Option<i32>,
  
}



impl Object for ChatActionUploadingPhoto {}
impl RObject for ChatActionUploadingPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionUploadingPhoto" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionUploadingPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionUploadingPhoto {}


impl ChatActionUploadingPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionUploadingPhoto".to_string(),
      progress: None,
      
    }
  }
  
  pub fn progress(&self) -> Option<i32> { self.progress.clone() }
  #[doc(hidden)] pub fn _set_progress(&mut self, progress: i32) -> &mut Self { self.progress = Some(progress); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



