
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Stops the uploading of a file. Supported only for files uploaded by using 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelUploadFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // cancelUploadFile
  /// Identifier of the file to stop uploading.
  file_id: Option<i32>,
  
}



impl Object for CancelUploadFile {}
impl RObject for CancelUploadFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "cancelUploadFile" }
  fn td_type(&self) -> RTDType { RTDType::CancelUploadFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CancelUploadFile {}


impl CancelUploadFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "cancelUploadFile".to_string(),
      file_id: None,
      
    }
  }
  
  pub fn file_id(&self) -> Option<i32> { self.file_id.clone() }
  #[doc(hidden)] pub fn _set_file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



