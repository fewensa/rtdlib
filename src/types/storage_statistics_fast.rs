
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains approximate storage usage statistics, excluding files of unknown file type. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStatisticsFast {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // storageStatisticsFast
  /// Approximate total size of files.
  files_size: Option<i64>,
  /// Approximate number of files.
  file_count: Option<i32>,
  /// Size of the database.
  database_size: Option<i64>,
  /// Size of the language pack database.
  language_pack_database_size: Option<i64>,
  /// Size of the TDLib internal log.
  log_size: Option<i64>,
  
}



impl Object for StorageStatisticsFast {}
impl RObject for StorageStatisticsFast {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "storageStatisticsFast" }
  fn td_type(&self) -> RTDType { RTDType::StorageStatisticsFast }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl StorageStatisticsFast {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "storageStatisticsFast".to_string(),
      files_size: None,
      file_count: None,
      database_size: None,
      language_pack_database_size: None,
      log_size: None,
      
    }
  }
  
  pub fn files_size(&self) -> Option<i64> { self.files_size.clone() }
  #[doc(hidden)] pub fn _set_files_size(&mut self, files_size: i64) -> &mut Self { self.files_size = Some(files_size); self }
  
  pub fn file_count(&self) -> Option<i32> { self.file_count.clone() }
  #[doc(hidden)] pub fn _set_file_count(&mut self, file_count: i32) -> &mut Self { self.file_count = Some(file_count); self }
  
  pub fn database_size(&self) -> Option<i64> { self.database_size.clone() }
  #[doc(hidden)] pub fn _set_database_size(&mut self, database_size: i64) -> &mut Self { self.database_size = Some(database_size); self }
  
  pub fn language_pack_database_size(&self) -> Option<i64> { self.language_pack_database_size.clone() }
  #[doc(hidden)] pub fn _set_language_pack_database_size(&mut self, language_pack_database_size: i64) -> &mut Self { self.language_pack_database_size = Some(language_pack_database_size); self }
  
  pub fn log_size(&self) -> Option<i64> { self.log_size.clone() }
  #[doc(hidden)] pub fn _set_log_size(&mut self, log_size: i64) -> &mut Self { self.log_size = Some(log_size); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



