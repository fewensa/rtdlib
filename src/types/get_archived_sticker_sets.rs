
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of archived sticker sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArchivedStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getArchivedStickerSets
  /// Pass true to return mask stickers sets; pass false to return ordinary sticker sets.
  is_masks: Option<bool>,
  /// Identifier of the sticker set from which to return the result.
  offset_sticker_set_id: Option<i64>,
  /// Maximum number of sticker sets to return.
  limit: Option<i32>,
  
}



impl Object for GetArchivedStickerSets {}
impl RObject for GetArchivedStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getArchivedStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::GetArchivedStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetArchivedStickerSets {}


impl GetArchivedStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getArchivedStickerSets".to_string(),
      is_masks: None,
      offset_sticker_set_id: None,
      limit: None,
      
    }
  }
  
  pub fn is_masks(&self) -> Option<bool> { self.is_masks.clone() }
  #[doc(hidden)] pub fn _set_is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn offset_sticker_set_id(&self) -> Option<i64> { self.offset_sticker_set_id.clone() }
  #[doc(hidden)] pub fn _set_offset_sticker_set_id(&mut self, offset_sticker_set_id: i64) -> &mut Self { self.offset_sticker_set_id = Some(offset_sticker_set_id); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



