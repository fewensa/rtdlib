
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a sticker that should be added to a sticker set. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputSticker
  /// PNG image with the sticker; must be up to 512 kB in size and fit in a 512x512 square.
  png_sticker: Option<Box<InputFile>>,
  /// Emoji corresponding to the sticker.
  emojis: Option<String>,
  /// For masks, position where the mask should be placed; may be null.
  mask_position: Option<MaskPosition>,
  
}


impl Clone for InputSticker {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputSticker {}
impl RObject for InputSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputSticker" }
  fn td_type(&self) -> RTDType { RTDType::InputSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl InputSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputSticker".to_string(),
      png_sticker: None,
      emojis: None,
      mask_position: None,
      
    }
  }
  
  pub fn png_sticker(&self) -> Option<Box<InputFile>> { self.png_sticker.clone() }
  #[doc(hidden)] pub fn _set_png_sticker(&mut self, png_sticker: Box<InputFile>) -> &mut Self { self.png_sticker = Some(png_sticker); self }
  
  pub fn emojis(&self) -> Option<String> { self.emojis.clone() }
  #[doc(hidden)] pub fn _set_emojis(&mut self, emojis: String) -> &mut Self { self.emojis = Some(emojis); self }
  
  pub fn mask_position(&self) -> Option<MaskPosition> { self.mask_position.clone() }
  #[doc(hidden)] pub fn _set_mask_position(&mut self, mask_position: MaskPosition) -> &mut Self { self.mask_position = Some(mask_position); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



