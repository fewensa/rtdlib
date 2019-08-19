
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatOnlineMemberCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatOnlineMemberCount
  /// Identifier of the chat.
  chat_id: Option<i64>,
  /// New number of online members in the chat, or 0 if unknown.
  online_member_count: Option<i32>,
  
}



impl Object for UpdateChatOnlineMemberCount {}
impl RObject for UpdateChatOnlineMemberCount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatOnlineMemberCount" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatOnlineMemberCount }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatOnlineMemberCount {}


impl UpdateChatOnlineMemberCount {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatOnlineMemberCount".to_string(),
      chat_id: None,
      online_member_count: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn online_member_count(&self) -> Option<i32> { self.online_member_count.clone() }
  #[doc(hidden)] pub fn _set_online_member_count(&mut self, online_member_count: i32) -> &mut Self { self.online_member_count = Some(online_member_count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



