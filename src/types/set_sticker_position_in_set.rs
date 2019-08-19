
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot.
#[derive(Debug, Serialize, Deserialize)]
pub struct SetStickerPositionInSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setStickerPositionInSet
  /// Sticker.
  sticker: Option<Box<InputFile>>,
  /// New position of the sticker in the set, zero-based.
  position: Option<i32>,
  
}


impl Clone for SetStickerPositionInSet {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetStickerPositionInSet {}
impl RObject for SetStickerPositionInSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setStickerPositionInSet" }
  fn td_type(&self) -> RTDType { RTDType::SetStickerPositionInSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetStickerPositionInSet {}


impl SetStickerPositionInSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setStickerPositionInSet".to_string(),
      sticker: None,
      position: None,
      
    }
  }
  
  pub fn sticker(&self) -> Option<Box<InputFile>> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn position(&self) -> Option<i32> { self.position.clone() }
  #[doc(hidden)] pub fn _set_position(&mut self, position: i32) -> &mut Self { self.position = Some(position); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



