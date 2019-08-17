
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds a new sticker to a set; for bots only. Returns the sticker set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddStickerToSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addStickerToSet
  /// Sticker set owner.
  user_id: Option<i32>,
  /// Sticker set name.
  name: Option<String>,
  /// Sticker to add to the set.
  sticker: Option<InputSticker>,
  
}



impl Object for AddStickerToSet {}
impl RObject for AddStickerToSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addStickerToSet" }
  fn td_type(&self) -> RTDType { RTDType::AddStickerToSet }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddStickerToSet {}


impl AddStickerToSet {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addStickerToSet".to_string(),
      user_id: None,
      name: None,
      sticker: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn sticker(&self) -> Option<InputSticker> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: InputSticker) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



