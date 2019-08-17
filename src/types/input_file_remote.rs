
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A file defined by its remote ID. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputFileRemote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputFileRemote
  /// Remote file identifier.
  id: Option<String>,
  
}



impl Object for InputFileRemote {}
impl RObject for InputFileRemote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputFileRemote" }
  fn td_type(&self) -> RTDType { RTDType::InputFileRemote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputFile for InputFileRemote {}


impl InputFileRemote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputFileRemote".to_string(),
      id: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



