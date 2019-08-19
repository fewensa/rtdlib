
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a part of a file. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // filePart
  /// File bytes.
  data: Option<String>,
  
}



impl Object for FilePart {}
impl RObject for FilePart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "filePart" }
  fn td_type(&self) -> RTDType { RTDType::FilePart }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl FilePart {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "filePart".to_string(),
      data: None,
      
    }
  }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



