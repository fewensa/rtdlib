
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Uploads a new profile photo for the current user. If something changes, 
#[derive(Debug, Serialize, Deserialize)]
pub struct SetProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setProfilePhoto
  /// Profile photo to set. inputFileId and inputFileRemote may still be unsupported.
  photo: Option<Box<InputFile>>,
  
}


impl Clone for SetProfilePhoto {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetProfilePhoto {}
impl RObject for SetProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setProfilePhoto" }
  fn td_type(&self) -> RTDType { RTDType::SetProfilePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetProfilePhoto {}


impl SetProfilePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setProfilePhoto".to_string(),
      photo: None,
      
    }
  }
  
  pub fn photo(&self) -> Option<Box<InputFile>> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Box<InputFile>) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



