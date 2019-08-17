
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Asynchronously uploads a file to the cloud without sending it in a message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // uploadFile
  /// File to upload.
  file: Option<Box<InputFile>>,
  /// File type.
  file_type: Option<Box<FileType>>,
  /// Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which uploadFile was called will be uploaded first.
  priority: Option<i32>,
  
}


impl Clone for UploadFile {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for UploadFile {}
impl RObject for UploadFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "uploadFile" }
  fn td_type(&self) -> RTDType { RTDType::UploadFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for UploadFile {}


impl UploadFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "uploadFile".to_string(),
      file: None,
      file_type: None,
      priority: None,
      
    }
  }
  
  pub fn file(&self) -> Option<Box<InputFile>> { self.file.clone() }
  #[doc(hidden)] pub fn _set_file(&mut self, file: Box<InputFile>) -> &mut Self { self.file = Some(file); self }
  
  pub fn file_type(&self) -> Option<Box<FileType>> { self.file_type.clone() }
  #[doc(hidden)] pub fn _set_file_type(&mut self, file_type: Box<FileType>) -> &mut Self { self.file_type = Some(file_type); self }
  
  pub fn priority(&self) -> Option<i32> { self.priority.clone() }
  #[doc(hidden)] pub fn _set_priority(&mut self, priority: i32) -> &mut Self { self.priority = Some(priority); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



