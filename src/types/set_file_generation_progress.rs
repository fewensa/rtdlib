
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Informs TDLib on a file generation prograss.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFileGenerationProgress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setFileGenerationProgress
  /// The identifier of the generation process.
  generation_id: Option<i64>,
  /// Expected size of the generated file, in bytes; 0 if unknown.
  expected_size: Option<i32>,
  /// The number of bytes already generated.
  local_prefix_size: Option<i32>,
  
}



impl Object for SetFileGenerationProgress {}
impl RObject for SetFileGenerationProgress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setFileGenerationProgress" }
  fn td_type(&self) -> RTDType { RTDType::SetFileGenerationProgress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetFileGenerationProgress {}


impl SetFileGenerationProgress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setFileGenerationProgress".to_string(),
      generation_id: None,
      expected_size: None,
      local_prefix_size: None,
      
    }
  }
  
  pub fn generation_id(&self) -> Option<i64> { self.generation_id.clone() }
  #[doc(hidden)] pub fn _set_generation_id(&mut self, generation_id: i64) -> &mut Self { self.generation_id = Some(generation_id); self }
  
  pub fn expected_size(&self) -> Option<i32> { self.expected_size.clone() }
  #[doc(hidden)] pub fn _set_expected_size(&mut self, expected_size: i32) -> &mut Self { self.expected_size = Some(expected_size); self }
  
  pub fn local_prefix_size(&self) -> Option<i32> { self.local_prefix_size.clone() }
  #[doc(hidden)] pub fn _set_local_prefix_size(&mut self, local_prefix_size: i32) -> &mut Self { self.local_prefix_size = Some(local_prefix_size); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



