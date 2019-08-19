
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchStickerSets
  /// Query to search for.
  query: Option<String>,
  
}



impl Object for SearchStickerSets {}
impl RObject for SearchStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::SearchStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchStickerSets {}


impl SearchStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchStickerSets".to_string(),
      query: None,
      
    }
  }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



