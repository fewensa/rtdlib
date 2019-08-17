
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits the caption of an inline message sent via a bot; for bots only.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditInlineMessageCaption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editInlineMessageCaption
  /// Inline message identifier.
  inline_message_id: Option<String>,
  /// The new message reply markup.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// New message content caption; 0-GetOption("message_caption_length_max") characters.
  caption: Option<FormattedText>,
  
}


impl Clone for EditInlineMessageCaption {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EditInlineMessageCaption {}
impl RObject for EditInlineMessageCaption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageCaption" }
  fn td_type(&self) -> RTDType { RTDType::EditInlineMessageCaption }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditInlineMessageCaption {}


impl EditInlineMessageCaption {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editInlineMessageCaption".to_string(),
      inline_message_id: None,
      reply_markup: None,
      caption: None,
      
    }
  }
  
  pub fn inline_message_id(&self) -> Option<String> { self.inline_message_id.clone() }
  #[doc(hidden)] pub fn _set_inline_message_id(&mut self, inline_message_id: String) -> &mut Self { self.inline_message_id = Some(inline_message_id); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



