
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A URL linking to a sticker set. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMeUrlTypeStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // tMeUrlTypeStickerSet
  /// Identifier of the sticker set.
  sticker_set_id: Option<i64>,
  
}



impl Object for TMeUrlTypeStickerSet {}
impl RObject for TMeUrlTypeStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrlTypeStickerSet" }
  fn td_type(&self) -> RTDType { RTDType::TMeUrlTypeStickerSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TMeUrlType for TMeUrlTypeStickerSet {}


impl TMeUrlTypeStickerSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "tMeUrlTypeStickerSet".to_string(),
      sticker_set_id: None,
      
    }
  }
  
  pub fn sticker_set_id(&self) -> Option<i64> { self.sticker_set_id.clone() }
  #[doc(hidden)] pub fn _set_sticker_set_id(&mut self, sticker_set_id: i64) -> &mut Self { self.sticker_set_id = Some(sticker_set_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



