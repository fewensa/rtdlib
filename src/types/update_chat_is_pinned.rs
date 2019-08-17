
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat was pinned or unpinned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatIsPinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatIsPinned
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New value of is_pinned.
  is_pinned: Option<bool>,
  /// New value of the chat order.
  order: Option<i64>,
  
}



impl Object for UpdateChatIsPinned {}
impl RObject for UpdateChatIsPinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatIsPinned" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatIsPinned }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatIsPinned {}


impl UpdateChatIsPinned {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatIsPinned".to_string(),
      chat_id: None,
      is_pinned: None,
      order: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn order(&self) -> Option<i64> { self.order.clone() }
  #[doc(hidden)] pub fn _set_order(&mut self, order: i64) -> &mut Self { self.order = Some(order); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



