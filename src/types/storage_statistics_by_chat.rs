
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the storage usage statistics for a specific chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStatisticsByChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // storageStatisticsByChat
  /// Chat identifier; 0 if none.
  chat_id: Option<i64>,
  /// Total size of the files in the chat.
  size: Option<i64>,
  /// Total number of files in the chat.
  count: Option<i32>,
  /// Statistics split by file types.
  by_file_type: Option<Vec<StorageStatisticsByFileType>>,
  
}



impl Object for StorageStatisticsByChat {}
impl RObject for StorageStatisticsByChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "storageStatisticsByChat" }
  fn td_type(&self) -> RTDType { RTDType::StorageStatisticsByChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl StorageStatisticsByChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "storageStatisticsByChat".to_string(),
      chat_id: None,
      size: None,
      count: None,
      by_file_type: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn size(&self) -> Option<i64> { self.size.clone() }
  #[doc(hidden)] pub fn _set_size(&mut self, size: i64) -> &mut Self { self.size = Some(size); self }
  
  pub fn count(&self) -> Option<i32> { self.count.clone() }
  #[doc(hidden)] pub fn _set_count(&mut self, count: i32) -> &mut Self { self.count = Some(count); self }
  
  pub fn by_file_type(&self) -> Option<Vec<StorageStatisticsByFileType>> { self.by_file_type.clone() }
  #[doc(hidden)] pub fn _set_by_file_type(&mut self, by_file_type: Vec<StorageStatisticsByFileType>) -> &mut Self { self.by_file_type = Some(by_file_type); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



