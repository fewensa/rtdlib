
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Installs/uninstalls or activates/archives a sticker set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // changeStickerSet
  /// Identifier of the sticker set.
  set_id: Option<i64>,
  /// The new value of is_installed.
  is_installed: Option<bool>,
  /// The new value of is_archived. A sticker set can't be installed and archived simultaneously.
  is_archived: Option<bool>,
  
}



impl Object for ChangeStickerSet {}
impl RObject for ChangeStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "changeStickerSet" }
  fn td_type(&self) -> RTDType { RTDType::ChangeStickerSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ChangeStickerSet {}


impl ChangeStickerSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "changeStickerSet".to_string(),
      set_id: None,
      is_installed: None,
      is_archived: None,
      
    }
  }
  
  pub fn set_id(&self) -> Option<i64> { self.set_id.clone() }
  #[doc(hidden)] pub fn _set_set_id(&mut self, set_id: i64) -> &mut Self { self.set_id = Some(set_id); self }
  
  pub fn is_installed(&self) -> Option<bool> { self.is_installed.clone() }
  #[doc(hidden)] pub fn _set_is_installed(&mut self, is_installed: bool) -> &mut Self { self.is_installed = Some(is_installed); self }
  
  pub fn is_archived(&self) -> Option<bool> { self.is_archived.clone() }
  #[doc(hidden)] pub fn _set_is_archived(&mut self, is_archived: bool) -> &mut Self { self.is_archived = Some(is_archived); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



