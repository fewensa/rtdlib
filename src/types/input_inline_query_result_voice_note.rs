
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to an opus-encoded audio file within an OGG container, single channel audio. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultVoiceNote
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Title of the voice note.
  title: Option<String>,
  /// The URL of the voice note file.
  voice_note_url: Option<String>,
  /// Duration of the voice note, in seconds.
  voice_note_duration: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageVoiceNote, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultVoiceNote {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultVoiceNote {}
impl RObject for InputInlineQueryResultVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultVoiceNote" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultVoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultVoiceNote {}


impl InputInlineQueryResultVoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultVoiceNote".to_string(),
      id: None,
      title: None,
      voice_note_url: None,
      voice_note_duration: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn voice_note_url(&self) -> Option<String> { self.voice_note_url.clone() }
  #[doc(hidden)] pub fn _set_voice_note_url(&mut self, voice_note_url: String) -> &mut Self { self.voice_note_url = Some(voice_note_url); self }
  
  pub fn voice_note_duration(&self) -> Option<i32> { self.voice_note_duration.clone() }
  #[doc(hidden)] pub fn _set_voice_note_duration(&mut self, voice_note_duration: i32) -> &mut Self { self.voice_note_duration = Some(voice_note_duration); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



