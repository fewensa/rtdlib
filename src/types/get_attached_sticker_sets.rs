
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of sticker sets attached to a file. Currently only photos and videos can have attached sticker sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttachedStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getAttachedStickerSets
  /// File identifier.
  file_id: Option<i32>,
  
}



impl Object for GetAttachedStickerSets {}
impl RObject for GetAttachedStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getAttachedStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::GetAttachedStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetAttachedStickerSets {}


impl GetAttachedStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getAttachedStickerSets".to_string(),
      file_id: None,
      
    }
  }
  
  pub fn file_id(&self) -> Option<i32> { self.file_id.clone() }
  #[doc(hidden)] pub fn _set_file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



