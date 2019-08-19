
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to a page containing an embedded video player or a video file. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultVideo
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Title of the result.
  title: Option<String>,
  /// A short description of the result, if known.
  description: Option<String>,
  /// The URL of the video thumbnail (JPEG), if it exists.
  thumbnail_url: Option<String>,
  /// URL of the embedded video player or video file.
  video_url: Option<String>,
  /// MIME type of the content of the video URL, only "text/html" or "video/mp4" are currently supported.
  mime_type: Option<String>,
  /// Width of the video.
  video_width: Option<i32>,
  /// Height of the video.
  video_height: Option<i32>,
  /// Video duration, in seconds.
  video_duration: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageVideo, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultVideo {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultVideo {}
impl RObject for InputInlineQueryResultVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultVideo" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultVideo {}


impl InputInlineQueryResultVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultVideo".to_string(),
      id: None,
      title: None,
      description: None,
      thumbnail_url: None,
      video_url: None,
      mime_type: None,
      video_width: None,
      video_height: None,
      video_duration: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn thumbnail_url(&self) -> Option<String> { self.thumbnail_url.clone() }
  #[doc(hidden)] pub fn _set_thumbnail_url(&mut self, thumbnail_url: String) -> &mut Self { self.thumbnail_url = Some(thumbnail_url); self }
  
  pub fn video_url(&self) -> Option<String> { self.video_url.clone() }
  #[doc(hidden)] pub fn _set_video_url(&mut self, video_url: String) -> &mut Self { self.video_url = Some(video_url); self }
  
  pub fn mime_type(&self) -> Option<String> { self.mime_type.clone() }
  #[doc(hidden)] pub fn _set_mime_type(&mut self, mime_type: String) -> &mut Self { self.mime_type = Some(mime_type); self }
  
  pub fn video_width(&self) -> Option<i32> { self.video_width.clone() }
  #[doc(hidden)] pub fn _set_video_width(&mut self, video_width: i32) -> &mut Self { self.video_width = Some(video_width); self }
  
  pub fn video_height(&self) -> Option<i32> { self.video_height.clone() }
  #[doc(hidden)] pub fn _set_video_height(&mut self, video_height: i32) -> &mut Self { self.video_height = Some(video_height); self }
  
  pub fn video_duration(&self) -> Option<i32> { self.video_duration.clone() }
  #[doc(hidden)] pub fn _set_video_duration(&mut self, video_duration: i32) -> &mut Self { self.video_duration = Some(video_duration); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



