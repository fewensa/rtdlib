
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user was banned (and hence is not a member of the chat). Implies the user can't return to the chat or view messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberStatusBanned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMemberStatusBanned
  /// Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever.
  banned_until_date: Option<i32>,
  
}



impl Object for ChatMemberStatusBanned {}
impl RObject for ChatMemberStatusBanned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusBanned" }
  fn td_type(&self) -> RTDType { RTDType::ChatMemberStatusBanned }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatMemberStatus for ChatMemberStatusBanned {}


impl ChatMemberStatusBanned {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMemberStatusBanned".to_string(),
      banned_until_date: None,
      
    }
  }
  
  pub fn banned_until_date(&self) -> Option<i32> { self.banned_until_date.clone() }
  #[doc(hidden)] pub fn _set_banned_until_date(&mut self, banned_until_date: i32) -> &mut Self { self.banned_until_date = Some(banned_until_date); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



