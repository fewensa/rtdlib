
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Uploads a PNG image with a sticker; for bots only; returns the uploaded file.
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadStickerFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // uploadStickerFile
  /// Sticker file owner.
  user_id: Option<i32>,
  /// PNG image with the sticker; must be up to 512 kB in size and fit in 512x512 square.
  png_sticker: Option<Box<InputFile>>,
  
}


impl Clone for UploadStickerFile {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UploadStickerFile {}
impl RObject for UploadStickerFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "uploadStickerFile" }
  fn td_type(&self) -> RTDType { RTDType::UploadStickerFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for UploadStickerFile {}


impl UploadStickerFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "uploadStickerFile".to_string(),
      user_id: None,
      png_sticker: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn png_sticker(&self) -> Option<Box<InputFile>> { self.png_sticker.clone() }
  #[doc(hidden)] pub fn _set_png_sticker(&mut self, png_sticker: Box<InputFile>) -> &mut Self { self.png_sticker = Some(png_sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



