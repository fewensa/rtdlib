
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to an article or web page. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultArticle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultArticle
  /// Unique identifier of the query result.
  id: Option<String>,
  /// URL of the result, if it exists.
  url: Option<String>,
  /// True, if the URL must be not shown.
  hide_url: Option<bool>,
  /// Title of the result.
  title: Option<String>,
  /// A short description of the result.
  description: Option<String>,
  /// URL of the result thumbnail, if it exists.
  thumbnail_url: Option<String>,
  /// Thumbnail width, if known.
  thumbnail_width: Option<i32>,
  /// Thumbnail height, if known.
  thumbnail_height: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultArticle {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultArticle {}
impl RObject for InputInlineQueryResultArticle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultArticle" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultArticle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultArticle {}


impl InputInlineQueryResultArticle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultArticle".to_string(),
      id: None,
      url: None,
      hide_url: None,
      title: None,
      description: None,
      thumbnail_url: None,
      thumbnail_width: None,
      thumbnail_height: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn hide_url(&self) -> Option<bool> { self.hide_url.clone() }
  #[doc(hidden)] pub fn _set_hide_url(&mut self, hide_url: bool) -> &mut Self { self.hide_url = Some(hide_url); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn thumbnail_url(&self) -> Option<String> { self.thumbnail_url.clone() }
  #[doc(hidden)] pub fn _set_thumbnail_url(&mut self, thumbnail_url: String) -> &mut Self { self.thumbnail_url = Some(thumbnail_url); self }
  
  pub fn thumbnail_width(&self) -> Option<i32> { self.thumbnail_width.clone() }
  #[doc(hidden)] pub fn _set_thumbnail_width(&mut self, thumbnail_width: i32) -> &mut Self { self.thumbnail_width = Some(thumbnail_width); self }
  
  pub fn thumbnail_height(&self) -> Option<i32> { self.thumbnail_height.clone() }
  #[doc(hidden)] pub fn _set_thumbnail_height(&mut self, thumbnail_height: i32) -> &mut Self { self.thumbnail_height = Some(thumbnail_height); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



