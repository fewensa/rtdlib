
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a list of all emoji corresponding to a sticker in a sticker set. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerEmojis {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // stickerEmojis
  /// List of emojis.
  emojis: Option<Vec<String>>,
  
}



impl Object for StickerEmojis {}
impl RObject for StickerEmojis {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickerEmojis" }
  fn td_type(&self) -> RTDType { RTDType::StickerEmojis }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl StickerEmojis {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "stickerEmojis".to_string(),
      emojis: None,
      
    }
  }
  
  pub fn emojis(&self) -> Option<Vec<String>> { self.emojis.clone() }
  #[doc(hidden)] pub fn _set_emojis(&mut self, emojis: Vec<String>) -> &mut Self { self.emojis = Some(emojis); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



