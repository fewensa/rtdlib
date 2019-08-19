
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A photo message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessagePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessagePhoto
  /// Photo to send.
  photo: Option<Box<InputFile>>,
  /// Photo thumbnail to be sent, this is sent to the other party in secret chats only.
  thumbnail: Option<InputThumbnail>,
  /// File identifiers of the stickers added to the photo, if applicable.
  added_sticker_file_ids: Option<Vec<i32>>,
  /// Photo width.
  width: Option<i32>,
  /// Photo height.
  height: Option<i32>,
  /// Photo caption; 0-GetOption("message_caption_length_max") characters.
  caption: Option<FormattedText>,
  /// Photo TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats.
  ttl: Option<i32>,
  
}


impl Clone for InputMessagePhoto {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputMessagePhoto {}
impl RObject for InputMessagePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessagePhoto" }
  fn td_type(&self) -> RTDType { RTDType::InputMessagePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessagePhoto {}


impl InputMessagePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessagePhoto".to_string(),
      photo: None,
      thumbnail: None,
      added_sticker_file_ids: None,
      width: None,
      height: None,
      caption: None,
      ttl: None,
      
    }
  }
  
  pub fn photo(&self) -> Option<Box<InputFile>> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Box<InputFile>) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn thumbnail(&self) -> Option<InputThumbnail> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn added_sticker_file_ids(&self) -> Option<Vec<i32>> { self.added_sticker_file_ids.clone() }
  #[doc(hidden)] pub fn _set_added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self { self.added_sticker_file_ids = Some(added_sticker_file_ids); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn ttl(&self) -> Option<i32> { self.ttl.clone() }
  #[doc(hidden)] pub fn _set_ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



