
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Information about a file was updated. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateFile
  /// New data about the file.
  file: Option<File>,
  
}



impl Object for UpdateFile {}
impl RObject for UpdateFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateFile" }
  fn td_type(&self) -> RTDType { RTDType::UpdateFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateFile {}


impl UpdateFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateFile".to_string(),
      file: None,
      
    }
  }
  
  pub fn file(&self) -> Option<File> { self.file.clone() }
  #[doc(hidden)] pub fn _set_file(&mut self, file: File) -> &mut Self { self.file = Some(file); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



