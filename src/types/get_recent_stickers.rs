
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of recently used stickers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getRecentStickers
  /// Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers.
  is_attached: Option<bool>,
  
}



impl Object for GetRecentStickers {}
impl RObject for GetRecentStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRecentStickers" }
  fn td_type(&self) -> RTDType { RTDType::GetRecentStickers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetRecentStickers {}


impl GetRecentStickers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getRecentStickers".to_string(),
      is_attached: None,
      
    }
  }
  
  pub fn is_attached(&self) -> Option<bool> { self.is_attached.clone() }
  #[doc(hidden)] pub fn _set_is_attached(&mut self, is_attached: bool) -> &mut Self { self.is_attached = Some(is_attached); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



