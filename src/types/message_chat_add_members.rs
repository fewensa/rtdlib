
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// New chat members were added. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatAddMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatAddMembers
  /// User identifiers of the new members.
  member_user_ids: Option<Vec<i32>>,
  
}



impl Object for MessageChatAddMembers {}
impl RObject for MessageChatAddMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatAddMembers" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatAddMembers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatAddMembers {}


impl MessageChatAddMembers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatAddMembers".to_string(),
      member_user_ids: None,
      
    }
  }
  
  pub fn member_user_ids(&self) -> Option<Vec<i32>> { self.member_user_ids.clone() }
  #[doc(hidden)] pub fn _set_member_user_ids(&mut self, member_user_ids: Vec<i32>) -> &mut Self { self.member_user_ids = Some(member_user_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



