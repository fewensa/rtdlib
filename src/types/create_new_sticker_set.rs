
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Creates a new sticker set; for bots only. Returns the newly created sticker set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createNewStickerSet
  /// Sticker set owner.
  user_id: Option<i32>,
  /// Sticker set title; 1-64 characters.
  title: Option<String>,
  /// Sticker set name. Can contain only English letters, digits and underscores. Must end with "by<bot username>" (<bot_username> is case insensitive); 1-64 characters.
  name: Option<String>,
  /// True, if stickers are masks.
  is_masks: Option<bool>,
  /// List of stickers to be added to the set.
  stickers: Option<Vec<InputSticker>>,
  
}



impl Object for CreateNewStickerSet {}
impl RObject for CreateNewStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createNewStickerSet" }
  fn td_type(&self) -> RTDType { RTDType::CreateNewStickerSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateNewStickerSet {}


impl CreateNewStickerSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createNewStickerSet".to_string(),
      user_id: None,
      title: None,
      name: None,
      is_masks: None,
      stickers: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn is_masks(&self) -> Option<bool> { self.is_masks.clone() }
  #[doc(hidden)] pub fn _set_is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn stickers(&self) -> Option<Vec<InputSticker>> { self.stickers.clone() }
  #[doc(hidden)] pub fn _set_stickers(&mut self, stickers: Vec<InputSticker>) -> &mut Self { self.stickers = Some(stickers); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



