
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed. 
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatEventMemberPromoted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventMemberPromoted
  /// Chat member user identifier.
  user_id: Option<i32>,
  /// Previous status of the chat member.
  old_status: Option<Box<ChatMemberStatus>>,
  /// New status of the chat member.
  new_status: Option<Box<ChatMemberStatus>>,
  
}


impl Clone for ChatEventMemberPromoted {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for ChatEventMemberPromoted {}
impl RObject for ChatEventMemberPromoted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberPromoted" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventMemberPromoted }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventMemberPromoted {}


impl ChatEventMemberPromoted {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventMemberPromoted".to_string(),
      user_id: None,
      old_status: None,
      new_status: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn old_status(&self) -> Option<Box<ChatMemberStatus>> { self.old_status.clone() }
  #[doc(hidden)] pub fn _set_old_status(&mut self, old_status: Box<ChatMemberStatus>) -> &mut Self { self.old_status = Some(old_status); self }
  
  pub fn new_status(&self) -> Option<Box<ChatMemberStatus>> { self.new_status.clone() }
  #[doc(hidden)] pub fn _set_new_status(&mut self, new_status: Box<ChatMemberStatus>) -> &mut Self { self.new_status = Some(new_status); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



