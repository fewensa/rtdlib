
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // closeChat
  /// Chat identifier.
  chat_id: Option<i64>,
  
}



impl Object for CloseChat {}
impl RObject for CloseChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "closeChat" }
  fn td_type(&self) -> RTDType { RTDType::CloseChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CloseChat {}


impl CloseChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "closeChat".to_string(),
      chat_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



