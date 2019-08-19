
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes a sticker from the list of recently used stickers.
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveRecentSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeRecentSticker
  /// Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers.
  is_attached: Option<bool>,
  /// Sticker file to delete.
  sticker: Option<Box<InputFile>>,
  
}


impl Clone for RemoveRecentSticker {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RemoveRecentSticker {}
impl RObject for RemoveRecentSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeRecentSticker" }
  fn td_type(&self) -> RTDType { RTDType::RemoveRecentSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveRecentSticker {}


impl RemoveRecentSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeRecentSticker".to_string(),
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



