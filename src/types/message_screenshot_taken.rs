
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A screenshot of a message in the chat has been taken. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageScreenshotTaken {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageScreenshotTaken
  
}



impl Object for MessageScreenshotTaken {}
impl RObject for MessageScreenshotTaken {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageScreenshotTaken" }
  fn td_type(&self) -> RTDType { RTDType::MessageScreenshotTaken }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageScreenshotTaken {}


impl MessageScreenshotTaken {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageScreenshotTaken".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



