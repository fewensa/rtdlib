
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a custom keyboard layout to quickly reply to bots. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyMarkupShowKeyboard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // replyMarkupShowKeyboard
  /// A list of rows of bot keyboard buttons.
  rows: Option<Vec<Vec<KeyboardButton>>>,
  /// True, if the client needs to resize the keyboard vertically.
  resize_keyboard: Option<bool>,
  /// True, if the client needs to hide the keyboard after use.
  one_time: Option<bool>,
  /// True, if the keyboard must automatically be shown to the current user. For outgoing messages, specify true to show the keyboard only for the mentioned users and for the target user of a reply.
  is_personal: Option<bool>,
  
}



impl Object for ReplyMarkupShowKeyboard {}
impl RObject for ReplyMarkupShowKeyboard {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "replyMarkupShowKeyboard" }
  fn td_type(&self) -> RTDType { RTDType::ReplyMarkupShowKeyboard }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ReplyMarkup for ReplyMarkupShowKeyboard {}


impl ReplyMarkupShowKeyboard {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "replyMarkupShowKeyboard".to_string(),
      rows: None,
      resize_keyboard: None,
      one_time: None,
      is_personal: None,
      
    }
  }
  
  pub fn rows(&self) -> Option<Vec<Vec<KeyboardButton>>> { self.rows.clone() }
  #[doc(hidden)] pub fn _set_rows(&mut self, rows: Vec<Vec<KeyboardButton>>) -> &mut Self { self.rows = Some(rows); self }
  
  pub fn resize_keyboard(&self) -> Option<bool> { self.resize_keyboard.clone() }
  #[doc(hidden)] pub fn _set_resize_keyboard(&mut self, resize_keyboard: bool) -> &mut Self { self.resize_keyboard = Some(resize_keyboard); self }
  
  pub fn one_time(&self) -> Option<bool> { self.one_time.clone() }
  #[doc(hidden)] pub fn _set_one_time(&mut self, one_time: bool) -> &mut Self { self.one_time = Some(one_time); self }
  
  pub fn is_personal(&self) -> Option<bool> { self.is_personal.clone() }
  #[doc(hidden)] pub fn _set_is_personal(&mut self, is_personal: bool) -> &mut Self { self.is_personal = Some(is_personal); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



