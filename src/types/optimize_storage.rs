
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted.
#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizeStorage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // optimizeStorage
  /// Limit on the total size of files after deletion. Pass -1 to use the default limit.
  size: Option<i64>,
  /// Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass -1 to use the default limit.
  ttl: Option<i32>,
  /// Limit on the total count of files after deletion. Pass -1 to use the default limit.
  count: Option<i32>,
  /// The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass -1 to use the default value.
  immunity_delay: Option<i32>,
  /// If not empty, only files with the given type(s) are considered. By default, all types except thumbnails, profile photos, stickers and wallpapers are deleted.
  file_types: Option<Vec<Box<FileType>>>,
  /// If not empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos).
  chat_ids: Option<Vec<i64>>,
  /// If not empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos).
  exclude_chat_ids: Option<Vec<i64>>,
  /// Same as in getStorageStatistics. Affects only returned statistics.
  chat_limit: Option<i32>,
  
}


impl Clone for OptimizeStorage {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for OptimizeStorage {}
impl RObject for OptimizeStorage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optimizeStorage" }
  fn td_type(&self) -> RTDType { RTDType::OptimizeStorage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for OptimizeStorage {}


impl OptimizeStorage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "optimizeStorage".to_string(),
      size: None,
      ttl: None,
      count: None,
      immunity_delay: None,
      file_types: None,
      chat_ids: None,
      exclude_chat_ids: None,
      chat_limit: None,
      
    }
  }
  
  pub fn size(&self) -> Option<i64> { self.size.clone() }
  #[doc(hidden)] pub fn _set_size(&mut self, size: i64) -> &mut Self { self.size = Some(size); self }
  
  pub fn ttl(&self) -> Option<i32> { self.ttl.clone() }
  #[doc(hidden)] pub fn _set_ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn count(&self) -> Option<i32> { self.count.clone() }
  #[doc(hidden)] pub fn _set_count(&mut self, count: i32) -> &mut Self { self.count = Some(count); self }
  
  pub fn immunity_delay(&self) -> Option<i32> { self.immunity_delay.clone() }
  #[doc(hidden)] pub fn _set_immunity_delay(&mut self, immunity_delay: i32) -> &mut Self { self.immunity_delay = Some(immunity_delay); self }
  
  pub fn file_types(&self) -> Option<Vec<Box<FileType>>> { self.file_types.clone() }
  #[doc(hidden)] pub fn _set_file_types(&mut self, file_types: Vec<Box<FileType>>) -> &mut Self { self.file_types = Some(file_types); self }
  
  pub fn chat_ids(&self) -> Option<Vec<i64>> { self.chat_ids.clone() }
  #[doc(hidden)] pub fn _set_chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self { self.chat_ids = Some(chat_ids); self }
  
  pub fn exclude_chat_ids(&self) -> Option<Vec<i64>> { self.exclude_chat_ids.clone() }
  #[doc(hidden)] pub fn _set_exclude_chat_ids(&mut self, exclude_chat_ids: Vec<i64>) -> &mut Self { self.exclude_chat_ids = Some(exclude_chat_ids); self }
  
  pub fn chat_limit(&self) -> Option<i32> { self.chat_limit.clone() }
  #[doc(hidden)] pub fn _set_chat_limit(&mut self, chat_limit: i32) -> &mut Self { self.chat_limit = Some(chat_limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



