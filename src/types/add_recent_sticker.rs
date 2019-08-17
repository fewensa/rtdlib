
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddRecentSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addRecentSticker
  /// Pass true to add the sticker to the list of stickers recently attached to photo or video files; pass false to add the sticker to the list of recently sent stickers.
  is_attached: Option<bool>,
  /// Sticker file to add.
  sticker: Option<Box<InputFile>>,
  
}


impl Clone for AddRecentSticker {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for AddRecentSticker {}
impl RObject for AddRecentSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addRecentSticker" }
  fn td_type(&self) -> RTDType { RTDType::AddRecentSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddRecentSticker {}


impl AddRecentSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addRecentSticker".to_string(),
      is_attached: None,
      sticker: None,
      
    }
  }
  
  pub fn is_attached(&self) -> Option<bool> { self.is_attached.clone() }
  #[doc(hidden)] pub fn _set_is_attached(&mut self, is_attached: bool) -> &mut Self { self.is_attached = Some(is_attached); self }
  
  pub fn sticker(&self) -> Option<Box<InputFile>> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



