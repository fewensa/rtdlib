
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A sticker message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessageSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageSticker
  /// Sticker to be sent.
  sticker: Option<Box<InputFile>>,
  /// Sticker thumbnail, if available.
  thumbnail: Option<InputThumbnail>,
  /// Sticker width.
  width: Option<i32>,
  /// Sticker height.
  height: Option<i32>,
  
}


impl Clone for InputMessageSticker {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputMessageSticker {}
impl RObject for InputMessageSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageSticker" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageSticker {}


impl InputMessageSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageSticker".to_string(),
      sticker: None,
      thumbnail: None,
      width: None,
      height: None,
      
    }
  }
  
  pub fn sticker(&self) -> Option<Box<InputFile>> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn thumbnail(&self) -> Option<InputThumbnail> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



