
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits the reply markup of an inline message sent via a bot; for bots only.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditInlineMessageReplyMarkup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editInlineMessageReplyMarkup
  /// Inline message identifier.
  inline_message_id: Option<String>,
  /// The new message reply markup.
  reply_markup: Option<Box<ReplyMarkup>>,
  
}


impl Clone for EditInlineMessageReplyMarkup {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EditInlineMessageReplyMarkup {}
impl RObject for EditInlineMessageReplyMarkup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageReplyMarkup" }
  fn td_type(&self) -> RTDType { RTDType::EditInlineMessageReplyMarkup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditInlineMessageReplyMarkup {}


impl EditInlineMessageReplyMarkup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editInlineMessageReplyMarkup".to_string(),
      inline_message_id: None,
      reply_markup: None,
      
    }
  }
  
  pub fn inline_message_id(&self) -> Option<String> { self.inline_message_id.clone() }
  #[doc(hidden)] pub fn _set_inline_message_id(&mut self, inline_message_id: String) -> &mut Self { self.inline_message_id = Some(inline_message_id); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



