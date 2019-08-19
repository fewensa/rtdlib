
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a sticker set by its identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getStickerSet
  /// Identifier of the sticker set.
  set_id: Option<i64>,
  
}



impl Object for GetStickerSet {}
impl RObject for GetStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStickerSet" }
  fn td_type(&self) -> RTDType { RTDType::GetStickerSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetStickerSet {}


impl GetStickerSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getStickerSet".to_string(),
      set_id: None,
      
    }
  }
  
  pub fn set_id(&self) -> Option<i64> { self.set_id.clone() }
  #[doc(hidden)] pub fn _set_set_id(&mut self, set_id: i64) -> &mut Self { self.set_id = Some(set_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



