
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat photo was edited. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentChatChangePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentChatChangePhoto
  
}



impl Object for PushMessageContentChatChangePhoto {}
impl RObject for PushMessageContentChatChangePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatChangePhoto" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentChatChangePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentChatChangePhoto {}


impl PushMessageContentChatChangePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentChatChangePhoto".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



