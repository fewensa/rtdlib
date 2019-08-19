
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to an animated (i.e. without sound) H.264/MPEG-4 AVC video. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultAnimatedMpeg4 {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultAnimatedMpeg4
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Title of the result.
  title: Option<String>,
  /// URL of the static result thumbnail (JPEG or GIF), if it exists.
  thumbnail_url: Option<String>,
  /// The URL of the MPEG4-file (file size must not exceed 1MB).
  mpeg4_url: Option<String>,
  /// Duration of the video, in seconds.
  mpeg4_duration: Option<i32>,
  /// Width of the video.
  mpeg4_width: Option<i32>,
  /// Height of the video.
  mpeg4_height: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAnimation, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultAnimatedMpeg4 {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultAnimatedMpeg4 {}
impl RObject for InputInlineQueryResultAnimatedMpeg4 {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultAnimatedMpeg4" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultAnimatedMpeg4 }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultAnimatedMpeg4 {}


impl InputInlineQueryResultAnimatedMpeg4 {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultAnimatedMpeg4".to_string(),
      id: None,
      title: None,
      thumbnail_url: None,
      mpeg4_url: None,
      mpeg4_duration: None,
      mpeg4_width: None,
      mpeg4_height: None,
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
  
  pub fn mpeg4_url(&self) -> Option<String> { self.mpeg4_url.clone() }
  #[doc(hidden)] pub fn _set_mpeg4_url(&mut self, mpeg4_url: String) -> &mut Self { self.mpeg4_url = Some(mpeg4_url); self }
  
  pub fn mpeg4_duration(&self) -> Option<i32> { self.mpeg4_duration.clone() }
  #[doc(hidden)] pub fn _set_mpeg4_duration(&mut self, mpeg4_duration: i32) -> &mut Self { self.mpeg4_duration = Some(mpeg4_duration); self }
  
  pub fn mpeg4_width(&self) -> Option<i32> { self.mpeg4_width.clone() }
  #[doc(hidden)] pub fn _set_mpeg4_width(&mut self, mpeg4_width: i32) -> &mut Self { self.mpeg4_width = Some(mpeg4_width); self }
  
  pub fn mpeg4_height(&self) -> Option<i32> { self.mpeg4_height.clone() }
  #[doc(hidden)] pub fn _set_mpeg4_height(&mut self, mpeg4_height: i32) -> &mut Self { self.mpeg4_height = Some(mpeg4_height); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



