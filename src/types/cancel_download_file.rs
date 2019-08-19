
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Stops the downloading of a file. If a file has already been downloaded, does nothing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelDownloadFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // cancelDownloadFile
  /// Identifier of a file to stop downloading.
  file_id: Option<i32>,
  /// Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server.
  only_if_pending: Option<bool>,
  
}



impl Object for CancelDownloadFile {}
impl RObject for CancelDownloadFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "cancelDownloadFile" }
  fn td_type(&self) -> RTDType { RTDType::CancelDownloadFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CancelDownloadFile {}


impl CancelDownloadFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "cancelDownloadFile".to_string(),
      file_id: None,
      only_if_pending: None,
      
    }
  }
  
  pub fn file_id(&self) -> Option<i32> { self.file_id.clone() }
  #[doc(hidden)] pub fn _set_file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn only_if_pending(&self) -> Option<bool> { self.only_if_pending.clone() }
  #[doc(hidden)] pub fn _set_only_if_pending(&mut self, only_if_pending: bool) -> &mut Self { self.only_if_pending = Some(only_if_pending); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



