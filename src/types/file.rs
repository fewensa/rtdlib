
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // file
  /// Unique file identifier.
  id: Option<i32>,
  /// File size; 0 if unknown.
  size: Option<i32>,
  /// Expected file size in case the exact file size is unknown, but an approximate size is known. Can be used to show download/upload progress.
  expected_size: Option<i32>,
  /// Information about the local copy of the file.
  local: Option<LocalFile>,
  /// Information about the remote copy of the file.
  remote: Option<RemoteFile>,
  
}



impl Object for File {}
impl RObject for File {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "file" }
  fn td_type(&self) -> RTDType { RTDType::File }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl File {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "file".to_string(),
      id: None,
      size: None,
      expected_size: None,
      local: None,
      remote: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn size(&self) -> Option<i32> { self.size.clone() }
  #[doc(hidden)] pub fn _set_size(&mut self, size: i32) -> &mut Self { self.size = Some(size); self }
  
  pub fn expected_size(&self) -> Option<i32> { self.expected_size.clone() }
  #[doc(hidden)] pub fn _set_expected_size(&mut self, expected_size: i32) -> &mut Self { self.expected_size = Some(expected_size); self }
  
  pub fn local(&self) -> Option<LocalFile> { self.local.clone() }
  #[doc(hidden)] pub fn _set_local(&mut self, local: LocalFile) -> &mut Self { self.local = Some(local); self }
  
  pub fn remote(&self) -> Option<RemoteFile> { self.remote.clone() }
  #[doc(hidden)] pub fn _set_remote(&mut self, remote: RemoteFile) -> &mut Self { self.remote = Some(remote); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



