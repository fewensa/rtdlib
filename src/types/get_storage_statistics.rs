
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns storage usage statistics. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStorageStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getStorageStatistics
  /// Maximum number of chats with the largest storage usage for which separate statistics should be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0.
  chat_limit: Option<i32>,
  
}



impl Object for GetStorageStatistics {}
impl RObject for GetStorageStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getStorageStatistics" }
  fn td_type(&self) -> RTDType { RTDType::GetStorageStatistics }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetStorageStatistics {}


impl GetStorageStatistics {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getStorageStatistics".to_string(),
      chat_limit: None,
      
    }
  }
  
  pub fn chat_limit(&self) -> Option<i32> { self.chat_limit.clone() }
  #[doc(hidden)] pub fn _set_chat_limit(&mut self, chat_limit: i32) -> &mut Self { self.chat_limit = Some(chat_limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



