
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains part of the list of user photos. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePhotos {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userProfilePhotos
  /// Total number of user profile photos.
  total_count: Option<i32>,
  /// A list of photos.
  photos: Option<Vec<UserProfilePhoto>>,
  
}



impl Object for UserProfilePhotos {}
impl RObject for UserProfilePhotos {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userProfilePhotos" }
  fn td_type(&self) -> RTDType { RTDType::UserProfilePhotos }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl UserProfilePhotos {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userProfilePhotos".to_string(),
      total_count: None,
      photos: None,
      
    }
  }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn photos(&self) -> Option<Vec<UserProfilePhoto>> { self.photos.clone() }
  #[doc(hidden)] pub fn _set_photos(&mut self, photos: Vec<UserProfilePhoto>) -> &mut Self { self.photos = Some(photos); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



