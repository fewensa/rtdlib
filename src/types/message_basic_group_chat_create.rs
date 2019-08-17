
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A newly created basic group. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBasicGroupChatCreate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageBasicGroupChatCreate
  /// Title of the basic group.
  title: Option<String>,
  /// User identifiers of members in the basic group.
  member_user_ids: Option<Vec<i32>>,
  
}



impl Object for MessageBasicGroupChatCreate {}
impl RObject for MessageBasicGroupChatCreate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageBasicGroupChatCreate" }
  fn td_type(&self) -> RTDType { RTDType::MessageBasicGroupChatCreate }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageBasicGroupChatCreate {}


impl MessageBasicGroupChatCreate {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageBasicGroupChatCreate".to_string(),
      title: None,
      member_user_ids: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn member_user_ids(&self) -> Option<Vec<i32>> { self.member_user_ids.clone() }
  #[doc(hidden)] pub fn _set_member_user_ids(&mut self, member_user_ids: Vec<i32>) -> &mut Self { self.member_user_ids = Some(member_user_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



