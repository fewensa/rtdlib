
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A basic group was upgraded to a supergroup and was deactivated as the result. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatUpgradeTo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatUpgradeTo
  /// Identifier of the supergroup to which the basic group was upgraded.
  supergroup_id: Option<i32>,
  
}



impl Object for MessageChatUpgradeTo {}
impl RObject for MessageChatUpgradeTo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatUpgradeTo" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatUpgradeTo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatUpgradeTo {}


impl MessageChatUpgradeTo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatUpgradeTo".to_string(),
      supergroup_id: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



