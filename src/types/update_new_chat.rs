
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewChat
  /// The chat.
  chat: Option<Chat>,
  
}



impl Object for UpdateNewChat {}
impl RObject for UpdateNewChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewChat" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewChat {}


impl UpdateNewChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewChat".to_string(),
      chat: None,
      
    }
  }
  
  pub fn chat(&self) -> Option<Chat> { self.chat.clone() }
  #[doc(hidden)] pub fn _set_chat(&mut self, chat: Chat) -> &mut Self { self.chat = Some(chat); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



