
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is recording a voice note. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionRecordingVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionRecordingVoiceNote
  
}



impl Object for ChatActionRecordingVoiceNote {}
impl RObject for ChatActionRecordingVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionRecordingVoiceNote" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionRecordingVoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionRecordingVoiceNote {}


impl ChatActionRecordingVoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionRecordingVoiceNote".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



