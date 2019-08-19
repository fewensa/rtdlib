
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some messages were deleted. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeleteMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateDeleteMessages
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Identifiers of the deleted messages.
  message_ids: Option<Vec<i64>>,
  /// True, if the messages are permanently deleted by a user (as opposed to just becoming inaccessible).
  is_permanent: Option<bool>,
  /// True, if the messages are deleted only from the cache and can possibly be retrieved again in the future.
  from_cache: Option<bool>,
  
}



impl Object for UpdateDeleteMessages {}
impl RObject for UpdateDeleteMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateDeleteMessages" }
  fn td_type(&self) -> RTDType { RTDType::UpdateDeleteMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateDeleteMessages {}


impl UpdateDeleteMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateDeleteMessages".to_string(),
      chat_id: None,
      message_ids: None,
      is_permanent: None,
      from_cache: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_ids(&self) -> Option<Vec<i64>> { self.message_ids.clone() }
  #[doc(hidden)] pub fn _set_message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn is_permanent(&self) -> Option<bool> { self.is_permanent.clone() }
  #[doc(hidden)] pub fn _set_is_permanent(&mut self, is_permanent: bool) -> &mut Self { self.is_permanent = Some(is_permanent); self }
  
  pub fn from_cache(&self) -> Option<bool> { self.from_cache.clone() }
  #[doc(hidden)] pub fn _set_from_cache(&mut self, from_cache: bool) -> &mut Self { self.from_cache = Some(from_cache); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



