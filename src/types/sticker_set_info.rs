
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents short information about a sticker set. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerSetInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // stickerSetInfo
  /// Identifier of the sticker set.
  id: Option<i64>,
  /// Title of the sticker set.
  title: Option<String>,
  /// Name of the sticker set.
  name: Option<String>,
  /// True, if the sticker set has been installed by current user.
  is_installed: Option<bool>,
  /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously.
  is_archived: Option<bool>,
  /// True, if the sticker set is official.
  is_official: Option<bool>,
  /// True, if the stickers in the set are masks.
  is_masks: Option<bool>,
  /// True for already viewed trending sticker sets.
  is_viewed: Option<bool>,
  /// Total number of stickers in the set.
  size: Option<i32>,
  /// Contains up to the first 5 stickers from the set, depending on the context. If the client needs more stickers the full set should be requested.
  covers: Option<Vec<Sticker>>,
  
}



impl Object for StickerSetInfo {}
impl RObject for StickerSetInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickerSetInfo" }
  fn td_type(&self) -> RTDType { RTDType::StickerSetInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl StickerSetInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "stickerSetInfo".to_string(),
      id: None,
      title: None,
      name: None,
      is_installed: None,
      is_archived: None,
      is_official: None,
      is_masks: None,
      is_viewed: None,
      size: None,
      covers: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn is_installed(&self) -> Option<bool> { self.is_installed.clone() }
  #[doc(hidden)] pub fn _set_is_installed(&mut self, is_installed: bool) -> &mut Self { self.is_installed = Some(is_installed); self }
  
  pub fn is_archived(&self) -> Option<bool> { self.is_archived.clone() }
  #[doc(hidden)] pub fn _set_is_archived(&mut self, is_archived: bool) -> &mut Self { self.is_archived = Some(is_archived); self }
  
  pub fn is_official(&self) -> Option<bool> { self.is_official.clone() }
  #[doc(hidden)] pub fn _set_is_official(&mut self, is_official: bool) -> &mut Self { self.is_official = Some(is_official); self }
  
  pub fn is_masks(&self) -> Option<bool> { self.is_masks.clone() }
  #[doc(hidden)] pub fn _set_is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn is_viewed(&self) -> Option<bool> { self.is_viewed.clone() }
  #[doc(hidden)] pub fn _set_is_viewed(&mut self, is_viewed: bool) -> &mut Self { self.is_viewed = Some(is_viewed); self }
  
  pub fn size(&self) -> Option<i32> { self.size.clone() }
  #[doc(hidden)] pub fn _set_size(&mut self, size: i32) -> &mut Self { self.size = Some(size); self }
  
  pub fn covers(&self) -> Option<Vec<Sticker>> { self.covers.clone() }
  #[doc(hidden)] pub fn _set_covers(&mut self, covers: Vec<Sticker>) -> &mut Self { self.covers = Some(covers); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



