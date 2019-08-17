
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits the text of an inline text or game message sent via a bot; for bots only.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditInlineMessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editInlineMessageText
  /// Inline message identifier.
  inline_message_id: Option<String>,
  /// The new message reply markup.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// New text content of the message. Should be of type InputMessageText.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for EditInlineMessageText {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EditInlineMessageText {}
impl RObject for EditInlineMessageText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageText" }
  fn td_type(&self) -> RTDType { RTDType::EditInlineMessageText }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditInlineMessageText {}


impl EditInlineMessageText {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editInlineMessageText".to_string(),
      inline_message_id: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn inline_message_id(&self) -> Option<String> { self.inline_message_id.clone() }
  #[doc(hidden)] pub fn _set_inline_message_id(&mut self, inline_message_id: String) -> &mut Self { self.inline_message_id = Some(inline_message_id); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



