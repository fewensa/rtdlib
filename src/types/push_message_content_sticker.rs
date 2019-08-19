
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a sticker. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentSticker
  /// Message content; may be null.
  sticker: Option<Sticker>,
  /// Emoji corresponding to the sticker; may be empty.
  emoji: Option<String>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentSticker {}
impl RObject for PushMessageContentSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentSticker" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentSticker {}


impl PushMessageContentSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentSticker".to_string(),
      sticker: None,
      emoji: None,
      is_pinned: None,
      
    }
  }
  
  pub fn sticker(&self) -> Option<Sticker> { self.sticker.clone() }
  #[doc(hidden)] pub fn _set_sticker(&mut self, sticker: Sticker) -> &mut Self { self.sticker = Some(sticker); self }
  
  pub fn emoji(&self) -> Option<String> { self.emoji.clone() }
  #[doc(hidden)] pub fn _set_emoji(&mut self, emoji: String) -> &mut Self { self.emoji = Some(emoji); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



