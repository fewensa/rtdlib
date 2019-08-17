
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat photo was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventPhotoChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventPhotoChanged
  /// Previous chat photo value; may be null.
  old_photo: Option<ChatPhoto>,
  /// New chat photo value; may be null.
  new_photo: Option<ChatPhoto>,
  
}



impl Object for ChatEventPhotoChanged {}
impl RObject for ChatEventPhotoChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventPhotoChanged" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventPhotoChanged }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventPhotoChanged {}


impl ChatEventPhotoChanged {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventPhotoChanged".to_string(),
      old_photo: None,
      new_photo: None,
      
    }
  }
  
  pub fn old_photo(&self) -> Option<ChatPhoto> { self.old_photo.clone() }
  #[doc(hidden)] pub fn _set_old_photo(&mut self, old_photo: ChatPhoto) -> &mut Self { self.old_photo = Some(old_photo); self }
  
  pub fn new_photo(&self) -> Option<ChatPhoto> { self.new_photo.clone() }
  #[doc(hidden)] pub fn _set_new_photo(&mut self, new_photo: ChatPhoto) -> &mut Self { self.new_photo = Some(new_photo); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



