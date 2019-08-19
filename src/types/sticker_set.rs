
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a sticker set. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // stickerSet
  /// Identifier of the sticker set.
  id: Option<i64>,
  /// Title of the sticker set.
  title: Option<String>,
  /// Name of the sticker set.
  name: Option<String>,
  /// True, if the sticker set has been installed by the current user.
  is_installed: Option<bool>,
  /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously.
  is_archived: Option<bool>,
  /// True, if the sticker set is official.
  is_official: Option<bool>,
  /// True, if the stickers in the set are masks.
  is_masks: Option<bool>,
  /// True for already viewed trending sticker sets.
  is_viewed: Option<bool>,
  /// List of stickers in this set.
  stickers: Option<Vec<Sticker>>,
  /// A list of emoji corresponding to the stickers in the same order.
  emojis: Option<Vec<StickerEmojis>>,
  
}



impl Object for StickerSet {}
impl RObject for StickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickerSet" }
  fn td_type(&self) -> RTDType { RTDType::StickerSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl StickerSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "stickerSet".to_string(),
      id: None,
      title: None,
      name: None,
      is_installed: None,
      is_archived: None,
      is_official: None,
      is_masks: None,
      is_viewed: None,
      stickers: None,
      emojis: None,
      
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
  
  pub fn stickers(&self) -> Option<Vec<Sticker>> { self.stickers.clone() }
  #[doc(hidden)] pub fn _set_stickers(&mut self, stickers: Vec<Sticker>) -> &mut Self { self.stickers = Some(stickers); self }
  
  pub fn emojis(&self) -> Option<Vec<StickerEmojis>> { self.emojis.clone() }
  #[doc(hidden)] pub fn _set_emojis(&mut self, emojis: Vec<StickerEmojis>) -> &mut Self { self.emojis = Some(emojis); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



