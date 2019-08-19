
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a file; this is an offline request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getFile
  /// Identifier of the file to get.
  file_id: Option<i32>,
  
}



impl Object for GetFile {}
impl RObject for GetFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFile" }
  fn td_type(&self) -> RTDType { RTDType::GetFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetFile {}


impl GetFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getFile".to_string(),
      file_id: None,
      
    }
  }
  
  pub fn file_id(&self) -> Option<i32> { self.file_id.clone() }
  #[doc(hidden)] pub fn _set_file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



