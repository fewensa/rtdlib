
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A photo message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentPhoto
  /// Message content; may be null.
  photo: Option<Photo>,
  /// Photo caption.
  caption: Option<String>,
  /// True, if the photo is secret.
  is_secret: Option<bool>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentPhoto {}
impl RObject for PushMessageContentPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentPhoto" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentPhoto {}


impl PushMessageContentPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentPhoto".to_string(),
      photo: None,
      caption: None,
      is_secret: None,
      is_pinned: None,
      
    }
  }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn caption(&self) -> Option<String> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: String) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn is_secret(&self) -> Option<bool> { self.is_secret.clone() }
  #[doc(hidden)] pub fn _set_is_secret(&mut self, is_secret: bool) -> &mut Self { self.is_secret = Some(is_secret); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



