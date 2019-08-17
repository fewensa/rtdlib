
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns emoji corresponding to a sticker.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetStickerEmojis {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getStickerEmojis
  /// Sticker file identifier.
  sticker: Option<Box<InputFile>>,
  
}


impl Clone for GetStickerEmojis {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetStickerEmojis {}
impl RObject for GetStickerEmojis {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStickerEmojis" }
  fn td_type(&self) -> RTDType { RTDType::GetStickerEmojis }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetStickerEmojis {}


impl GetStickerEmojis {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getStickerEmojis".to_string(),
      sticker: None,
      
    }
  }
  
  pub fn sticker(&self) -> Option<Box<InputFile>> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



