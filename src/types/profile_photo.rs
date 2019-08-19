
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a user profile photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // profilePhoto
  /// Photo identifier; 0 for an empty photo. Can be used to find a photo in a list of userProfilePhotos.
  id: Option<String>,
  /// A small (160x160) user profile photo.
  small: Option<File>,
  /// A big (640x640) user profile photo.
  big: Option<File>,
  
}



impl Object for ProfilePhoto {}
impl RObject for ProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "profilePhoto" }
  fn td_type(&self) -> RTDType { RTDType::ProfilePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ProfilePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "profilePhoto".to_string(),
      id: None,
      small: None,
      big: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn small(&self) -> Option<File> { self.small.clone() }
  #[doc(hidden)] pub fn _set_small(&mut self, small: File) -> &mut Self { self.small = Some(small); self }
  
  pub fn big(&self) -> Option<File> { self.big.clone() }
  #[doc(hidden)] pub fn _set_big(&mut self, big: File) -> &mut Self { self.big = Some(big); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



