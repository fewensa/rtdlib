
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new member joined the chat by invite link. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentChatJoinByLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentChatJoinByLink
  
}



impl Object for PushMessageContentChatJoinByLink {}
impl RObject for PushMessageContentChatJoinByLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatJoinByLink" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentChatJoinByLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentChatJoinByLink {}


impl PushMessageContentChatJoinByLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentChatJoinByLink".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



