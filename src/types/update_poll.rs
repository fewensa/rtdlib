
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Information about a poll was updated; for bots only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updatePoll
  /// New data about the poll.
  poll: Option<Poll>,
  
}



impl Object for UpdatePoll {}
impl RObject for UpdatePoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updatePoll" }
  fn td_type(&self) -> RTDType { RTDType::UpdatePoll }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdatePoll {}


impl UpdatePoll {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updatePoll".to_string(),
      poll: None,
      
    }
  }
  
  pub fn poll(&self) -> Option<Poll> { self.poll.clone() }
  #[doc(hidden)] pub fn _set_poll(&mut self, poll: Poll) -> &mut Self { self.poll = Some(poll); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



