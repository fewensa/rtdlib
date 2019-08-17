
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // openChat
  /// Chat identifier.
  chat_id: Option<i64>,
  
}



impl Object for OpenChat {}
impl RObject for OpenChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "openChat" }
  fn td_type(&self) -> RTDType { RTDType::OpenChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for OpenChat {}


impl OpenChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "openChat".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



