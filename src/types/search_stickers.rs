
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for stickers from public sticker sets that correspond to a given emoji.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchStickers
  /// String representation of emoji; must be non-empty.
  emoji: Option<String>,
  /// Maximum number of stickers to be returned.
  limit: Option<i32>,
  
}



impl Object for SearchStickers {}
impl RObject for SearchStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchStickers" }
  fn td_type(&self) -> RTDType { RTDType::SearchStickers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchStickers {}


impl SearchStickers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchStickers".to_string(),
      emoji: None,
      limit: None,
      
    }
  }
  
  pub fn emoji(&self) -> Option<String> { self.emoji.clone() }
  #[doc(hidden)] pub fn _set_emoji(&mut self, emoji: String) -> &mut Self { self.emoji = Some(emoji); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



