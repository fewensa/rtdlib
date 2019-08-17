
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns file downloaded prefix size from a given offset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileDownloadedPrefixSize {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getFileDownloadedPrefixSize
  /// Identifier of the file.
  file_id: Option<i32>,
  /// Offset from which downloaded prefix size should be calculated.
  offset: Option<i32>,
  
}



impl Object for GetFileDownloadedPrefixSize {}
impl RObject for GetFileDownloadedPrefixSize {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFileDownloadedPrefixSize" }
  fn td_type(&self) -> RTDType { RTDType::GetFileDownloadedPrefixSize }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetFileDownloadedPrefixSize {}


impl GetFileDownloadedPrefixSize {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getFileDownloadedPrefixSize".to_string(),
      file_id: None,
      offset: None,
      
    }
  }
  
  pub fn file_id(&self) -> Option<i32> { self.file_id.clone() }
  #[doc(hidden)] pub fn _set_file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



