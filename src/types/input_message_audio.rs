
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An audio message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessageAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageAudio
  /// Audio file to be sent.
  audio: Option<Box<InputFile>>,
  /// Thumbnail of the cover for the album, if available.
  album_cover_thumbnail: Option<InputThumbnail>,
  /// Duration of the audio, in seconds; may be replaced by the server.
  duration: Option<i32>,
  /// Title of the audio; 0-64 characters; may be replaced by the server.
  title: Option<String>,
  /// Performer of the audio; 0-64 characters, may be replaced by the server.
  performer: Option<String>,
  /// Audio caption; 0-GetOption("message_caption_length_max") characters.
  caption: Option<FormattedText>,
  
}


impl Clone for InputMessageAudio {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputMessageAudio {}
impl RObject for InputMessageAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageAudio" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageAudio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageAudio {}


impl InputMessageAudio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageAudio".to_string(),
      audio: None,
      album_cover_thumbnail: None,
      duration: None,
      title: None,
      performer: None,
      caption: None,
      
    }
  }
  
  pub fn audio(&self) -> Option<Box<InputFile>> { self.audio.clone() }
  #[doc(hidden)] pub fn _set_audio(&mut self, audio: Box<InputFile>) -> &mut Self { self.audio = Some(audio); self }
  
  pub fn album_cover_thumbnail(&self) -> Option<InputThumbnail> { self.album_cover_thumbnail.clone() }
  #[doc(hidden)] pub fn _set_album_cover_thumbnail(&mut self, album_cover_thumbnail: InputThumbnail) -> &mut Self { self.album_cover_thumbnail = Some(album_cover_thumbnail); self }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn performer(&self) -> Option<String> { self.performer.clone() }
  #[doc(hidden)] pub fn _set_performer(&mut self, performer: String) -> &mut Self { self.performer = Some(performer); self }
  
  pub fn caption(&self) -> Option<FormattedText> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



