
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanFileName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // cleanFileName
  /// File name or path to the file.
  file_name: Option<String>,
  
}



impl Object for CleanFileName {}
impl RObject for CleanFileName {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "cleanFileName" }
  fn td_type(&self) -> RTDType { RTDType::CleanFileName }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CleanFileName {}


impl CleanFileName {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "cleanFileName".to_string(),
      file_name: None,
      
    }
  }
  
  pub fn file_name(&self) -> Option<String> { self.file_name.clone() }
  #[doc(hidden)] pub fn _set_file_name(&mut self, file_name: String) -> &mut Self { self.file_name = Some(file_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



