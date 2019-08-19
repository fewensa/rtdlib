
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The list of favorite stickers was updated. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFavoriteStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateFavoriteStickers
  /// The new list of file identifiers of favorite stickers.
  sticker_ids: Option<Vec<i32>>,
  
}



impl Object for UpdateFavoriteStickers {}
impl RObject for UpdateFavoriteStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateFavoriteStickers" }
  fn td_type(&self) -> RTDType { RTDType::UpdateFavoriteStickers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateFavoriteStickers {}


impl UpdateFavoriteStickers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateFavoriteStickers".to_string(),
      sticker_ids: None,
      
    }
  }
  
  pub fn sticker_ids(&self) -> Option<Vec<i32>> { self.sticker_ids.clone() }
  #[doc(hidden)] pub fn _set_sticker_ids(&mut self, sticker_ids: Vec<i32>) -> &mut Self { self.sticker_ids = Some(sticker_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



