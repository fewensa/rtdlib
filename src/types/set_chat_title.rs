
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the chat title. Supported only for basic groups, supergroups and channels. Requires administrator rights in basic groups and the appropriate administrator rights in supergroups and channels. The title will not be changed until the request to the server has been completed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setChatTitle
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New title of the chat; 1-128 characters.
  title: Option<String>,
  
}



impl Object for SetChatTitle {}
impl RObject for SetChatTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatTitle" }
  fn td_type(&self) -> RTDType { RTDType::SetChatTitle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetChatTitle {}


impl SetChatTitle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setChatTitle".to_string(),
      chat_id: None,
      title: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



