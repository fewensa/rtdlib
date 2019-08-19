
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is recording a video note. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionRecordingVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionRecordingVideoNote
  
}



impl Object for ChatActionRecordingVideoNote {}
impl RObject for ChatActionRecordingVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionRecordingVideoNote" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionRecordingVideoNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionRecordingVideoNote {}


impl ChatActionRecordingVideoNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionRecordingVideoNote".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



