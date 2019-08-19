
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Writes a part of a generated file. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteGeneratedFilePart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // writeGeneratedFilePart
  /// The identifier of the generation process.
  generation_id: Option<i64>,
  /// The offset from which to write the data to the file.
  offset: Option<i32>,
  /// The data to write.
  data: Option<String>,
  
}



impl Object for WriteGeneratedFilePart {}
impl RObject for WriteGeneratedFilePart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "writeGeneratedFilePart" }
  fn td_type(&self) -> RTDType { RTDType::WriteGeneratedFilePart }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for WriteGeneratedFilePart {}


impl WriteGeneratedFilePart {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "writeGeneratedFilePart".to_string(),
      generation_id: None,
      offset: None,
      data: None,
      
    }
  }
  
  pub fn generation_id(&self) -> Option<i64> { self.generation_id.clone() }
  #[doc(hidden)] pub fn _set_generation_id(&mut self, generation_id: i64) -> &mut Self { self.generation_id = Some(generation_id); self }
  
  pub fn offset(&self) -> Option<i32> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



