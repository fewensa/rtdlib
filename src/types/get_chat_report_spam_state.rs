
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information on whether the current chat can be reported as spam.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatReportSpamState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatReportSpamState
  /// Chat identifier.
  chat_id: Option<i64>,
  
}



impl Object for GetChatReportSpamState {}
impl RObject for GetChatReportSpamState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatReportSpamState" }
  fn td_type(&self) -> RTDType { RTDType::GetChatReportSpamState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatReportSpamState {}


impl GetChatReportSpamState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatReportSpamState".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



