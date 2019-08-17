
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the sticker set of a supergroup; requires appropriate rights in the supergroup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSupergroupStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setSupergroupStickerSet
  /// Identifier of the supergroup.
  supergroup_id: Option<i32>,
  /// New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set.
  sticker_set_id: Option<i64>,
  
}



impl Object for SetSupergroupStickerSet {}
impl RObject for SetSupergroupStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setSupergroupStickerSet" }
  fn td_type(&self) -> RTDType { RTDType::SetSupergroupStickerSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetSupergroupStickerSet {}


impl SetSupergroupStickerSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setSupergroupStickerSet".to_string(),
      supergroup_id: None,
      sticker_set_id: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn sticker_set_id(&self) -> Option<i64> { self.sticker_set_id.clone() }
  #[doc(hidden)] pub fn _set_sticker_set_id(&mut self, sticker_set_id: i64) -> &mut Self { self.sticker_set_id = Some(sticker_set_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



