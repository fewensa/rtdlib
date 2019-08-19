
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A personal document, containing some information about a user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // personalDocument
  /// List of files containing the pages of the document.
  files: Option<Vec<DatedFile>>,
  /// List of files containing a certified English translation of the document.
  translation: Option<Vec<DatedFile>>,
  
}



impl Object for PersonalDocument {}
impl RObject for PersonalDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "personalDocument" }
  fn td_type(&self) -> RTDType { RTDType::PersonalDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PersonalDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "personalDocument".to_string(),
      files: None,
      translation: None,
      
    }
  }
  
  pub fn files(&self) -> Option<Vec<DatedFile>> { self.files.clone() }
  #[doc(hidden)] pub fn _set_files(&mut self, files: Vec<DatedFile>) -> &mut Self { self.files = Some(files); self }
  
  pub fn translation(&self) -> Option<Vec<DatedFile>> { self.translation.clone() }
  #[doc(hidden)] pub fn _set_translation(&mut self, translation: Vec<DatedFile>) -> &mut Self { self.translation = Some(translation); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



