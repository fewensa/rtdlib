
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a poll. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messagePoll
  /// Poll.
  poll: Option<Poll>,
  
}



impl Object for MessagePoll {}
impl RObject for MessagePoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePoll" }
  fn td_type(&self) -> RTDType { RTDType::MessagePoll }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessagePoll {}


impl MessagePoll {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messagePoll".to_string(),
      poll: None,
      
    }
  }
  
  pub fn poll(&self) -> Option<Poll> { self.poll.clone() }
  #[doc(hidden)] pub fn _set_poll(&mut self, poll: Poll) -> &mut Self { self.poll = Some(poll); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



