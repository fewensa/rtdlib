
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A supergroup has been created from a basic group. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatUpgradeFrom {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatUpgradeFrom
  /// Title of the newly created supergroup.
  title: Option<String>,
  /// The identifier of the original basic group.
  basic_group_id: Option<i32>,
  
}



impl Object for MessageChatUpgradeFrom {}
impl RObject for MessageChatUpgradeFrom {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatUpgradeFrom" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatUpgradeFrom }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatUpgradeFrom {}


impl MessageChatUpgradeFrom {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatUpgradeFrom".to_string(),
      title: None,
      basic_group_id: None,
      
    }
  }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn basic_group_id(&self) -> Option<i32> { self.basic_group_id.clone() }
  #[doc(hidden)] pub fn _set_basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



