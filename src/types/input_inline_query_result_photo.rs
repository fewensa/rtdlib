
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents link to a JPEG image. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultPhoto
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Title of the result, if known.
  title: Option<String>,
  /// A short description of the result, if known.
  description: Option<String>,
  /// URL of the photo thumbnail, if it exists.
  thumbnail_url: Option<String>,
  /// The URL of the JPEG photo (photo size must not exceed 5MB).
  photo_url: Option<String>,
  /// Width of the photo.
  photo_width: Option<i32>,
  /// Height of the photo.
  photo_height: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessagePhoto, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultPhoto {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultPhoto {}
impl RObject for InputInlineQueryResultPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultPhoto" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultPhoto {}


impl InputInlineQueryResultPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultPhoto".to_string(),
      id: None,
      title: None,
      description: None,
      thumbnail_url: None,
      photo_url: None,
      photo_width: None,
      photo_height: None,
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
  
  pub fn photo_url(&self) -> Option<String> { self.photo_url.clone() }
  #[doc(hidden)] pub fn _set_photo_url(&mut self, photo_url: String) -> &mut Self { self.photo_url = Some(photo_url); self }
  
  pub fn photo_width(&self) -> Option<i32> { self.photo_width.clone() }
  #[doc(hidden)] pub fn _set_photo_width(&mut self, photo_width: i32) -> &mut Self { self.photo_width = Some(photo_width); self }
  
  pub fn photo_height(&self) -> Option<i32> { self.photo_height.clone() }
  #[doc(hidden)] pub fn _set_photo_height(&mut self, photo_height: i32) -> &mut Self { self.photo_height = Some(photo_height); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



