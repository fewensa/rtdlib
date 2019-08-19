
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Informs the server that some trending sticker sets have been viewed by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewTrendingStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // viewTrendingStickerSets
  /// Identifiers of viewed trending sticker sets.
  sticker_set_ids: Option<Vec<i64>>,
  
}



impl Object for ViewTrendingStickerSets {}
impl RObject for ViewTrendingStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "viewTrendingStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::ViewTrendingStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ViewTrendingStickerSets {}


impl ViewTrendingStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "viewTrendingStickerSets".to_string(),
      sticker_set_ids: None,
      
    }
  }
  
  pub fn sticker_set_ids(&self) -> Option<Vec<i64>> { self.sticker_set_ids.clone() }
  #[doc(hidden)] pub fn _set_sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self { self.sticker_set_ids = Some(sticker_set_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



