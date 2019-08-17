
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a sticker. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sticker
  /// The identifier of the sticker set to which the sticker belongs; 0 if none.
  set_id: Option<String>,
  /// Sticker width; as defined by the sender.
  width: Option<i32>,
  /// Sticker height; as defined by the sender.
  height: Option<i32>,
  /// Emoji corresponding to the sticker.
  emoji: Option<String>,
  /// True, if the sticker is a mask.
  is_mask: Option<bool>,
  /// Position where the mask should be placed; may be null.
  mask_position: Option<MaskPosition>,
  /// Sticker thumbnail in WEBP or JPEG format; may be null.
  thumbnail: Option<PhotoSize>,
  /// File containing the sticker.
  sticker: Option<File>,
  
}



impl Object for Sticker {}
impl RObject for Sticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sticker" }
  fn td_type(&self) -> RTDType { RTDType::Sticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Sticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sticker".to_string(),
      set_id: None,
      width: None,
      height: None,
      emoji: None,
      is_mask: None,
      mask_position: None,
      thumbnail: None,
      sticker: None,
      
    }
  }
  
  pub fn set_id(&self) -> Option<String> { self.set_id.clone() }
  #[doc(hidden)] pub fn _set_set_id(&mut self, set_id: String) -> &mut Self { self.set_id = Some(set_id); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn emoji(&self) -> Option<String> { self.emoji.clone() }
  #[doc(hidden)] pub fn _set_emoji(&mut self, emoji: String) -> &mut Self { self.emoji = Some(emoji); self }
  
  pub fn is_mask(&self) -> Option<bool> { self.is_mask.clone() }
  #[doc(hidden)] pub fn _set_is_mask(&mut self, is_mask: bool) -> &mut Self { self.is_mask = Some(is_mask); self }
  
  pub fn mask_position(&self) -> Option<MaskPosition> { self.mask_position.clone() }
  #[doc(hidden)] pub fn _set_mask_position(&mut self, mask_position: MaskPosition) -> &mut Self { self.mask_position = Some(mask_position); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn sticker(&self) -> Option<File> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: File) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



