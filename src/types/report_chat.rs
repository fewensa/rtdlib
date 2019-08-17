
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Reports a chat to the Telegram moderators. Supported only for supergroups, channels, or private chats with bots, since other chats can't be checked by moderators.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // reportChat
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The reason for reporting the chat.
  reason: Option<Box<ChatReportReason>>,
  /// Identifiers of reported messages, if any.
  message_ids: Option<Vec<i64>>,
  
}


impl Clone for ReportChat {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for ReportChat {}
impl RObject for ReportChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "reportChat" }
  fn td_type(&self) -> RTDType { RTDType::ReportChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ReportChat {}


impl ReportChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "reportChat".to_string(),
      chat_id: None,
      reason: None,
      message_ids: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn reason(&self) -> Option<Box<ChatReportReason>> { self.reason.clone() }
  #[doc(hidden)] pub fn _set_reason(&mut self, reason: Box<ChatReportReason>) -> &mut Self { self.reason = Some(reason); self }
  
  pub fn message_ids(&self) -> Option<Vec<i64>> { self.message_ids.clone() }
  #[doc(hidden)] pub fn _set_message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



