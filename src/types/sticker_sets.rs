
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a list of sticker sets. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // stickerSets
  /// Approximate total number of sticker sets found.
  total_count: Option<i32>,
  /// List of sticker sets.
  sets: Option<Vec<StickerSetInfo>>,
  
}



impl Object for StickerSets {}
impl RObject for StickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickerSets" }
  fn td_type(&self) -> RTDType { RTDType::StickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl StickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "stickerSets".to_string(),
      total_count: None,
      sets: None,
      
    }
  }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn sets(&self) -> Option<Vec<StickerSetInfo>> { self.sets.clone() }
  #[doc(hidden)] pub fn _set_sets(&mut self, sets: Vec<StickerSetInfo>) -> &mut Self { self.sets = Some(sets); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



