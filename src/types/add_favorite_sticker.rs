
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddFavoriteSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addFavoriteSticker
  /// Sticker file to add.
  sticker: Option<Box<InputFile>>,
  
}


impl Clone for AddFavoriteSticker {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for AddFavoriteSticker {}
impl RObject for AddFavoriteSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addFavoriteSticker" }
  fn td_type(&self) -> RTDType { RTDType::AddFavoriteSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddFavoriteSticker {}


impl AddFavoriteSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addFavoriteSticker".to_string(),
      sticker: None,
      
    }
  }
  
  pub fn sticker(&self) -> Option<Box<InputFile>> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



