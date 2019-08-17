
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A file defined by a local path. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputFileLocal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputFileLocal
  /// Local path to the file.
  path: Option<String>,
  
}



impl Object for InputFileLocal {}
impl RObject for InputFileLocal {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputFileLocal" }
  fn td_type(&self) -> RTDType { RTDType::InputFileLocal }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputFile for InputFileLocal {}


impl InputFileLocal {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputFileLocal".to_string(),
      path: None,
      
    }
  }
  
  pub fn path(&self) -> Option<String> { self.path.clone() }
  #[doc(hidden)] pub fn _set_path(&mut self, path: String) -> &mut Self { self.path = Some(path); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



