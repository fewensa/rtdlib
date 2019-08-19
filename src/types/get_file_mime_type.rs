
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileMimeType {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getFileMimeType
  /// The name of the file or path to the file.
  file_name: Option<String>,
  
}



impl Object for GetFileMimeType {}
impl RObject for GetFileMimeType {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFileMimeType" }
  fn td_type(&self) -> RTDType { RTDType::GetFileMimeType }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetFileMimeType {}


impl GetFileMimeType {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getFileMimeType".to_string(),
      file_name: None,
      
    }
  }
  
  pub fn file_name(&self) -> Option<String> { self.file_name.clone() }
  #[doc(hidden)] pub fn _set_file_name(&mut self, file_name: String) -> &mut Self { self.file_name = Some(file_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



