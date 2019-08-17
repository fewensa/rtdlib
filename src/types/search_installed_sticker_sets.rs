
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for installed sticker sets by looking for specified query in their title and name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchInstalledStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchInstalledStickerSets
  /// Pass true to return mask sticker sets; pass false to return ordinary sticker sets.
  is_masks: Option<bool>,
  /// Query to search for.
  query: Option<String>,
  /// Maximum number of sticker sets to return.
  limit: Option<i32>,
  
}



impl Object for SearchInstalledStickerSets {}
impl RObject for SearchInstalledStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchInstalledStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::SearchInstalledStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchInstalledStickerSets {}


impl SearchInstalledStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchInstalledStickerSets".to_string(),
      is_masks: None,
      query: None,
      limit: None,
      
    }
  }
  
  pub fn is_masks(&self) -> Option<bool> { self.is_masks.clone() }
  #[doc(hidden)] pub fn _set_is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



