
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The list of trending sticker sets was updated or some of them were viewed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTrendingStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateTrendingStickerSets
  /// The new list of trending sticker sets.
  sticker_sets: Option<StickerSets>,
  
}



impl Object for UpdateTrendingStickerSets {}
impl RObject for UpdateTrendingStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateTrendingStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::UpdateTrendingStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateTrendingStickerSets {}


impl UpdateTrendingStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateTrendingStickerSets".to_string(),
      sticker_sets: None,
      
    }
  }
  
  pub fn sticker_sets(&self) -> Option<StickerSets> { self.sticker_sets.clone() }
  #[doc(hidden)] pub fn _set_sticker_sets(&mut self, sticker_sets: StickerSets) -> &mut Self { self.sticker_sets = Some(sticker_sets); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



