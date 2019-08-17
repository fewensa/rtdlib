
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The title of a chat was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatTitle
  /// Chat identifier.
  chat_id: Option<i64>,
  /// The new chat title.
  title: Option<String>,
  
}



impl Object for UpdateChatTitle {}
impl RObject for UpdateChatTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatTitle" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatTitle }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatTitle {}


impl UpdateChatTitle {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatTitle".to_string(),
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



