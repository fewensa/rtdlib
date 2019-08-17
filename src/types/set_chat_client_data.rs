
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes client data associated with a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChatClientData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setChatClientData
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New value of client_data.
  client_data: Option<String>,
  
}



impl Object for SetChatClientData {}
impl RObject for SetChatClientData {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setChatClientData" }
  fn td_type(&self) -> RTDType { RTDType::SetChatClientData }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetChatClientData {}


impl SetChatClientData {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setChatClientData".to_string(),
      chat_id: None,
      client_data: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn client_data(&self) -> Option<String> { self.client_data.clone() }
  #[doc(hidden)] pub fn _set_client_data(&mut self, client_data: String) -> &mut Self { self.client_data = Some(client_data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



