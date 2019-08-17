
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a list of stickers. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // stickers
  /// List of stickers.
  stickers: Option<Vec<Sticker>>,
  
}



impl Object for Stickers {}
impl RObject for Stickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickers" }
  fn td_type(&self) -> RTDType { RTDType::Stickers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Stickers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "stickers".to_string(),
      stickers: None,
      
    }
  }
  
  pub fn stickers(&self) -> Option<Vec<Sticker>> { self.stickers.clone() }
  #[doc(hidden)] pub fn _set_stickers(&mut self, stickers: Vec<Sticker>) -> &mut Self { self.stickers = Some(stickers); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



