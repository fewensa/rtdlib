
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the order of installed sticker sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderInstalledStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // reorderInstalledStickerSets
  /// Pass true to change the order of mask sticker sets; pass false to change the order of ordinary sticker sets.
  is_masks: Option<bool>,
  /// Identifiers of installed sticker sets in the new correct order.
  sticker_set_ids: Option<Vec<i64>>,
  
}



impl Object for ReorderInstalledStickerSets {}
impl RObject for ReorderInstalledStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "reorderInstalledStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::ReorderInstalledStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ReorderInstalledStickerSets {}


impl ReorderInstalledStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "reorderInstalledStickerSets".to_string(),
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



