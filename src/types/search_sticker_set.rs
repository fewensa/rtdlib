
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for a sticker set by its name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchStickerSet
  /// Name of the sticker set.
  name: Option<String>,
  
}



impl Object for SearchStickerSet {}
impl RObject for SearchStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchStickerSet" }
  fn td_type(&self) -> RTDType { RTDType::SearchStickerSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchStickerSet {}


impl SearchStickerSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchStickerSet".to_string(),
      name: None,
      
    }
  }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



