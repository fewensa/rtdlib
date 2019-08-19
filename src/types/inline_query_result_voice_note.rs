
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a voice note. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultVoiceNote
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Voice note.
  voice_note: Option<VoiceNote>,
  /// Title of the voice note.
  title: Option<String>,
  
}



impl Object for InlineQueryResultVoiceNote {}
impl RObject for InlineQueryResultVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultVoiceNote" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultVoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultVoiceNote {}


impl InlineQueryResultVoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultVoiceNote".to_string(),
      id: None,
      voice_note: None,
      title: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn voice_note(&self) -> Option<VoiceNote> { self.voice_note.clone() }
  #[doc(hidden)] pub fn _set_voice_note(&mut self, voice_note: VoiceNote) -> &mut Self { self.voice_note = Some(voice_note); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



