
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of installed sticker sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstalledStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getInstalledStickerSets
  /// Pass true to return mask sticker sets; pass false to return ordinary sticker sets.
  is_masks: Option<bool>,
  
}



impl Object for GetInstalledStickerSets {}
impl RObject for GetInstalledStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInstalledStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::GetInstalledStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetInstalledStickerSets {}


impl GetInstalledStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getInstalledStickerSets".to_string(),
      is_masks: None,
      
    }
  }
  
  pub fn is_masks(&self) -> Option<bool> { self.is_masks.clone() }
  #[doc(hidden)] pub fn _set_is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



