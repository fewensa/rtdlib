
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A file defined by its unique ID. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputFileId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputFileId
  /// Unique file identifier.
  id: Option<i32>,
  
}



impl Object for InputFileId {}
impl RObject for InputFileId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputFileId" }
  fn td_type(&self) -> RTDType { RTDType::InputFileId }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputFile for InputFileId {}


impl InputFileId {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputFileId".to_string(),
      id: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



