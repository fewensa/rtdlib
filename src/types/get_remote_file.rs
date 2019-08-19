
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetRemoteFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getRemoteFile
  /// Remote identifier of the file to get.
  remote_file_id: Option<String>,
  /// File type, if known.
  file_type: Option<Box<FileType>>,
  
}


impl Clone for GetRemoteFile {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for GetRemoteFile {}
impl RObject for GetRemoteFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRemoteFile" }
  fn td_type(&self) -> RTDType { RTDType::GetRemoteFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetRemoteFile {}


impl GetRemoteFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getRemoteFile".to_string(),
      remote_file_id: None,
      file_type: None,
      
    }
  }
  
  pub fn remote_file_id(&self) -> Option<String> { self.remote_file_id.clone() }
  #[doc(hidden)] pub fn _set_remote_file_id(&mut self, remote_file_id: String) -> &mut Self { self.remote_file_id = Some(remote_file_id); self }
  
  pub fn file_type(&self) -> Option<Box<FileType>> { self.file_type.clone() }
  #[doc(hidden)] pub fn _set_file_type(&mut self, file_type: Box<FileType>) -> &mut Self { self.file_type = Some(file_type); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



