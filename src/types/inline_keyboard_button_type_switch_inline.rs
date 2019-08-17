
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A button that forces an inline query to the bot to be inserted in the input field. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeSwitchInline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineKeyboardButtonTypeSwitchInline
  /// Inline query to be sent to the bot.
  query: Option<String>,
  /// True, if the inline query should be sent from the current chat.
  in_current_chat: Option<bool>,
  
}



impl Object for InlineKeyboardButtonTypeSwitchInline {}
impl RObject for InlineKeyboardButtonTypeSwitchInline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeSwitchInline" }
  fn td_type(&self) -> RTDType { RTDType::InlineKeyboardButtonTypeSwitchInline }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineKeyboardButtonType for InlineKeyboardButtonTypeSwitchInline {}


impl InlineKeyboardButtonTypeSwitchInline {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineKeyboardButtonTypeSwitchInline".to_string(),
      query: None,
      in_current_chat: None,
      
    }
  }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn in_current_chat(&self) -> Option<bool> { self.in_current_chat.clone() }
  #[doc(hidden)] pub fn _set_in_current_chat(&mut self, in_current_chat: bool) -> &mut Self { self.in_current_chat = Some(in_current_chat); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



