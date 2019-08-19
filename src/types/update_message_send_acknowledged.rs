
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageSendAcknowledged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateMessageSendAcknowledged
  /// The chat identifier of the sent message.
  chat_id: Option<i64>,
  /// A temporary message identifier.
  message_id: Option<i64>,
  
}



impl Object for UpdateMessageSendAcknowledged {}
impl RObject for UpdateMessageSendAcknowledged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateMessageSendAcknowledged" }
  fn td_type(&self) -> RTDType { RTDType::UpdateMessageSendAcknowledged }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateMessageSendAcknowledged {}


impl UpdateMessageSendAcknowledged {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateMessageSendAcknowledged".to_string(),
      chat_id: None,
      message_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



