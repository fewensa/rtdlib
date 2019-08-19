
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The supergroup sticker set was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventStickerSetChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventStickerSetChanged
  /// Previous identifier of the chat sticker set; 0 if none.
  old_sticker_set_id: Option<i64>,
  /// New identifier of the chat sticker set; 0 if none.
  new_sticker_set_id: Option<i64>,
  
}



impl Object for ChatEventStickerSetChanged {}
impl RObject for ChatEventStickerSetChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventStickerSetChanged" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventStickerSetChanged }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventStickerSetChanged {}


impl ChatEventStickerSetChanged {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventStickerSetChanged".to_string(),
      old_sticker_set_id: None,
      new_sticker_set_id: None,
      
    }
  }
  
  pub fn old_sticker_set_id(&self) -> Option<i64> { self.old_sticker_set_id.clone() }
  #[doc(hidden)] pub fn _set_old_sticker_set_id(&mut self, old_sticker_set_id: i64) -> &mut Self { self.old_sticker_set_id = Some(old_sticker_set_id); self }
  
  pub fn new_sticker_set_id(&self) -> Option<i64> { self.new_sticker_set_id.clone() }
  #[doc(hidden)] pub fn _set_new_sticker_set_id(&mut self, new_sticker_set_id: i64) -> &mut Self { self.new_sticker_set_id = Some(new_sticker_set_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



