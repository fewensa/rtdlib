
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Creates a new supergroup from an existing basic group and sends a corresponding 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeBasicGroupChatToSupergroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // upgradeBasicGroupChatToSupergroupChat
  /// Identifier of the chat to upgrade.
  chat_id: Option<i64>,
  
}



impl Object for UpgradeBasicGroupChatToSupergroupChat {}
impl RObject for UpgradeBasicGroupChatToSupergroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "upgradeBasicGroupChatToSupergroupChat" }
  fn td_type(&self) -> RTDType { RTDType::UpgradeBasicGroupChatToSupergroupChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for UpgradeBasicGroupChatToSupergroupChat {}


impl UpgradeBasicGroupChatToSupergroupChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "upgradeBasicGroupChatToSupergroupChat".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



