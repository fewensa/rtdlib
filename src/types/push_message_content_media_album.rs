
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A media album. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentMediaAlbum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentMediaAlbum
  /// Number of messages in the album.
  total_count: Option<i32>,
  /// True, if the album has at least one photo.
  has_photos: Option<bool>,
  /// True, if the album has at least one video.
  has_videos: Option<bool>,
  
}



impl Object for PushMessageContentMediaAlbum {}
impl RObject for PushMessageContentMediaAlbum {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentMediaAlbum" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentMediaAlbum }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentMediaAlbum {}


impl PushMessageContentMediaAlbum {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentMediaAlbum".to_string(),
      total_count: None,
      has_photos: None,
      has_videos: None,
      
    }
  }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn has_photos(&self) -> Option<bool> { self.has_photos.clone() }
  #[doc(hidden)] pub fn _set_has_photos(&mut self, has_photos: bool) -> &mut Self { self.has_photos = Some(has_photos); self }
  
  pub fn has_videos(&self) -> Option<bool> { self.has_videos.clone() }
  #[doc(hidden)] pub fn _set_has_videos(&mut self, has_videos: bool) -> &mut Self { self.has_videos = Some(has_videos); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



