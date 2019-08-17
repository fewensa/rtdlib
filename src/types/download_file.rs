
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Downloads a file from the cloud. Download progress and completion of the download will be notified through 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // downloadFile
  /// Identifier of the file to download.
  file_id: Option<i32>,
  /// Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile was called will be downloaded first.
  priority: Option<i32>,
  /// The starting position from which the file should be downloaded.
  offset: Option<i32>,
  /// Number of bytes which should be downloaded starting from the "offset" position before the download will be automatically cancelled; use 0 to download without a limit.
  limit: Option<i32>,
  /// If false, this request returns file state just after the download has been started. If true, this request returns file state only after the download has succeeded, has failed, has been cancelled or a new downloadFile request with different offset/limit parameters was sent.
  synchronous: Option<bool>,
  
}



impl Object for DownloadFile {}
impl RObject for DownloadFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "downloadFile" }
  fn td_type(&self) -> RTDType { RTDType::DownloadFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DownloadFile {}


impl DownloadFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "downloadFile".to_string(),
      file_id: None,
      priority: None,
      offset: None,
      limit: None,
      synchronous: None,
      
    }
  }
  
  pub fn file_id(&self) -> Option<i32> { self.file_id.clone() }
  #[doc(hidden)] pub fn _set_file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn priority(&self) -> Option<i32> { self.priority.clone() }
  #[doc(hidden)] pub fn _set_priority(&mut self, priority: i32) -> &mut Self { self.priority = Some(priority); self }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn synchronous(&self) -> Option<bool> { self.synchronous.clone() }
  #[doc(hidden)] pub fn _set_synchronous(&mut self, synchronous: bool) -> &mut Self { self.synchronous = Some(synchronous); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



