
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is recording a video. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionRecordingVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionRecordingVideo
  
}



impl Object for ChatActionRecordingVideo {}
impl RObject for ChatActionRecordingVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionRecordingVideo" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionRecordingVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionRecordingVideo {}


impl ChatActionRecordingVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionRecordingVideo".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



