
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the storage usage statistics for a specific file type. 
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStatisticsByFileType {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // storageStatisticsByFileType
  /// File type.
  file_type: Option<Box<FileType>>,
  /// Total size of the files.
  size: Option<i64>,
  /// Total number of files.
  count: Option<i32>,
  
}


impl Clone for StorageStatisticsByFileType {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for StorageStatisticsByFileType {}
impl RObject for StorageStatisticsByFileType {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "storageStatisticsByFileType" }
  fn td_type(&self) -> RTDType { RTDType::StorageStatisticsByFileType }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl StorageStatisticsByFileType {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "storageStatisticsByFileType".to_string(),
      file_type: None,
      size: None,
      count: None,
      
    }
  }
  
  pub fn file_type(&self) -> Option<Box<FileType>> { self.file_type.clone() }
  #[doc(hidden)] pub fn _set_file_type(&mut self, file_type: Box<FileType>) -> &mut Self { self.file_type = Some(file_type); self }
  
  pub fn size(&self) -> Option<i64> { self.size.clone() }
  #[doc(hidden)] pub fn _set_size(&mut self, size: i64) -> &mut Self { self.size = Some(size); self }
  
  pub fn count(&self) -> Option<i32> { self.count.clone() }
  #[doc(hidden)] pub fn _set_count(&mut self, count: i32) -> &mut Self { self.count = Some(count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



