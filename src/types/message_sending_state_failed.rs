
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The message failed to be sent. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSendingStateFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageSendingStateFailed
  
}



impl Object for MessageSendingStateFailed {}
impl RObject for MessageSendingStateFailed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSendingStateFailed" }
  fn td_type(&self) -> RTDType { RTDType::MessageSendingStateFailed }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageSendingState for MessageSendingStateFailed {}


impl MessageSendingStateFailed {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageSendingStateFailed".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



