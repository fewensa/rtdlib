
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to a file. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultDocument
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Title of the resulting file.
  title: Option<String>,
  /// Short description of the result, if known.
  description: Option<String>,
  /// URL of the file.
  document_url: Option<String>,
  /// MIME type of the file content; only "application/pdf" and "application/zip" are currently allowed.
  mime_type: Option<String>,
  /// The URL of the file thumbnail, if it exists.
  thumbnail_url: Option<String>,
  /// Width of the thumbnail.
  thumbnail_width: Option<i32>,
  /// Height of the thumbnail.
  thumbnail_height: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageDocument, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultDocument {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultDocument {}
impl RObject for InputInlineQueryResultDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultDocument" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultDocument {}


impl InputInlineQueryResultDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultDocument".to_string(),
      id: None,
      title: None,
      description: None,
      document_url: None,
      mime_type: None,
      thumbnail_url: None,
      thumbnail_width: None,
      thumbnail_height: None,
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
  
  pub fn document_url(&self) -> Option<String> { self.document_url.clone() }
  #[doc(hidden)] pub fn _set_document_url(&mut self, document_url: String) -> &mut Self { self.document_url = Some(document_url); self }
  
  pub fn mime_type(&self) -> Option<String> { self.mime_type.clone() }
  #[doc(hidden)] pub fn _set_mime_type(&mut self, mime_type: String) -> &mut Self { self.mime_type = Some(mime_type); self }
  
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



