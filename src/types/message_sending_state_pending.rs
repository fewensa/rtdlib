
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The message is being sent now, but has not yet been delivered to the server. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSendingStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageSendingStatePending
  
}



impl Object for MessageSendingStatePending {}
impl RObject for MessageSendingStatePending {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSendingStatePending" }
  fn td_type(&self) -> RTDType { RTDType::MessageSendingStatePending }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageSendingState for MessageSendingStatePending {}


impl MessageSendingStatePending {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageSendingStatePending".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



