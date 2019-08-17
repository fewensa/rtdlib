
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A sticker message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageSticker
  /// Message content.
  sticker: Option<Sticker>,
  
}



impl Object for MessageSticker {}
impl RObject for MessageSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSticker" }
  fn td_type(&self) -> RTDType { RTDType::MessageSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageSticker {}


impl MessageSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageSticker".to_string(),
      sticker: None,
      
    }
  }
  
  pub fn sticker(&self) -> Option<Sticker> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Sticker) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



