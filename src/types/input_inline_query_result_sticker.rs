
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to a WEBP sticker. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultSticker
  /// Unique identifier of the query result.
  id: Option<String>,
  /// URL of the sticker thumbnail, if it exists.
  thumbnail_url: Option<String>,
  /// The URL of the WEBP sticker (sticker file size must not exceed 5MB).
  sticker_url: Option<String>,
  /// Width of the sticker.
  sticker_width: Option<i32>,
  /// Height of the sticker.
  sticker_height: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, inputMessageSticker, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultSticker {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultSticker {}
impl RObject for InputInlineQueryResultSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultSticker" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultSticker }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultSticker {}


impl InputInlineQueryResultSticker {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultSticker".to_string(),
      id: None,
      thumbnail_url: None,
      sticker_url: None,
      sticker_width: None,
      sticker_height: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn thumbnail_url(&self) -> Option<String> { self.thumbnail_url.clone() }
  #[doc(hidden)] pub fn _set_thumbnail_url(&mut self, thumbnail_url: String) -> &mut Self { self.thumbnail_url = Some(thumbnail_url); self }
  
  pub fn sticker_url(&self) -> Option<String> { self.sticker_url.clone() }
  #[doc(hidden)] pub fn _set_sticker_url(&mut self, sticker_url: String) -> &mut Self { self.sticker_url = Some(sticker_url); self }
  
  pub fn sticker_width(&self) -> Option<i32> { self.sticker_width.clone() }
  #[doc(hidden)] pub fn _set_sticker_width(&mut self, sticker_width: i32) -> &mut Self { self.sticker_width = Some(sticker_width); self }
  
  pub fn sticker_height(&self) -> Option<i32> { self.sticker_height.clone() }
  #[doc(hidden)] pub fn _set_sticker_height(&mut self, sticker_height: i32) -> &mut Self { self.sticker_height = Some(sticker_height); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



