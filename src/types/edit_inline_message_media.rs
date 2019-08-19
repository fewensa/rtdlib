
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditInlineMessageMedia {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editInlineMessageMedia
  /// Inline message identifier.
  inline_message_id: Option<String>,
  /// The new message reply markup; for bots only.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for EditInlineMessageMedia {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EditInlineMessageMedia {}
impl RObject for EditInlineMessageMedia {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageMedia" }
  fn td_type(&self) -> RTDType { RTDType::EditInlineMessageMedia }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditInlineMessageMedia {}


impl EditInlineMessageMedia {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editInlineMessageMedia".to_string(),
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



