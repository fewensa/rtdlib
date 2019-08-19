
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot.
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveStickerFromSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeStickerFromSet
  /// Sticker.
  sticker: Option<Box<InputFile>>,
  
}


impl Clone for RemoveStickerFromSet {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RemoveStickerFromSet {}
impl RObject for RemoveStickerFromSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeStickerFromSet" }
  fn td_type(&self) -> RTDType { RTDType::RemoveStickerFromSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveStickerFromSet {}


impl RemoveStickerFromSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeStickerFromSet".to_string(),
      sticker: None,
      
    }
  }
  
  pub fn sticker(&self) -> Option<Box<InputFile>> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



