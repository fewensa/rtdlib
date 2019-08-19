
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes current user from chat members. Private and secret chats can't be left using this method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // leaveChat
  /// Chat identifier.
  chat_id: Option<i64>,
  
}



impl Object for LeaveChat {}
impl RObject for LeaveChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "leaveChat" }
  fn td_type(&self) -> RTDType { RTDType::LeaveChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for LeaveChat {}


impl LeaveChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "leaveChat".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



