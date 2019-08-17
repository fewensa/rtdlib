
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Reports to the server whether a chat is a spam chat or not. Can be used only if ChatReportSpamState.can_report_spam is true. After this request, ChatReportSpamState.can_report_spam becomes false forever.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeChatReportSpamState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // changeChatReportSpamState
  /// Chat identifier.
  chat_id: Option<i64>,
  /// If true, the chat will be reported as spam; otherwise it will be marked as not spam.
  is_spam_chat: Option<bool>,
  
}



impl Object for ChangeChatReportSpamState {}
impl RObject for ChangeChatReportSpamState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "changeChatReportSpamState" }
  fn td_type(&self) -> RTDType { RTDType::ChangeChatReportSpamState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ChangeChatReportSpamState {}


impl ChangeChatReportSpamState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "changeChatReportSpamState".to_string(),
      chat_id: None,
      is_spam_chat: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn is_spam_chat(&self) -> Option<bool> { self.is_spam_chat.clone() }
  #[doc(hidden)] pub fn _set_is_spam_chat(&mut self, is_spam_chat: bool) -> &mut Self { self.is_spam_chat = Some(is_spam_chat); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



