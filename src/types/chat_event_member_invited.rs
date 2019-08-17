
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new chat member was invited. 
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatEventMemberInvited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventMemberInvited
  /// New member user identifier.
  user_id: Option<i32>,
  /// New member status.
  status: Option<Box<ChatMemberStatus>>,
  
}


impl Clone for ChatEventMemberInvited {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for ChatEventMemberInvited {}
impl RObject for ChatEventMemberInvited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberInvited" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventMemberInvited }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventMemberInvited {}


impl ChatEventMemberInvited {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventMemberInvited".to_string(),
      user_id: None,
      status: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn status(&self) -> Option<Box<ChatMemberStatus>> { self.status.clone() }
  #[doc(hidden)] pub fn _set_status(&mut self, status: Box<ChatMemberStatus>) -> &mut Self { self.status = Some(status); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



