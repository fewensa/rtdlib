
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes a file from the TDLib file cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteFile
  /// Identifier of the file to delete.
  file_id: Option<i32>,
  
}



impl Object for DeleteFile {}
impl RObject for DeleteFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteFile" }
  fn td_type(&self) -> RTDType { RTDType::DeleteFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteFile {}


impl DeleteFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteFile".to_string(),
      file_id: None,
      
    }
  }
  
  pub fn file_id(&self) -> Option<i32> { self.file_id.clone() }
  #[doc(hidden)] pub fn _set_file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



