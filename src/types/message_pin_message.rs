
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message has been pinned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePinMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messagePinMessage
  /// Identifier of the pinned message, can be an identifier of a deleted message or 0.
  message_id: Option<i64>,
  
}



impl Object for MessagePinMessage {}
impl RObject for MessagePinMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePinMessage" }
  fn td_type(&self) -> RTDType { RTDType::MessagePinMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessagePinMessage {}


impl MessagePinMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messagePinMessage".to_string(),
      message_id: None,
      
    }
  }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



