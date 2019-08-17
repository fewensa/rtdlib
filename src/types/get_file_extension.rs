
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileExtension {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getFileExtension
  /// The MIME type of the file.
  mime_type: Option<String>,
  
}



impl Object for GetFileExtension {}
impl RObject for GetFileExtension {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFileExtension" }
  fn td_type(&self) -> RTDType { RTDType::GetFileExtension }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetFileExtension {}


impl GetFileExtension {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getFileExtension".to_string(),
      mime_type: None,
      
    }
  }
  
  pub fn mime_type(&self) -> Option<String> { self.mime_type.clone() }
  #[doc(hidden)] pub fn _set_mime_type(&mut self, mime_type: String) -> &mut Self { self.mime_type = Some(mime_type); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



