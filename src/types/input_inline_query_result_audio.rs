
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a link to an MP3 audio file. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultAudio
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Title of the audio file.
  title: Option<String>,
  /// Performer of the audio file.
  performer: Option<String>,
  /// The URL of the audio file.
  audio_url: Option<String>,
  /// Audio file duration, in seconds.
  audio_duration: Option<i32>,
  /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAudio, InputMessageLocation, InputMessageVenue or InputMessageContact.
  input_message_content: Option<Box<InputMessageContent>>,
  
}


impl Clone for InputInlineQueryResultAudio {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultAudio {}
impl RObject for InputInlineQueryResultAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultAudio" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultAudio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultAudio {}


impl InputInlineQueryResultAudio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultAudio".to_string(),
      id: None,
      title: None,
      performer: None,
      audio_url: None,
      audio_duration: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn performer(&self) -> Option<String> { self.performer.clone() }
  #[doc(hidden)] pub fn _set_performer(&mut self, performer: String) -> &mut Self { self.performer = Some(performer); self }
  
  pub fn audio_url(&self) -> Option<String> { self.audio_url.clone() }
  #[doc(hidden)] pub fn _set_audio_url(&mut self, audio_url: String) -> &mut Self { self.audio_url = Some(audio_url); self }
  
  pub fn audio_duration(&self) -> Option<i32> { self.audio_duration.clone() }
  #[doc(hidden)] pub fn _set_audio_duration(&mut self, audio_duration: i32) -> &mut Self { self.audio_duration = Some(audio_duration); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn input_message_content(&self) -> Option<Box<InputMessageContent>> { self.input_message_content.clone() }
  #[doc(hidden)] pub fn _set_input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



