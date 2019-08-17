
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of chat members. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatMembers
  /// Approximate total count of chat members found.
  total_count: Option<i32>,
  /// A list of chat members.
  members: Option<Vec<ChatMember>>,
  
}



impl Object for ChatMembers {}
impl RObject for ChatMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembers" }
  fn td_type(&self) -> RTDType { RTDType::ChatMembers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatMembers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatMembers".to_string(),
      total_count: None,
      members: None,
      
    }
  }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn members(&self) -> Option<Vec<ChatMember>> { self.members.clone() }
  #[doc(hidden)] pub fn _set_members(&mut self, members: Vec<ChatMember>) -> &mut Self { self.members = Some(members); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



