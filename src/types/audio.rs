
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes an audio file. Audio is usually in MP3 format. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // audio
  /// Duration of the audio, in seconds; as defined by the sender.
  duration: Option<i32>,
  /// Title of the audio; as defined by the sender.
  title: Option<String>,
  /// Performer of the audio; as defined by the sender.
  performer: Option<String>,
  /// Original name of the file; as defined by the sender.
  file_name: Option<String>,
  /// The MIME type of the file; as defined by the sender.
  mime_type: Option<String>,
  /// The thumbnail of the album cover; as defined by the sender. The full size thumbnail should be extracted from the downloaded file; may be null.
  album_cover_thumbnail: Option<PhotoSize>,
  /// File containing the audio.
  audio: Option<File>,
  
}



impl Object for Audio {}
impl RObject for Audio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "audio" }
  fn td_type(&self) -> RTDType { RTDType::Audio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Audio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "audio".to_string(),
      duration: None,
      title: None,
      performer: None,
      file_name: None,
      mime_type: None,
      album_cover_thumbnail: None,
      audio: None,
      
    }
  }
  
  pub fn duration(&self) -> Option<i32> { self.duration.clone() }
  #[doc(hidden)] pub fn _set_duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn performer(&self) -> Option<String> { self.performer.clone() }
  #[doc(hidden)] pub fn _set_performer(&mut self, performer: String) -> &mut Self { self.performer = Some(performer); self }
  
  pub fn file_name(&self) -> Option<String> { self.file_name.clone() }
  #[doc(hidden)] pub fn _set_file_name(&mut self, file_name: String) -> &mut Self { self.file_name = Some(file_name); self }
  
  pub fn mime_type(&self) -> Option<String> { self.mime_type.clone() }
  #[doc(hidden)] pub fn _set_mime_type(&mut self, mime_type: String) -> &mut Self { self.mime_type = Some(mime_type); self }
  
  pub fn album_cover_thumbnail(&self) -> Option<PhotoSize> { self.album_cover_thumbnail.clone() }
  #[doc(hidden)] pub fn _set_album_cover_thumbnail(&mut self, album_cover_thumbnail: PhotoSize) -> &mut Self { self.album_cover_thumbnail = Some(album_cover_thumbnail); self }
  
  pub fn audio(&self) -> Option<File> { self.audio.clone() }
  #[doc(hidden)] pub fn _set_audio(&mut self, audio: File) -> &mut Self { self.audio = Some(audio); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



