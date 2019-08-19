
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for adding new members to the chat; instead, use 
#[derive(Debug, Serialize, Deserialize)]
pub struct SetChatMemberStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setChatMemberStatus
  /// Chat identifier.
  chat_id: Option<i64>,
  /// User identifier.
  user_id: Option<i32>,
  /// The new status of the member in the chat.
  status: Option<Box<ChatMemberStatus>>,
  
}


impl Clone for SetChatMemberStatus {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetChatMemberStatus {}
impl RObject for SetChatMemberStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatMemberStatus" }
  fn td_type(&self) -> RTDType { RTDType::SetChatMemberStatus }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetChatMemberStatus {}


impl SetChatMemberStatus {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setChatMemberStatus".to_string(),
      chat_id: None,
      user_id: None,
      status: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn status(&self) -> Option<Box<ChatMemberStatus>> { self.status.clone() }
  #[doc(hidden)] pub fn _set_status(&mut self, status: Box<ChatMemberStatus>) -> &mut Self { self.status = Some(status); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



