
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A screenshot of a message in the chat has been taken. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentScreenshotTaken {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentScreenshotTaken
  
}



impl Object for PushMessageContentScreenshotTaken {}
impl RObject for PushMessageContentScreenshotTaken {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentScreenshotTaken" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentScreenshotTaken }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentScreenshotTaken {}


impl PushMessageContentScreenshotTaken {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentScreenshotTaken".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



