
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an ordered list of chats. Chats are sorted by the pair (order, chat_id) in decreasing order. (For example, to get a list of chats from the beginning, the offset_order should be equal to a biggest signed 64-bit number 9223372036854775807 == 2^63 - 1). For optimal performance the number of returned chats is chosen by the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChats
  /// Chat order to return chats from.
  offset_order: Option<i64>,
  /// Chat identifier to return chats from.
  offset_chat_id: Option<i64>,
  /// The maximum number of chats to be returned. It is possible that fewer chats than the limit are returned even if the end of the list is not reached.
  limit: Option<i32>,
  
}



impl Object for GetChats {}
impl RObject for GetChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChats" }
  fn td_type(&self) -> RTDType { RTDType::GetChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChats {}


impl GetChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChats".to_string(),
      offset_order: None,
      offset_chat_id: None,
      limit: None,
      
    }
  }
  
  pub fn offset_order(&self) -> Option<i64> { self.offset_order.clone() }
  #[doc(hidden)] pub fn _set_offset_order(&mut self, offset_order: i64) -> &mut Self { self.offset_order = Some(offset_order); self }
  
  pub fn offset_chat_id(&self) -> Option<i64> { self.offset_chat_id.clone() }
  #[doc(hidden)] pub fn _set_offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self { self.offset_chat_id = Some(offset_chat_id); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



