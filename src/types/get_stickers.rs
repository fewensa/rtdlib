
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is not empty, favorite and recently used stickers may also be returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getStickers
  /// String representation of emoji. If empty, returns all known installed stickers.
  emoji: Option<String>,
  /// Maximum number of stickers to be returned.
  limit: Option<i32>,
  
}



impl Object for GetStickers {}
impl RObject for GetStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStickers" }
  fn td_type(&self) -> RTDType { RTDType::GetStickers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetStickers {}


impl GetStickers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getStickers".to_string(),
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



