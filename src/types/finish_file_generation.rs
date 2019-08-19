
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Finishes the file generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishFileGeneration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // finishFileGeneration
  /// The identifier of the generation process.
  generation_id: Option<i64>,
  /// If set, means that file generation has failed and should be terminated.
  error: Option<Error>,
  
}



impl Object for FinishFileGeneration {}
impl RObject for FinishFileGeneration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "finishFileGeneration" }
  fn td_type(&self) -> RTDType { RTDType::FinishFileGeneration }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for FinishFileGeneration {}


impl FinishFileGeneration {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "finishFileGeneration".to_string(),
      generation_id: None,
      error: None,
      
    }
  }
  
  pub fn generation_id(&self) -> Option<i64> { self.generation_id.clone() }
  #[doc(hidden)] pub fn _set_generation_id(&mut self, generation_id: i64) -> &mut Self { self.generation_id = Some(generation_id); self }
  
  pub fn error(&self) -> Option<Error> { self.error.clone() }
  #[doc(hidden)] pub fn _set_error(&mut self, error: Error) -> &mut Self { self.error = Some(error); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



