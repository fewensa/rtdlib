
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A photo message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messagePhoto
  /// Message content.
  photo: Option<Photo>,
  /// Photo caption.
  caption: Option<FormattedText>,
  /// True, if the photo must be blurred and must be shown only while tapped.
  is_secret: Option<bool>,
  
}



impl Object for MessagePhoto {}
impl RObject for MessagePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePhoto" }
  fn td_type(&self) -> RTDType { RTDType::MessagePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessagePhoto {}


impl MessagePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messagePhoto".to_string(),
      photo: None,
      caption: None,
      is_secret: None,
      
    }
  }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn is_secret(&self) -> Option<bool> { self.is_secret.clone() }
  #[doc(hidden)] pub fn _set_is_secret(&mut self, is_secret: bool) -> &mut Self { self.is_secret = Some(is_secret); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



