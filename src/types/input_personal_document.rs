
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A personal document to be saved to Telegram Passport. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputPersonalDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPersonalDocument
  /// List of files containing the pages of the document.
  files: Option<Vec<Box<InputFile>>>,
  /// List of files containing a certified English translation of the document.
  translation: Option<Vec<Box<InputFile>>>,
  
}


impl Clone for InputPersonalDocument {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputPersonalDocument {}
impl RObject for InputPersonalDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPersonalDocument" }
  fn td_type(&self) -> RTDType { RTDType::InputPersonalDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl InputPersonalDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPersonalDocument".to_string(),
      files: None,
      translation: None,
      
    }
  }
  
  pub fn files(&self) -> Option<Vec<Box<InputFile>>> { self.files.clone() }
  #[doc(hidden)] pub fn _set_files(&mut self, files: Vec<Box<InputFile>>) -> &mut Self { self.files = Some(files); self }
  
  pub fn translation(&self) -> Option<Vec<Box<InputFile>>> { self.translation.clone() }
  #[doc(hidden)] pub fn _set_translation(&mut self, translation: Vec<Box<InputFile>>) -> &mut Self { self.translation = Some(translation); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



