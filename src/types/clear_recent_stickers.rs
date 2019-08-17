
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Clears the list of recently used stickers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearRecentStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // clearRecentStickers
  /// Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers.
  is_attached: Option<bool>,
  
}



impl Object for ClearRecentStickers {}
impl RObject for ClearRecentStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "clearRecentStickers" }
  fn td_type(&self) -> RTDType { RTDType::ClearRecentStickers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ClearRecentStickers {}


impl ClearRecentStickers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "clearRecentStickers".to_string(),
      is_attached: None,
      
    }
  }
  
  pub fn is_attached(&self) -> Option<bool> { self.is_attached.clone() }
  #[doc(hidden)] pub fn _set_is_attached(&mut self, is_attached: bool) -> &mut Self { self.is_attached = Some(is_attached); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



