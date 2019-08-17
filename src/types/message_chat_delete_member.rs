
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat member was deleted. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatDeleteMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatDeleteMember
  /// User identifier of the deleted chat member.
  user_id: Option<i32>,
  
}



impl Object for MessageChatDeleteMember {}
impl RObject for MessageChatDeleteMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatDeleteMember" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatDeleteMember }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatDeleteMember {}


impl MessageChatDeleteMember {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatDeleteMember".to_string(),
      user_id: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



