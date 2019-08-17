
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new member joined the chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventMemberJoined {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventMemberJoined
  
}



impl Object for ChatEventMemberJoined {}
impl RObject for ChatEventMemberJoined {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberJoined" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventMemberJoined }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventMemberJoined {}


impl ChatEventMemberJoined {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventMemberJoined".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



