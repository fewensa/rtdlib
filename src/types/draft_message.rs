
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a message draft. 
#[derive(Debug, Serialize, Deserialize)]
pub struct DraftMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // draftMessage
  /// Identifier of the message to reply to; 0 if none.
  reply_to_message_id: Option<i64>,
  /// Content of the message draft; this should always be of type inputMessageText.
  input_message_text: Option<Box<InputMessageContent>>,
  
}


impl Clone for DraftMessage {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for DraftMessage {}
impl RObject for DraftMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "draftMessage" }
  fn td_type(&self) -> RTDType { RTDType::DraftMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl DraftMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "draftMessage".to_string(),
      reply_to_message_id: None,
      input_message_text: None,
      
    }
  }
  
  pub fn reply_to_message_id(&self) -> Option<i64> { self.reply_to_message_id.clone() }
  #[doc(hidden)] pub fn _set_reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn input_message_text(&self) -> Option<Box<InputMessageContent>> { self.input_message_text.clone() }
  #[doc(hidden)] pub fn _set_input_message_text(&mut self, input_message_text: Box<InputMessageContent>) -> &mut Self { self.input_message_text = Some(input_message_text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



