
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The list of installed sticker sets was updated. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstalledStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateInstalledStickerSets
  /// True, if the list of installed mask sticker sets was updated.
  is_masks: Option<bool>,
  /// The new list of installed ordinary sticker sets.
  sticker_set_ids: Option<Vec<i64>>,
  
}



impl Object for UpdateInstalledStickerSets {}
impl RObject for UpdateInstalledStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateInstalledStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::UpdateInstalledStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateInstalledStickerSets {}


impl UpdateInstalledStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateInstalledStickerSets".to_string(),
      is_masks: None,
      sticker_set_ids: None,
      
    }
  }
  
  pub fn is_masks(&self) -> Option<bool> { self.is_masks.clone() }
  #[doc(hidden)] pub fn _set_is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn sticker_set_ids(&self) -> Option<Vec<i64>> { self.sticker_set_ids.clone() }
  #[doc(hidden)] pub fn _set_sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self { self.sticker_set_ids = Some(sticker_set_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



