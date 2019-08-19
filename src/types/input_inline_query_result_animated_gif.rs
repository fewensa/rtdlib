
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to an animated GIF. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultAnimatedGif {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultAnimatedGif
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Title of the query result.
  title: Option<String>,
  /// URL of the static result thumbnail (JPEG or GIF), if it exists.
  thumbnail_url: Option<String>,
  /// The URL of the GIF-file (file size must not exceed 1MB).
  gif_url: Option<String>,
  /// Duration of the GIF, in seconds.
  gif_duration: Option<i32>,
  /// Width of the GIF.
  gif_width: Option<i32>,
  /// Height of the GIF.
  gif_height: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAnimation, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultAnimatedGif {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultAnimatedGif {}
impl RObject for InputInlineQueryResultAnimatedGif {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultAnimatedGif" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultAnimatedGif }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultAnimatedGif {}


impl InputInlineQueryResultAnimatedGif {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultAnimatedGif".to_string(),
      id: None,
      title: None,
      thumbnail_url: None,
      gif_url: None,
      gif_duration: None,
      gif_width: None,
      gif_height: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn thumbnail_url(&self) -> Option<String> { self.thumbnail_url.clone() }
  #[doc(hidden)] pub fn _set_thumbnail_url(&mut self, thumbnail_url: String) -> &mut Self { self.thumbnail_url = Some(thumbnail_url); self }
  
  pub fn gif_url(&self) -> Option<String> { self.gif_url.clone() }
  #[doc(hidden)] pub fn _set_gif_url(&mut self, gif_url: String) -> &mut Self { self.gif_url = Some(gif_url); self }
  
  pub fn gif_duration(&self) -> Option<i32> { self.gif_duration.clone() }
  #[doc(hidden)] pub fn _set_gif_duration(&mut self, gif_duration: i32) -> &mut Self { self.gif_duration = Some(gif_duration); self }
  
  pub fn gif_width(&self) -> Option<i32> { self.gif_width.clone() }
  #[doc(hidden)] pub fn _set_gif_width(&mut self, gif_width: i32) -> &mut Self { self.gif_width = Some(gif_width); self }
  
  pub fn gif_height(&self) -> Option<i32> { self.gif_height.clone() }
  #[doc(hidden)] pub fn _set_gif_height(&mut self, gif_height: i32) -> &mut Self { self.gif_height = Some(gif_height); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



