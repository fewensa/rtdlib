
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes a sticker from the list of favorite stickers.
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveFavoriteSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeFavoriteSticker
  /// Sticker file to delete from the list.
  sticker: Option<Box<InputFile>>,
  
}


impl Clone for RemoveFavoriteSticker {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RemoveFavoriteSticker {}
impl RObject for RemoveFavoriteSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeFavoriteSticker" }
  fn td_type(&self) -> RTDType { RTDType::RemoveFavoriteSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveFavoriteSticker {}


impl RemoveFavoriteSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeFavoriteSticker".to_string(),
      sticker: None,
      
    }
  }
  
  pub fn sticker(&self) -> Option<Box<InputFile>> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



