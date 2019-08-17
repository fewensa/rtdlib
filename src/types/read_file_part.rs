
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct read from the file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadFilePart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // readFilePart
  /// Identifier of the file. The file must be located in the TDLib file cache.
  file_id: Option<i32>,
  /// The offset from which to read the file.
  offset: Option<i32>,
  /// Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position.
  count: Option<i32>,
  
}



impl Object for ReadFilePart {}
impl RObject for ReadFilePart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "readFilePart" }
  fn td_type(&self) -> RTDType { RTDType::ReadFilePart }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ReadFilePart {}


impl ReadFilePart {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "readFilePart".to_string(),
      file_id: None,
      offset: None,
      count: None,
      
    }
  }
  
  pub fn file_id(&self) -> Option<i32> { self.file_id.clone() }
  #[doc(hidden)] pub fn _set_file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn count(&self) -> Option<i32> { self.count.clone() }
  #[doc(hidden)] pub fn _set_count(&mut self, count: i32) -> &mut Self { self.count = Some(count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



