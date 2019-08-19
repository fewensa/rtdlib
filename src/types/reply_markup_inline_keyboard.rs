
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains an inline keyboard layout. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyMarkupInlineKeyboard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // replyMarkupInlineKeyboard
  /// A list of rows of inline keyboard buttons.
  rows: Option<Vec<Vec<InlineKeyboardButton>>>,
  
}



impl Object for ReplyMarkupInlineKeyboard {}
impl RObject for ReplyMarkupInlineKeyboard {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "replyMarkupInlineKeyboard" }
  fn td_type(&self) -> RTDType { RTDType::ReplyMarkupInlineKeyboard }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ReplyMarkup for ReplyMarkupInlineKeyboard {}


impl ReplyMarkupInlineKeyboard {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "replyMarkupInlineKeyboard".to_string(),
      rows: None,
      
    }
  }
  
  pub fn rows(&self) -> Option<Vec<Vec<InlineKeyboardButton>>> { self.rows.clone() }
  #[doc(hidden)] pub fn _set_rows(&mut self, rows: Vec<Vec<InlineKeyboardButton>>) -> &mut Self { self.rows = Some(rows); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



