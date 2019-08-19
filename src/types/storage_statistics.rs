
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the exact storage usage statistics split by chats and file type. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // storageStatistics
  /// Total size of files.
  size: Option<i64>,
  /// Total number of files.
  count: Option<i32>,
  /// Statistics split by chats.
  by_chat: Option<Vec<StorageStatisticsByChat>>,
  
}



impl Object for StorageStatistics {}
impl RObject for StorageStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "storageStatistics" }
  fn td_type(&self) -> RTDType { RTDType::StorageStatistics }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl StorageStatistics {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "storageStatistics".to_string(),
      size: None,
      count: None,
      by_chat: None,
      
    }
  }
  
  pub fn size(&self) -> Option<i64> { self.size.clone() }
  #[doc(hidden)] pub fn _set_size(&mut self, size: i64) -> &mut Self { self.size = Some(size); self }
  
  pub fn count(&self) -> Option<i32> { self.count.clone() }
  #[doc(hidden)] pub fn _set_count(&mut self, count: i32) -> &mut Self { self.count = Some(count); self }
  
  pub fn by_chat(&self) -> Option<Vec<StorageStatisticsByChat>> { self.by_chat.clone() }
  #[doc(hidden)] pub fn _set_by_chat(&mut self, by_chat: Vec<StorageStatisticsByChat>) -> &mut Self { self.by_chat = Some(by_chat); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



