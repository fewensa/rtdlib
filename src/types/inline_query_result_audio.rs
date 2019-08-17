
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents an audio file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultAudio
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Audio file.
  audio: Option<Audio>,
  
}



impl Object for InlineQueryResultAudio {}
impl RObject for InlineQueryResultAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultAudio" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultAudio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultAudio {}


impl InlineQueryResultAudio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultAudio".to_string(),
      id: None,
      audio: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn audio(&self) -> Option<Audio> { self.audio.clone() }
  #[doc(hidden)] pub fn _set_audio(&mut self, audio: Audio) -> &mut Self { self.audio = Some(audio); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



